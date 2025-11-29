// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../interfaces/IPaymentEscrow.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/// @title PaymentEscrow
/// @notice Trustless escrow for crypto-to-fiat payments
/// @dev Multi-chain compatible (Polygon, Base, Ethereum)
contract PaymentEscrow is IPaymentEscrow, ReentrancyGuard, Pausable, AccessControl {
    bytes32 public constant OPERATOR_ROLE = keccak256("OPERATOR_ROLE");
    bytes32 public constant RATE_ORACLE_ROLE = keccak256("RATE_ORACLE_ROLE");

    // Supported stablecoins
    mapping(address => bool) public supportedTokens;

    // Payment ID => Payment
    mapping(bytes32 => Payment) public payments;

    // Fee configuration
    uint256 public platformFeePercent = 50; // 0.5% (basis points: 50/10000)
    address public feeCollector;

    // Rate oracle (updated by authorized oracle)
    mapping(string => uint256) public exchangeRates; // Currency => Rate (6 decimals)
    mapping(string => uint256) public rateLastUpdated;

    // Security: Rate deviation limits
    uint256 public constant MAX_RATE_DEVIATION = 500; // 5% max change

    // Timeouts
    uint256 public constant PAYMENT_DEADLINE = 2 hours;
    uint256 public constant DISPUTE_PERIOD = 7 days;

    constructor(address _feeCollector) {
        require(_feeCollector != address(0), "Fee collector is zero address");
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(OPERATOR_ROLE, msg.sender);
        _grantRole(RATE_ORACLE_ROLE, msg.sender);
        feeCollector = _feeCollector;
    }

    /// @notice User creates payment and locks stablecoin
    function createPayment(
        address token,
        uint256 amount,
        string calldata fiatCurrency,
        bytes32 recipientHash
    ) external nonReentrant whenNotPaused returns (bytes32 paymentId) {
        require(supportedTokens[token], "Token not supported");
        require(amount > 0, "Amount must be > 0");
        require(exchangeRates[fiatCurrency] > 0, "Currency not supported");

        // Transfer stablecoin from user to escrow
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        // Calculate fiat amount based on current rate
        uint256 rate = exchangeRates[fiatCurrency];

        // Deduct platform fee
        uint256 fee = (amount * platformFeePercent) / 10000;
        uint256 netAmount = amount - fee;
        uint256 netFiatAmount = (netAmount * rate) / 1e6;

        // Generate unique payment ID
        paymentId = keccak256(
            abi.encodePacked(
                msg.sender,
                token,
                amount,
                recipientHash,
                block.timestamp
            )
        );

        // Store payment
        payments[paymentId] = Payment({
            sender: msg.sender,
            token: token,
            amount: netAmount,
            fiatCurrency: fiatCurrency,
            fiatAmount: netFiatAmount,
            exchangeRate: rate,
            recipientHash: recipientHash,
            createdAt: block.timestamp,
            deadline: block.timestamp + PAYMENT_DEADLINE,
            status: PaymentStatus.Pending,
            bankReference: "",
            operator: address(0)
        });

        // Transfer fee to collector
        if (fee > 0) {
            IERC20(token).transfer(feeCollector, fee);
        }

        emit PaymentCreated(
            paymentId,
            msg.sender,
            token,
            netAmount,
            fiatCurrency,
            netFiatAmount
        );

        return paymentId;
    }

    /// @notice Operator marks payment as processing after initiating bank transfer
    function markProcessing(
        bytes32 paymentId,
        string calldata bankReference
    ) external onlyRole(OPERATOR_ROLE) {
        Payment storage payment = payments[paymentId];
        require(payment.status == PaymentStatus.Pending, "Invalid status");
        require(block.timestamp <= payment.deadline, "Payment expired");

        payment.status = PaymentStatus.Processing;
        payment.bankReference = bankReference;
        payment.operator = msg.sender;

        emit PaymentProcessing(paymentId, msg.sender, bankReference);
    }

    /// @notice Complete payment after bank transfer confirmed
    function completePayment(
        bytes32 paymentId,
        bytes32 proofHash
    ) external onlyRole(OPERATOR_ROLE) nonReentrant {
        Payment storage payment = payments[paymentId];
        require(payment.status == PaymentStatus.Processing, "Must be processing");
        require(payment.operator == msg.sender, "Not assigned operator");

        // Transfer stablecoin to operator wallet
        IERC20(payment.token).transfer(payment.operator, payment.amount);

        payment.status = PaymentStatus.Completed;

        emit PaymentCompleted(paymentId, proofHash);
    }

    /// @notice Refund user if payment fails
    function refundPayment(
        bytes32 paymentId,
        string calldata reason
    ) external onlyRole(OPERATOR_ROLE) nonReentrant {
        Payment storage payment = payments[paymentId];
        require(
            payment.status == PaymentStatus.Pending ||
            payment.status == PaymentStatus.Processing,
            "Cannot refund"
        );

        // Return stablecoin to user
        IERC20(payment.token).transfer(payment.sender, payment.amount);

        payment.status = PaymentStatus.Refunded;

        emit PaymentRefunded(paymentId, reason);
    }

    /// @notice User can claim refund if deadline passed
    function claimRefund(bytes32 paymentId) external nonReentrant {
        Payment storage payment = payments[paymentId];
        require(payment.sender == msg.sender, "Not payment sender");
        require(payment.status == PaymentStatus.Pending, "Not pending");
        require(block.timestamp > payment.deadline, "Deadline not passed");

        // Automatic refund after deadline
        IERC20(payment.token).transfer(payment.sender, payment.amount);

        payment.status = PaymentStatus.Refunded;

        emit PaymentRefunded(paymentId, "Deadline expired");
    }

    /// @notice Sender raises a dispute for a payment in process
    function disputePayment(bytes32 paymentId) external {
        Payment storage payment = payments[paymentId];
        require(payment.sender == msg.sender, "Not payment sender");
        require(payment.status == PaymentStatus.Processing, "Not processing");
        // Optional: Add a time window for disputes after processing starts
        // require(block.timestamp < payment.deadline + DISPUTE_PERIOD, "Dispute period over");

        payment.status = PaymentStatus.Disputed;
        emit PaymentDisputed(paymentId, msg.sender);
    }

    /// @notice Admin resolves a dispute
    function resolveDispute(
        bytes32 paymentId,
        bool refundToSender
    ) external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant {
        Payment storage payment = payments[paymentId];
        require(payment.status == PaymentStatus.Disputed, "Not in dispute");

        if (refundToSender) {
            // Refund to the original sender
            IERC20(payment.token).transfer(payment.sender, payment.amount);
            payment.status = PaymentStatus.Refunded;
            emit DisputeResolved(paymentId, msg.sender, payment.sender);
            emit PaymentRefunded(paymentId, "Dispute resolved to sender");
        } else {
            // Complete payment to the assigned operator
            require(payment.operator != address(0), "Operator not assigned");
            IERC20(payment.token).transfer(payment.operator, payment.amount);
            payment.status = PaymentStatus.Completed;
            emit DisputeResolved(paymentId, msg.sender, payment.operator);
        }
    }

    /// @notice Update exchange rate (called by oracle)
    function updateExchangeRate(
        string calldata currency,
        uint256 newRate
    ) external onlyRole(RATE_ORACLE_ROLE) {
        require(newRate > 0, "Rate must be > 0");

        uint256 oldRate = exchangeRates[currency];

        // Prevent sudden rate changes (security)
        if (oldRate > 0) {
            uint256 deviation = oldRate > newRate
                ? ((oldRate - newRate) * 10000) / oldRate
                : ((newRate - oldRate) * 10000) / oldRate;
            
            require(
                deviation <= MAX_RATE_DEVIATION,
                "Rate change too large"
            );
        }

        exchangeRates[currency] = newRate;
        rateLastUpdated[currency] = block.timestamp;

        emit ExchangeRateUpdated(currency, newRate);
    }

    /// @notice Add supported token
    function addSupportedToken(address token) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(token != address(0), "Token is zero address");
        supportedTokens[token] = true;
        emit SupportedTokenAdded(token);
    }

    /// @notice Remove supported token
    function removeSupportedToken(address token) external onlyRole(DEFAULT_ADMIN_ROLE) {
        supportedTokens[token] = false;
        emit SupportedTokenRemoved(token);
    }

    /// @notice Update platform fee
    function setPlatformFee(uint256 newFee) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(newFee <= 200, "Fee too high (max 2%)"); // 200 basis points = 2%
        platformFeePercent = newFee;
        emit PlatformFeeUpdated(newFee);
    }

    /// @notice Emergency pause
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Emergency unpause
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    /// @notice Get payment details
    function getPayment(bytes32 paymentId) external view returns (Payment memory) {
        return payments[paymentId];
    }
}
