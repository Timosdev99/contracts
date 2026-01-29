// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../interfaces/IRateOracle.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@chainlink/contracts/src/v0.8/shared/interfaces/AggregatorV3Interface.sol";

/// @title RateOracle
/// @notice Aggregates exchange rates from multiple sources
contract RateOracle is IRateOracle, AccessControl {
    bytes32 public constant ORACLE_UPDATER_ROLE = keccak256("ORACLE_UPDATER_ROLE");
    
    // Currency => Source => Rate
    mapping(string => mapping(address => Rate)) public rates;
    
    // Approved rate sources
    mapping(address => bool) public approvedSources;

    // Chainlink price feeds
    mapping(string => AggregatorV3Interface) public priceFeeds;
    mapping(string => uint8) public priceFeedDecimals;
    uint256 public maxPriceAge; // Maximum age of a price feed in seconds

    constructor(uint256 _maxPriceAge) {
        require(_maxPriceAge > 0, "Max price age must be greater than 0");
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ORACLE_UPDATER_ROLE, msg.sender);
        maxPriceAge = _maxPriceAge;
    }

    /// @notice Set the maximum age of a price feed
    function setMaxPriceAge(uint256 _maxPriceAge) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_maxPriceAge > 0, "Max price age must be greater than 0");
        maxPriceAge = _maxPriceAge;
    }

    /// @notice Set the Chainlink price feed address for a given currency pair
    function setPriceFeed(string calldata currency, address priceFeedAddress, uint8 decimals)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        require(priceFeedAddress != address(0), "Invalid price feed address");
        priceFeeds[currency] = AggregatorV3Interface(priceFeedAddress);
        priceFeedDecimals[currency] = decimals;
    }
    
    /// @notice Update rate from a source
    function updateRate(
        string calldata currency,
        uint256 rate,
        uint256 confidence
    ) external onlyRole(ORACLE_UPDATER_ROLE) {
        require(confidence <= 100, "Invalid confidence");
        
        rates[currency][msg.sender] = Rate({
            value: rate,
            timestamp: block.timestamp,
            confidence: confidence
        });
        
        emit RateUpdated(currency, msg.sender, rate, confidence);
    }
    
    /// @notice Get aggregated rate from Chainlink (simplified for now)
    function getAggregatedRate(
        string calldata currency
    ) external view returns (uint256 rate, uint256 totalConfidence) {
        AggregatorV3Interface priceFeed = priceFeeds[currency];
        require(address(priceFeed) != address(0), "Price feed not set for currency");

        (, int256 answer, , uint256 updatedAt, ) = priceFeed.latestRoundData();
        require(answer > 0, "Invalid price from feed");
        require(block.timestamp - updatedAt <= maxPriceAge, "Price data is stale");

        uint8 decimals = priceFeedDecimals[currency];
        uint256 systemDecimals = 6;

        if (decimals > systemDecimals) {
            /// forge-lint: disable-next-line unsafe-typecast
            rate = uint256(answer) / (10 ** (decimals - systemDecimals));
        } else {
            /// forge-lint: disable-next-line unsafe-typecast
            rate = uint256(answer) * (10 ** (systemDecimals - decimals));
        }
        
        totalConfidence = 100; // Assume 100% confidence from Chainlink for simplicity

        return (rate, totalConfidence);
    }
}
