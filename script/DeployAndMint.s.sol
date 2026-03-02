// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../test/MockERC20.sol";
import "../src/core/LPRegistry.sol";

contract DeployAndMintScript is Script {
    MockERC20 public mockToken;
    LPRegistry public lpRegistry;

    function run() public {
        vm.startBroadcast();

        //Deploy MockERC20
        mockToken = new MockERC20("MockUSD", "MUSD", 1_000_000 ether); //deployer gets initial balance
        console.log("MockERC20 deployed at:", address(mockToken));

        // mint token to target LP address
        address lpAddress = 0xedC0129245cDe0410e5dbD2f7bCA4b505cEd2c3e;
        uint256 amountToMint = 100_000 ether; // mint 100,000 MUSD for the LP
        mockToken.mint(lpAddress, amountToMint);
        console.log("Minted", amountToMint, "MUSD to LP address:", lpAddress);
        console.log("LP's MUSD balance:", mockToken.balanceOf(lpAddress));

        //Deploy LPRegistry
        uint256 minStakeAmount = 10_000 ether; // 10,000 MUSD
        uint256 slashingPenaltyPercent = 500; // 5%
        address treasuryAddress = vm.addr(0x1337); // mock treasury address

        lpRegistry = new LPRegistry(address(mockToken), minStakeAmount, slashingPenaltyPercent, treasuryAddress);
        console.log("LPRegistry deployed at:", address(lpRegistry));
        console.log("LPRegistry staking token:", address(lpRegistry.stakingToken()));
        console.log("LPRegistry min stake amount:", lpRegistry.minStakeAmount());
        console.log("LPRegistry slashing penalty percent:", lpRegistry.slashingPenaltyPercent());

        vm.stopBroadcast();
    }
}
