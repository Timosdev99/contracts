// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title ILPRegistry
/// @notice Interface for the LPRegistry contract.
interface ILPRegistry {
    // --- Structs ---
    struct LPInfo {
        bool isRegistered;
        bool isActive;
        uint256 stakedAmount;
        uint256 lastStakeChange;
    }

    // --- Functions ---

    function isLPActive(address lpAddress) external view returns (bool);

    function getLPInfo(address lpAddress) external view returns (LPInfo memory);

    function slash(address lpAddress, uint256 penaltyAmount) external;
}
