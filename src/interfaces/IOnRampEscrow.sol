// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IOnRampEscrow
/// @notice Interface for the trustless escrow for fiat-to-crypto (on-ramping) payments.
interface IOnRampEscrow {
    // --- Enums and Structs ---

    enum OrderStatus {
        Pending,
        FundsLocked,
        FiatSent,
        Completed,
        Cancelled,
        Disputed
    }

    struct OnRampOrder {
        address buyer;
        address lp;
        address token;
        uint256 tokenAmount;
        string fiatCurrency;
        uint256 fiatAmount;
        OrderStatus status;
        uint256 createdAt;
        uint256 fundsLockedAt;
        bytes32 userPaymentProof;
    }

    // --- Events ---

    event OrderCreated(
        bytes32 indexed orderId,
        address indexed buyer,
        address token,
        uint256 tokenAmount,
        uint256 fiatAmount,
        string fiatCurrency
    );
    event FundsLocked(bytes32 indexed orderId, address indexed lp, uint256 amount);
    event FiatSent(bytes32 indexed orderId, bytes32 proofHash);
    event OrderCompleted(bytes32 indexed orderId, address indexed buyer);
    event OrderCancelled(bytes32 indexed orderId, string reason);
    event OrderDisputed(bytes32 indexed orderId, address indexed reporter);
    event DisputeResolved(
        bytes32 indexed orderId,
        address indexed resolver,
        address winner
    );

    // --- Functions ---

    function createOnRampOrder(
        address token,
        uint256 tokenAmount,
        uint256 fiatAmount,
        string calldata fiatCurrency
    ) external returns (bytes32 orderId);

    function lockFunds(bytes32 orderId) external;

    function confirmFiatSent(bytes32 orderId, bytes32 proofHash) external;

    function releaseFunds(bytes32 orderId) external;

    function reclaimLockedFunds(bytes32 orderId) external;

    function cancelOrder(bytes32 orderId) external;

    function disputeOrder(bytes32 orderId) external;

    function resolveDispute(bytes32 orderId, bool releaseToBuyer) external;

    function setLockDeadline(uint256 newDeadline) external;

    function setPaymentDeadline(uint256 newDeadline) external;

    function pause() external;

    function unpause() external;

    function getOrder(bytes32 orderId) external view returns (OnRampOrder memory);
}
