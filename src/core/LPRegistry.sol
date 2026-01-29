// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/// @title LPRegistry
/// @notice Manages Liquidity Provider (LP) registration, staking, and slashing.
contract LPRegistry is AccessControl, ReentrancyGuard, Pausable {
    bytes32 public constant LP_ROLE = keccak256("LP_ROLE");
    bytes32 public constant SLASHER_ROLE = keccak256("SLASHER_ROLE");

    // --- Structs ---

    struct LPInfo {
        bool isRegistered;
        bool isActive;          // Can be set to false if stake is too low or admin pauses
        uint256 stakedAmount;
        uint256 lastStakeChange; // Timestamp of last stake/unstake
    }

    // --- State Variables ---

    IERC20 public stakingToken;
    uint256 public minStakeAmount;
    uint256 public slashingPenaltyPercent; // e.g., 500 for 5% (basis points)
    address public treasuryAddress; // Address where slashed funds are sent

    mapping(address => LPInfo) public lpData;

    // --- Events ---

    event LPRegistered(address indexed lpAddress, uint256 initialStake);
    event LPStaked(address indexed lpAddress, uint256 amount, uint256 newTotalStake);
    event LPUnstaked(address indexed lpAddress, uint256 amount, uint256 newTotalStake);
    event LPSlashed(address indexed lpAddress, uint256 penaltyAmount, uint256 newTotalStake);
    event LPStatusChanged(address indexed lpAddress, bool newStatus);
    event MinStakeAmountUpdated(uint256 newMinStakeAmount);
    event SlashingPenaltyPercentUpdated(uint256 newSlashingPenaltyPercent);

    // --- Constructor ---

    constructor(address _stakingToken, uint256 _minStakeAmount, uint256 _slashingPenaltyPercent, address _treasuryAddress) {
        require(_stakingToken != address(0), "Invalid staking token address");
        require(_minStakeAmount > 0, "Min stake must be greater than 0");
        require(_slashingPenaltyPercent <= 10000, "Slashing percent too high"); // Max 100%
        require(_treasuryAddress != address(0), "Invalid treasury address");

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(SLASHER_ROLE, msg.sender); // Grant to deployer initially

        stakingToken = IERC20(_stakingToken);
        minStakeAmount = _minStakeAmount;
        slashingPenaltyPercent = _slashingPenaltyPercent;
        treasuryAddress = _treasuryAddress;
    }

    // --- LP Management Functions ---

    /// @notice Allows an address to register as an LP by staking the minimum amount.
    /// @param amount The amount of staking token to stake.
    function registerLP(uint256 amount) external nonReentrant whenNotPaused {
        require(!lpData[msg.sender].isRegistered, "LP already registered");
        require(amount >= minStakeAmount, "Initial stake below minimum");

        _stake(msg.sender, amount);

        lpData[msg.sender].isRegistered = true;
        lpData[msg.sender].isActive = true;
        _grantRole(LP_ROLE, msg.sender);

        emit LPRegistered(msg.sender, amount);
    }

    /// @notice Allows an LP to increase their stake.
    /// @param amount The amount of staking token to add to the stake.
    function stake(uint256 amount) external nonReentrant whenNotPaused {
        require(lpData[msg.sender].isRegistered, "LP not registered");
        require(amount > 0, "Stake amount must be greater than 0");

        _stake(msg.sender, amount);
    }

    /// @notice Allows an LP to withdraw part of their stake.
    /// @param amount The amount of staking token to withdraw.
    function unstake(uint256 amount) external nonReentrant whenNotPaused {
        require(lpData[msg.sender].isRegistered, "LP not registered");
        require(amount > 0, "Unstake amount must be greater than 0");
        require(lpData[msg.sender].stakedAmount - amount >= minStakeAmount, "Remaining stake below minimum");

        lpData[msg.sender].stakedAmount -= amount;
        lpData[msg.sender].lastStakeChange = block.timestamp;
        require(stakingToken.transfer(msg.sender, amount), "UNSTAKE_TRANSFER_FAILED");

        emit LPUnstaked(msg.sender, amount, lpData[msg.sender].stakedAmount);
    }

    /// @notice Slashes an LP's stake as a penalty for misbehavior.
    /// @param lpAddress The address of the LP to slash.
    /// @param penaltyAmount The amount to slash.
    function slash(address lpAddress, uint256 penaltyAmount) external onlyRole(SLASHER_ROLE) nonReentrant {
        require(lpData[lpAddress].isRegistered, "LP not registered");
        require(penaltyAmount > 0, "Penalty amount must be greater than 0");
        require(lpData[lpAddress].stakedAmount >= penaltyAmount, "Penalty exceeds stake");

        lpData[lpAddress].stakedAmount -= penaltyAmount;
        lpData[lpAddress].lastStakeChange = block.timestamp;
        require(stakingToken.transfer(treasuryAddress, penaltyAmount), "SLASH_TRANSFER_FAILED"); // Transfer to treasury

        // If stake falls below minimum, mark as inactive
        if (lpData[lpAddress].stakedAmount < minStakeAmount) {
            lpData[lpAddress].isActive = false;
            emit LPStatusChanged(lpAddress, false);
        }

        emit LPSlashed(lpAddress, penaltyAmount, lpData[lpAddress].stakedAmount);
    }

    // --- Admin Functions ---

    /// @notice Sets the minimum required stake amount for LPs.
    /// @param _minStakeAmount The new minimum stake amount.
    function setMinStakeAmount(uint256 _minStakeAmount) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_minStakeAmount > 0, "Min stake must be greater than 0");
        minStakeAmount = _minStakeAmount;
        emit MinStakeAmountUpdated(_minStakeAmount);
    }

    /// @notice Sets the percentage of stake to be slashed as a penalty.
    /// @param _slashingPenaltyPercent The new slashing penalty percentage (in basis points).
    function setSlashingPenaltyPercent(uint256 _slashingPenaltyPercent) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_slashingPenaltyPercent <= 10000, "Slashing percent too high");
        slashingPenaltyPercent = _slashingPenaltyPercent;
        emit SlashingPenaltyPercentUpdated(_slashingPenaltyPercent);
    }

    /// @notice Allows admin to change an LP's active status.
    function setLPActiveStatus(address lpAddress, bool status) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(lpData[lpAddress].isRegistered, "LP not registered");
        lpData[lpAddress].isActive = status;
        emit LPStatusChanged(lpAddress, status);
    }

    /// @notice Emergency pause
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Emergency unpause
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    // --- View Functions ---

    /// @notice Checks if an address is a registered and active LP with sufficient stake.
    function isLPActive(address lpAddress) public view returns (bool) {
        LPInfo memory info = lpData[lpAddress];
        return info.isRegistered && info.isActive && info.stakedAmount >= minStakeAmount;
    }

    /// @notice Get an LP's information
    function getLPInfo(address lpAddress) external view returns (LPInfo memory) {
        return lpData[lpAddress];
    }

    // --- Internal Helper Functions ---

    function _stake(address lpAddress, uint256 amount) internal {
        require(stakingToken.transferFrom(lpAddress, address(this), amount), "STAKE_TRANSFER_FAILED");
        lpData[lpAddress].stakedAmount += amount;
        lpData[lpAddress].lastStakeChange = block.timestamp;
        emit LPStaked(lpAddress, amount, lpData[lpAddress].stakedAmount);
    }
}
