// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../interfaces/IOnRampEscrow.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/// @title OnRampEscrow
/// @notice Trustless escrow for fiat-to-crypto (on-ramping) payments.
/// @dev The user is the buyer, the operator is the Liquidity Provider (LP).
contract OnRampEscrow is IOnRampEscrow, ReentrancyGuard, Pausable, AccessControl {
    bytes32 public constant LP_ROLE = keccak256("LP_ROLE");
    bytes32 public constant RELAYER_ROLE = keccak256("RELAYER_ROLE");

    // Order ID => Order
    mapping(bytes32 => OnRampOrder) public orders;

    // Timeouts
    uint256 public lockDeadline = 10 minutes; // Time for LP to lock funds
    uint256 public paymentDeadline = 30 minutes; // Time for user to pay after funds are locked

    // Optional per-LP caps by token (0 = unlimited)
    mapping(address => mapping(address => uint256)) public lpCapByToken;
    mapping(address => mapping(address => uint256)) public lpOutstandingByToken;

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(LP_ROLE, msg.sender); // Grant to deployer initially
        _grantRole(RELAYER_ROLE, msg.sender); // Grant to deployer initially
    }

    /// @notice User creates an order to buy crypto
    function createOnRampOrder(address token, uint256 tokenAmount, uint256 fiatAmount, string calldata fiatCurrency)
        external
        whenNotPaused
        returns (bytes32 orderId)
    {
        require(tokenAmount > 0, "Token amount must be > 0");

        orderId = keccak256(abi.encodePacked(msg.sender, token, tokenAmount, block.timestamp));

        orders[orderId] = OnRampOrder({
            buyer: msg.sender,
            lp: address(0),
            token: token,
            tokenAmount: tokenAmount,
            fiatCurrency: fiatCurrency,
            fiatAmount: fiatAmount,
            status: OrderStatus.Pending,
            createdAt: block.timestamp,
            fundsLockedAt: 0,
            userPaymentProof: bytes32(0)
        });

        emit OrderCreated(orderId, msg.sender, token, tokenAmount, fiatAmount, fiatCurrency);
        return orderId;
    }

    /// @notice LP locks crypto funds in escrow for a pending order
    function lockFunds(bytes32 orderId) external nonReentrant onlyRole(LP_ROLE) {
        _lockFunds(orderId, msg.sender);
    }

    /// @notice A relayer (backend) locks crypto funds in escrow on behalf of an LP.
    /// @dev LPs grant a one-time allowance to the relayer, which implies ongoing trust in relayer operation.
    /// @param orderId The ID of the order for which funds are being locked.
    /// @param lpAddress The address of the LP whose funds are being locked.
    function lockFundsByRelayer(bytes32 orderId, address lpAddress) external nonReentrant onlyRole(RELAYER_ROLE) {
        _lockFunds(orderId, lpAddress);
    }

    /// @dev Internal helper function to handle the actual logic of locking funds.
    function _lockFunds(bytes32 orderId, address _lpAddress) internal {
        OnRampOrder storage order = orders[orderId];
        require(order.status == OrderStatus.Pending, "Order not pending");
        require(block.timestamp <= order.createdAt + lockDeadline, "Lock deadline passed");
        require(hasRole(LP_ROLE, _lpAddress), "LP_ROLE_REQUIRED"); // Ensure the specified address is an LP
        _applyLpCap(_lpAddress, order.token, order.tokenAmount);

        order.lp = _lpAddress;
        order.status = OrderStatus.FundsLocked;
        order.fundsLockedAt = block.timestamp;

        require(IERC20(order.token).transferFrom(_lpAddress, address(this), order.tokenAmount), "LOCK_TRANSFER_FAILED");

        _increaseOutstanding(_lpAddress, order.token, order.tokenAmount);
        emit FundsLocked(orderId, _lpAddress, order.tokenAmount);
    }

    /// @notice User confirms they have sent the fiat payment off-chain
    function confirmFiatSent(bytes32 orderId, bytes32 proofHash) external {
        OnRampOrder storage order = orders[orderId];
        require(order.buyer == msg.sender, "Not the buyer");
        require(order.status == OrderStatus.FundsLocked, "Funds not locked");
        require(block.timestamp <= order.fundsLockedAt + paymentDeadline, "Payment deadline passed");

        order.status = OrderStatus.FiatSent;
        order.userPaymentProof = proofHash;

        emit FiatSent(orderId, proofHash);
    }

    /// @notice LP confirms fiat receipt and releases crypto to the buyer
    function releaseFunds(bytes32 orderId) external nonReentrant {
        OnRampOrder storage order = orders[orderId];
        require(order.lp == msg.sender, "Not the assigned LP");
        require(order.status == OrderStatus.FiatSent, "Fiat not marked as sent");

        order.status = OrderStatus.Completed;
        require(IERC20(order.token).transfer(order.buyer, order.tokenAmount), "RELEASE_TRANSFER_FAILED");
        _decreaseOutstanding(order.lp, order.token, order.tokenAmount);

        emit OrderCompleted(orderId, order.buyer);
    }

    /// @notice LP can reclaim their locked funds if the user fails to pay in time
    function reclaimLockedFunds(bytes32 orderId) external nonReentrant {
        OnRampOrder storage order = orders[orderId];
        require(order.lp == msg.sender, "Not the assigned LP");
        require(order.status == OrderStatus.FundsLocked, "Order status invalid");
        require(block.timestamp > order.fundsLockedAt + paymentDeadline, "Payment deadline not passed");

        order.status = OrderStatus.Cancelled;
        require(IERC20(order.token).transfer(order.lp, order.tokenAmount), "RECLAIM_TRANSFER_FAILED");
        _decreaseOutstanding(order.lp, order.token, order.tokenAmount);

        emit OrderCancelled(orderId, "User payment timed out");
    }

    /// @notice Buyer can cancel if LP fails to lock funds in time
    function cancelOrder(bytes32 orderId) external {
        OnRampOrder storage order = orders[orderId];
        require(order.buyer == msg.sender, "Not the buyer");
        require(order.status == OrderStatus.Pending, "Order not pending");
        require(block.timestamp > order.createdAt + lockDeadline, "Lock deadline not passed");

        order.status = OrderStatus.Cancelled;
        emit OrderCancelled(orderId, "LP failed to lock funds");
    }

    // --- Dispute Functions ---

    /// @notice Raise a dispute (callable by buyer or LP)
    function disputeOrder(bytes32 orderId) external {
        OnRampOrder storage order = orders[orderId];
        require(msg.sender == order.buyer || msg.sender == order.lp, "Not a party to the order");
        require(order.status == OrderStatus.FiatSent, "Can only dispute after fiat is marked sent");

        order.status = OrderStatus.Disputed;
        emit OrderDisputed(orderId, msg.sender);
    }

    /// @notice Admin resolves a dispute
    function resolveDispute(bytes32 orderId, bool releaseToBuyer) external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant {
        OnRampOrder storage order = orders[orderId];
        require(order.status == OrderStatus.Disputed, "Order not in dispute");

        if (releaseToBuyer) {
            order.status = OrderStatus.Completed;
            require(IERC20(order.token).transfer(order.buyer, order.tokenAmount), "DISPUTE_BUYER_TRANSFER_FAILED");
            _decreaseOutstanding(order.lp, order.token, order.tokenAmount);
            emit DisputeResolved(orderId, msg.sender, order.buyer);
        } else {
            order.status = OrderStatus.Cancelled;
            require(IERC20(order.token).transfer(order.lp, order.tokenAmount), "DISPUTE_LP_TRANSFER_FAILED");
            _decreaseOutstanding(order.lp, order.token, order.tokenAmount);
            emit DisputeResolved(orderId, msg.sender, order.lp);
        }
    }

    // --- Admin Functions ---

    function setLockDeadline(uint256 newDeadline) external onlyRole(DEFAULT_ADMIN_ROLE) {
        lockDeadline = newDeadline;
    }

    function setPaymentDeadline(uint256 newDeadline) external onlyRole(DEFAULT_ADMIN_ROLE) {
        paymentDeadline = newDeadline;
    }

    function setLpCap(address lp, address token, uint256 cap) external onlyRole(DEFAULT_ADMIN_ROLE) {
        lpCapByToken[lp][token] = cap;
        emit LpCapSet(lp, token, cap);
    }

    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    /// @notice Get an order's details
    function getOrder(bytes32 orderId) external view returns (OnRampOrder memory) {
        return orders[orderId];
    }

    function _applyLpCap(address lp, address token, uint256 amount) internal view {
        uint256 cap = lpCapByToken[lp][token];
        if (cap == 0) {
            return;
        }
        require(lpOutstandingByToken[lp][token] + amount <= cap, "LP_CAP_EXCEEDED");
    }

    function _increaseOutstanding(address lp, address token, uint256 amount) internal {
        uint256 newOutstanding = lpOutstandingByToken[lp][token] + amount;
        lpOutstandingByToken[lp][token] = newOutstanding;
        emit LpOutstandingUpdated(lp, token, newOutstanding);
    }

    function _decreaseOutstanding(address lp, address token, uint256 amount) internal {
        uint256 newOutstanding = lpOutstandingByToken[lp][token] - amount;
        lpOutstandingByToken[lp][token] = newOutstanding;
        emit LpOutstandingUpdated(lp, token, newOutstanding);
    }
}
