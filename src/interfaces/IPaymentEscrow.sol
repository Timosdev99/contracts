// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IPaymentEscrow
/// @notice Interface for the decentralized escrow for crypto-to-fiat payments.
interface IPaymentEscrow {
    // --- Enums and Structs ---

    enum PaymentStatus {
        Pending,
        Processing,
        Completed,
        Refunded
    }

    struct Payment {
        address sender;
        address token;
        uint256 amount;
        string fiatCurrency;
        uint256 fiatAmount;
        uint256 exchangeRate;
        bytes32 recipientHash;
        uint256 createdAt;
        uint256 deadline;
        PaymentStatus status;
        string bankReference;
        address operator; // The assigned LP
    }

    // --- Events ---

    event PaymentCreated(
        bytes32 indexed paymentId,
        address indexed sender,
        address token,
        uint256 amount,
        string fiatCurrency,
        uint256 fiatAmount
    );

    event PaymentProcessing(
        bytes32 indexed paymentId,
        address indexed operator,
        string bankReference
    );

    event PaymentCompleted(bytes32 indexed paymentId, bytes32 proofHash);

    event PaymentRefunded(bytes32 indexed paymentId, string reason);

    event SupportedTokenAdded(address indexed token);

    event SupportedTokenRemoved(address indexed token);

    event PlatformFeeUpdated(uint256 newFeePercent);

    // --- Functions ---

    function createPayment(
        address token,
        uint256 amount,
        uint256 fiatAmount,
        string calldata fiatCurrency,
        bytes32 recipientHash
    ) external returns (bytes32 paymentId);

    function claimPayment(bytes32 paymentId, bytes calldata permissionSlip) external;

    function confirmSettlement(bytes32 paymentId) external;

    function claimRefund(bytes32 paymentId) external;

    function addSupportedToken(address token) external;

    function removeSupportedToken(address token) external;

    function setPlatformFee(uint256 newFee) external;

    function setPermissionSlipSigner(address _newSigner) external;

    function setOracleWallet(address _newOracle) external;

    function pause() external;

    function unpause() external;

    function getPayment(bytes32 paymentId) external view returns (Payment memory);
}
