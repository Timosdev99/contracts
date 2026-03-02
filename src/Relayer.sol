// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./interfaces/IRelayer.sol";
import "./interfaces/IOnRampEscrow.sol";

/// @title Relayer
/// @notice A contract to facilitate the locking of funds in OnRampEscrow on behalf of LPs.
/// @dev LPs grant a one-time allowance to this relayer, which implies ongoing trust in relayer operation.
/// @dev This contract is intended to be controlled by an off-chain backend.
contract Relayer is IRelayer {
    IOnRampEscrow public onRampEscrow;
    address public owner;

    constructor(address _onRampEscrow) {
        require(_onRampEscrow != address(0), "ZERO_ADDRESS");
        onRampEscrow = IOnRampEscrow(_onRampEscrow);
        owner = msg.sender;
    }

    /// @notice Allows the backend (via this relayer) to lock LP funds in OnRampEscrow.
    /// @dev The LP must have pre-approved this Relayer contract to spend their tokens.
    /// @param orderId The ID of the on-ramp order.
    /// @param lpAddress The address of the LP whose funds are to be locked.
    function lockFundsForLP(bytes32 orderId, address lpAddress) external {
        require(msg.sender == owner, "Unauthorized");
        // This relayer contract must be granted RELAYER_ROLE in OnRampEscrow
        // The lpAddress must have granted allowance to this Relayer contract
        onRampEscrow.lockFundsByRelayer(orderId, lpAddress);
    }
}
