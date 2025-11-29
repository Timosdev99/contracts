// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IRateOracle
/// @notice Interface for the RateOracle contract.
interface IRateOracle {
    // --- Structs ---

    struct Rate {
        uint256 value;        // Rate with 6 decimals
        uint256 timestamp;
        uint256 confidence;   // 0-100, higher = more reliable
    }

    // --- Events ---

    event RateUpdated(
        string indexed currency,
        address indexed source,
        uint256 rate,
        uint256 confidence
    );

    // --- Functions ---

    function updateRate(
        string calldata currency,
        uint256 rate,
        uint256 confidence
    ) external;

    function getAggregatedRate(
        string calldata currency
    ) external view returns (uint256 rate, uint256 totalConfidence);

    function setPriceFeed(string calldata currency, address priceFeedAddress) external;
}
