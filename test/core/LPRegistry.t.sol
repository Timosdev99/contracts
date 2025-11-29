// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "src/core/LPRegistry.sol";
import "test/core/PaymentEscrow.t.sol"; // To reuse MockERC20
import "@openzeppelin/contracts/access/IAccessControl.sol";

contract LPRegistryTest is Test {
    LPRegistry public lpRegistry;
    MockERC20 public usdc; // Staking token

    address public admin = address(0x1);
    address public lp1 = address(0x10);
    address public lp2 = address(0x11);
    address public slasher = address(0x12);
    address public treasury = address(0x13); // Where slashed funds go

    uint256 public constant INITIAL_LP_BALANCE = 100_000 * 1e6; // 100k USDC
    uint256 public constant MIN_STAKE = 1_000 * 1e6; // 1k USDC
    uint256 public constant SLASH_PERCENT = 500; // 5%

    function setUp() public {
        vm.startPrank(admin);

        usdc = new MockERC20("USD Coin", "USDC");
        lpRegistry = new LPRegistry(address(usdc), MIN_STAKE, SLASH_PERCENT, treasury);

        // Grant SLASHER_ROLE to a specific address
        lpRegistry.grantRole(lpRegistry.SLASHER_ROLE(), slasher);

        vm.stopPrank();

        // Mint tokens to LPs
        usdc.mint(lp1, INITIAL_LP_BALANCE);
        usdc.mint(lp2, INITIAL_LP_BALANCE);
    }

    function test_Constructor() public view { // Changed to public view
        assertEq(address(lpRegistry.stakingToken()), address(usdc));
        assertEq(lpRegistry.minStakeAmount(), MIN_STAKE);
        assertEq(lpRegistry.slashingPenaltyPercent(), SLASH_PERCENT);
        assertTrue(lpRegistry.hasRole(lpRegistry.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(lpRegistry.hasRole(lpRegistry.SLASHER_ROLE(), admin)); // Deployer gets slasher role
        assertTrue(lpRegistry.hasRole(lpRegistry.SLASHER_ROLE(), slasher));
    }

    function test_RegisterLP_Success() public {
        vm.startPrank(lp1);
        usdc.approve(address(lpRegistry), MIN_STAKE);
        vm.expectEmit(true, false, false, true); // Corrected emit check
        emit LPRegistry.LPRegistered(lp1, MIN_STAKE);
        lpRegistry.registerLP(MIN_STAKE);
        vm.stopPrank();

        LPRegistry.LPInfo memory info = lpRegistry.getLPInfo(lp1);
        assertTrue(info.isRegistered);
        assertTrue(info.isActive);
        assertEq(info.stakedAmount, MIN_STAKE);
        assertTrue(lpRegistry.hasRole(lpRegistry.LP_ROLE(), lp1));
        assertEq(usdc.balanceOf(address(lpRegistry)), MIN_STAKE);
    }

    function test_RegisterLP_Fail_AlreadyRegistered() public {
        test_RegisterLP_Success(); // Register lp1 first

        vm.startPrank(lp1);
        usdc.approve(address(lpRegistry), MIN_STAKE);
        vm.expectRevert("LP already registered");
        lpRegistry.registerLP(MIN_STAKE);
        vm.stopPrank();
    }

    function test_RegisterLP_Fail_InsufficientStake() public {
        vm.startPrank(lp2);
        usdc.approve(address(lpRegistry), MIN_STAKE - 1);
        vm.expectRevert("Initial stake below minimum");
        lpRegistry.registerLP(MIN_STAKE - 1);
        vm.stopPrank();
    }

    function test_Stake_Success() public {
        test_RegisterLP_Success(); // lp1 registers

        uint256 additionalStake = 500 * 1e6;
        uint256 expectedTotalStake = MIN_STAKE + additionalStake;

        vm.startPrank(lp1);
        usdc.approve(address(lpRegistry), additionalStake);
        vm.expectEmit(true, false, false, true); // Corrected emit check
        emit LPRegistry.LPStaked(lp1, additionalStake, expectedTotalStake);
        lpRegistry.stake(additionalStake);
        vm.stopPrank();

        assertEq(lpRegistry.getLPInfo(lp1).stakedAmount, expectedTotalStake);
        assertEq(usdc.balanceOf(address(lpRegistry)), expectedTotalStake);
    }

    function test_Unstake_Success() public {
        test_Stake_Success(); // lp1 registers and stakes more

        uint256 unstakeAmount = 200 * 1e6;
        uint256 lpInitialBalance = usdc.balanceOf(lp1);
        uint256 expectedTotalStake = lpRegistry.getLPInfo(lp1).stakedAmount - unstakeAmount;

        vm.startPrank(lp1);
        vm.expectEmit(true, false, false, true); // Corrected emit check
        emit LPRegistry.LPUnstaked(lp1, unstakeAmount, expectedTotalStake);
        lpRegistry.unstake(unstakeAmount);
        vm.stopPrank();

        assertEq(lpRegistry.getLPInfo(lp1).stakedAmount, expectedTotalStake);
        assertEq(usdc.balanceOf(lp1), lpInitialBalance + unstakeAmount);
        assertEq(usdc.balanceOf(address(lpRegistry)), expectedTotalStake);
    }

    function test_Unstake_Fail_BelowMinStake() public {
        test_RegisterLP_Success(); // lp1 registers with MIN_STAKE

        vm.startPrank(lp1);
        vm.expectRevert("Remaining stake below minimum");
        lpRegistry.unstake(1); // Try to unstake 1, leaving below MIN_STAKE
        vm.stopPrank();
    }

    function test_Slash_Success() public {
        test_Stake_Success(); // lp1 registers and stakes more

        uint256 penaltyAmount = 100 * 1e6;
        uint256 lpRegistryBalance = usdc.balanceOf(address(lpRegistry));
        uint256 treasuryInitialBalance = usdc.balanceOf(treasury); // Changed to treasury
        uint256 expectedLPStake = lpRegistry.getLPInfo(lp1).stakedAmount - penaltyAmount;

        vm.startPrank(slasher);
        vm.expectEmit(true, false, false, true); // Corrected emit check
        emit LPRegistry.LPSlashed(lp1, penaltyAmount, expectedLPStake);
        lpRegistry.slash(lp1, penaltyAmount);
        vm.stopPrank();

        assertEq(lpRegistry.getLPInfo(lp1).stakedAmount, expectedLPStake);
        assertEq(usdc.balanceOf(address(lpRegistry)), lpRegistryBalance - penaltyAmount);
        assertEq(usdc.balanceOf(treasury), treasuryInitialBalance + penaltyAmount); // Slashed funds go to treasury
    }

    function test_Slash_Fail_NotSlasher() public {
        test_RegisterLP_Success();

        vm.startPrank(lp1); // lp1 tries to slash
        vm.expectRevert(abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, lp1, lpRegistry.SLASHER_ROLE()));
        lpRegistry.slash(lp1, 100);
        vm.stopPrank();
    }

    function test_Slash_MakesLPInactive() public {
        test_RegisterLP_Success(); // lp1 registers with MIN_STAKE

        uint256 penaltyAmount = MIN_STAKE; // Slash entire stake
        vm.startPrank(slasher);
        vm.expectEmit(true, false, false, true); // Corrected emit check
        emit LPRegistry.LPStatusChanged(lp1, false);
        lpRegistry.slash(lp1, penaltyAmount);
        vm.stopPrank();

        assertFalse(lpRegistry.getLPInfo(lp1).isActive);
    }

    function test_Admin_SetMinStakeAmount() public {
        vm.startPrank(admin);
        vm.expectEmit(false, false, false, true);
        emit LPRegistry.MinStakeAmountUpdated(2000 * 1e6);
        lpRegistry.setMinStakeAmount(2000 * 1e6);
        assertEq(lpRegistry.minStakeAmount(), 2000 * 1e6);
        vm.stopPrank();
    }

    function test_Admin_SetSlashingPenaltyPercent() public {
        vm.startPrank(admin);
        vm.expectEmit(false, false, false, true);
        emit LPRegistry.SlashingPenaltyPercentUpdated(1000);
        lpRegistry.setSlashingPenaltyPercent(1000); // 10%
        assertEq(lpRegistry.slashingPenaltyPercent(), 1000);
        vm.stopPrank();
    }

    function test_Admin_SetLPActiveStatus() public {
        test_RegisterLP_Success(); // lp1 registers
        vm.startPrank(admin);
        vm.expectEmit(true, false, false, true);
        emit LPRegistry.LPStatusChanged(lp1, false);
        lpRegistry.setLPActiveStatus(lp1, false);
        assertFalse(lpRegistry.getLPInfo(lp1).isActive);
        vm.stopPrank();
    }

    function test_ViewFunctions() public {
        assertFalse(lpRegistry.isLPActive(lp1)); // Not registered yet
        assertEq(lpRegistry.getLPInfo(lp1).stakedAmount, 0);

        test_RegisterLP_Success(); // lp1 registers

        assertTrue(lpRegistry.isLPActive(lp1));
        assertEq(lpRegistry.getLPInfo(lp1).stakedAmount, MIN_STAKE);

        vm.startPrank(admin);
        lpRegistry.setLPActiveStatus(lp1, false);
        vm.stopPrank();
        assertFalse(lpRegistry.getLPInfo(lp1).isActive); // Inactive
    }
}