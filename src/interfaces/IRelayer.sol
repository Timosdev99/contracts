// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./IOnRampEscrow.sol";

/// @title IRelayer
/// @notice Interface for the relayer that locks LP funds in OnRampEscrow.
interface IRelayer {
    function onRampEscrow() external view returns (IOnRampEscrow);

    function owner() external view returns (address);

    function lockFundsForLP(bytes32 orderId, address lpAddress) external;
}
