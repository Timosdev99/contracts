// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../interfaces/IPaymentEscrow.sol";
import "../interfaces/ILPRegistry.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/// @title PaymentEscrow
/// @notice A decentralized escrow for crypto-to-fiat payments.
/// @dev This contract implements the "Permission Slip" model for LP prioritization
/// and relies on a multi-sig oracle for final settlement.
contract PaymentEscrow is IPaymentEscrow, ReentrancyGuard, Pausable, AccessControl {
    using ECDSA for bytes32;

    // --- State Variables ---

    ILPRegistry public lpRegistry;
    address public oracleWallet; // The multi-sig wallet of the Oracle Consortium
    address public permissionSlipSigner; // The off-chain service that issues permission slips

    mapping(address => bool) public supportedTokens;
    mapping(bytes32 => Payment) public payments;

    uint256 public platformFeePercent = 50; // 0.5% in basis points (50/10000)
    address public feeCollector;

    uint256 public constant PAYMENT_DEADLINE = 2 hours;

    // --- Constructor ---

    constructor(
        address _feeCollector,
        address _lpRegistryAddress,
        address _oracleWallet,
        address _permissionSlipSigner
    ) {
        require(_feeCollector != address(0), "ZERO_ADDRESS");
        require(_lpRegistryAddress != address(0), "ZERO_ADDRESS");
        require(_oracleWallet != address(0), "ZERO_ADDRESS");
        require(_permissionSlipSigner != address(0), "ZERO_ADDRESS");

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        feeCollector = _feeCollector;
        lpRegistry = ILPRegistry(_lpRegistryAddress);
        oracleWallet = _oracleWallet;
        permissionSlipSigner = _permissionSlipSigner;
    }

    // --- Core Payment Flow ---

    /// @notice User creates a payment to initiate an off-ramp, locking their stablecoin.
    /// @param token The address of the stablecoin being sold.
    /// @param amount The amount of stablecoin to sell.
    /// @param fiatAmount The target amount of fiat currency to be received.
    /// @param fiatCurrency The symbol of the fiat currency (e.g., "NGN").
    /// @param recipientHash A hash of the recipient's off-chain details.
    function createPayment(
        address token,
        uint256 amount,
        uint256 fiatAmount,
        string calldata fiatCurrency,
        bytes32 recipientHash
    ) external nonReentrant whenNotPaused returns (bytes32 paymentId) {
        require(supportedTokens[token], "TOKEN_NOT_SUPPORTED");
        require(amount > 0, "AMOUNT_IS_ZERO");

        // Transfer stablecoin from user to escrow
        require(IERC20(token).transferFrom(msg.sender, address(this), amount), "TRANSFER_FAILED");

        // Deduct platform fee
        uint256 fee = (amount * platformFeePercent) / 10000;
        uint256 netAmount = amount - fee;

        paymentId = keccak256(abi.encodePacked(msg.sender, token, amount, recipientHash, block.timestamp));

        payments[paymentId] = Payment({
            sender: msg.sender,
            token: token,
            amount: netAmount,
            fiatCurrency: fiatCurrency,
            fiatAmount: fiatAmount,
            exchangeRate: 0, // rate is now implicit in fiatAmount
            recipientHash: recipientHash,
            createdAt: block.timestamp,
            deadline: block.timestamp + PAYMENT_DEADLINE,
            status: PaymentStatus.Pending,
            bankReference: "",
            operator: address(0) // The LP who will process the payment
        });

        if (fee > 0) {
            require(IERC20(token).transfer(feeCollector, fee), "FEE_TRANSFER_FAILED");
        }

        emit PaymentCreated(paymentId, msg.sender, token, netAmount, fiatCurrency, fiatAmount);
        return paymentId;
    }

    /// @notice An active LP claims a pending payment using a valid permission slip.
    /// @param paymentId The ID of the payment to claim.
    /// @param permissionSlip A signature from the permissionSlipSigner, authorizing this action.
    function claimPayment(bytes32 paymentId, bytes calldata permissionSlip) external whenNotPaused {
        Payment storage payment = payments[paymentId];
        require(payment.status == PaymentStatus.Pending, "NOT_PENDING");
        require(lpRegistry.isLPActive(msg.sender), "NOT_ACTIVE_LP");

        // Verify the permission slip was signed by the trusted signer for this specific payment and LP
        bytes32 messageHash = keccak256(abi.encodePacked(paymentId, msg.sender));
        address signer = messageHash.recover(permissionSlip);
        require(signer == permissionSlipSigner, "INVALID_SLIP");

        payment.status = PaymentStatus.Processing;
        payment.operator = msg.sender; // Assign the LP as the operator

        emit PaymentProcessing(paymentId, msg.sender, ""); // Bank reference can be updated later if needed
    }

    /// @notice Called by the Oracle (Multi-Sig Wallet) to confirm settlement.
    /// @dev This function releases the escrowed funds to the LP after the oracle has
    /// verified the fiat transfer was successfully completed off-chain.
    /// @param paymentId The ID of the payment to settle.
    function confirmSettlement(bytes32 paymentId) external nonReentrant {
        require(msg.sender == oracleWallet, "UNAUTHORIZED_ORACLE");

        Payment storage payment = payments[paymentId];
        require(payment.status == PaymentStatus.Processing, "NOT_PROCESSING");
        require(payment.operator != address(0), "OPERATOR_NOT_ASSIGNED");

        // Transfer the escrowed stablecoin to the LP who processed the payment
        require(IERC20(payment.token).transfer(payment.operator, payment.amount), "TRANSFER_FAILED");

        payment.status = PaymentStatus.Completed;

        emit PaymentCompleted(paymentId, bytes32(0)); // proofHash is no longer needed here
    }

    /// @notice Allows the original sender to claim a refund if the payment is not
    /// processed before the deadline.
    function claimRefund(bytes32 paymentId) external nonReentrant {
        Payment storage payment = payments[paymentId];
        require(payment.sender == msg.sender, "NOT_SENDER");
        require(payment.status == PaymentStatus.Pending, "NOT_PENDING"); // Only if never claimed
        require(block.timestamp > payment.deadline, "DEADLINE_NOT_PASSED");

        // Return the net amount to the user
        require(IERC20(payment.token).transfer(payment.sender, payment.amount), "REFUND_TRANSFER_FAILED");

        payment.status = PaymentStatus.Refunded;

        emit PaymentRefunded(paymentId, "Deadline expired");
    }

    // --- Admin Functions ---

    function addSupportedToken(address token) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(token != address(0), "ZERO_ADDRESS");
        supportedTokens[token] = true;
        emit SupportedTokenAdded(token);
    }

    function removeSupportedToken(address token) external onlyRole(DEFAULT_ADMIN_ROLE) {
        supportedTokens[token] = false;
        emit SupportedTokenRemoved(token);
    }

    function setPlatformFee(uint256 newFee) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(newFee <= 200, "FEE_TOO_HIGH"); // Max 2%
        platformFeePercent = newFee;
        emit PlatformFeeUpdated(newFee);
    }

    function setPermissionSlipSigner(address _newSigner) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_newSigner != address(0), "ZERO_ADDRESS");
        permissionSlipSigner = _newSigner;
    }

    function setOracleWallet(address _newOracle) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_newOracle != address(0), "ZERO_ADDRESS");
        oracleWallet = _newOracle;
    }

    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    // --- View Functions ---

    function getPayment(bytes32 paymentId) external view returns (Payment memory) {
        return payments[paymentId];
    }
}
