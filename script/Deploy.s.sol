// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {RateOracle} from "../src/core/RateOracle.sol";
import {LPRegistry} from "../src/core/LPRegistry.sol";
import {OnRampEscrow} from "../src/core/OnRampEscrow.sol";
import {PaymentEscrow} from "../src/core/PaymentEscrow.sol";
import {MockERC20} from "../test/MockERC20.sol";

contract Deploy is Script {
    function run() external returns (RateOracle, LPRegistry, OnRampEscrow, PaymentEscrow, MockERC20) {
        vm.startBroadcast();

        // --- Deploy Mock Staking Token ---
        MockERC20 stakingToken = new MockERC20("Mock Staking Token", "MST", 1_000_000 * 1e18);

        // --- Deploy RateOracle ---
        // maxPriceAge: 30 minutes (1800 seconds)
        RateOracle rateOracle = new RateOracle(1800);

        // --- Deploy LPRegistry ---
        // minStakeAmount: 500 tokens
        // slashingPenaltyPercent: 5% (500 basis points)
        // treasuryAddress: Read from TREASURY_ADDRESS env var
        LPRegistry lpRegistry = new LPRegistry(
            address(stakingToken),
            500 * 1e18,
            500,
            vm.envAddress("TREASURY_ADDRESS")
        );

        // --- Deploy OnRampEscrow ---
        OnRampEscrow onRampEscrow = new OnRampEscrow();

        // --- Deploy PaymentEscrow ---
        // feeCollector: deployer wallet
        // oracleWallet: deployer wallet
        // permissionSlipSigner: deployer wallet
        PaymentEscrow paymentEscrow = new PaymentEscrow(
            msg.sender,
            address(lpRegistry),
            msg.sender,
            msg.sender
        );

        // --- Post-Deployment Configuration ---

        // Grant the deployer the LP_ROLE and SLASHER_ROLE in LPRegistry
        lpRegistry.grantRole(lpRegistry.LP_ROLE(), msg.sender);
        lpRegistry.grantRole(lpRegistry.SLASHER_ROLE(), msg.sender);

        // Grant the deployer the LP_ROLE in OnRampEscrow
        onRampEscrow.grantRole(onRampEscrow.LP_ROLE(), msg.sender);

        // Add the mock staking token as a supported token in PaymentEscrow
        paymentEscrow.addSupportedToken(address(stakingToken));

        vm.stopBroadcast();
        return (rateOracle, lpRegistry, onRampEscrow, paymentEscrow, stakingToken);
    }
}