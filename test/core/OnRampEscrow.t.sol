// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "src/core/OnRampEscrow.sol";
import "src/interfaces/IOnRampEscrow.sol";
import "test/core/PaymentEscrow.t.sol"; // To reuse MockERC20

contract OnRampEscrowTest is Test {
    OnRampEscrow public escrow;
    MockERC20 public usdc;

    address public admin = address(0x1);
    address public buyer = address(0x7);
    address public lp = address(0x8); // Liquidity Provider

    uint256 public constant INITIAL_LP_BALANCE = 1_000_000 * 1e6; // 1M USDC

    function setUp() public {
        vm.startPrank(admin);
        usdc = new MockERC20("USD Coin", "USDC");
        escrow = new OnRampEscrow();
        escrow.grantRole(escrow.LP_ROLE(), lp);
        vm.stopPrank();

        usdc.mint(lp, INITIAL_LP_BALANCE);
    }

    /// Test the full, successful on-ramping lifecycle
    function test_Full_OnRamp_Lifecycle() public {
        uint256 tokenAmount = 100 * 1e6; // 100 USDC
        uint256 fiatAmount = 150000; // 150,000 NGN

        // 1. Buyer creates an order
        vm.prank(buyer);
        bytes32 orderId = escrow.createOnRampOrder(address(usdc), tokenAmount, fiatAmount, "NGN");
        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.Pending));

        // 2. LP locks the funds
        vm.prank(lp);
        usdc.approve(address(escrow), tokenAmount);
        vm.prank(lp);
        escrow.lockFunds(orderId);
        
        assertEq(usdc.balanceOf(address(escrow)), tokenAmount, "Escrow balance incorrect");
        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.FundsLocked));

        // 3. Buyer confirms they have sent the fiat
        vm.prank(buyer);
        escrow.confirmFiatSent(orderId, keccak256("proof_of_payment"));
        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.FiatSent));

        // 4. LP confirms fiat receipt and releases the crypto
        uint256 buyerInitialBalance = usdc.balanceOf(buyer);
        vm.prank(lp);
        escrow.releaseFunds(orderId);

        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.Completed));
        assertEq(usdc.balanceOf(buyer), buyerInitialBalance + tokenAmount, "Buyer did not receive crypto");
        assertEq(usdc.balanceOf(address(escrow)), 0, "Escrow should be empty");
    }

    /// Test that the LP can reclaim funds if the user never pays
    function test_Reclaim_If_User_Times_Out() public {
        uint256 tokenAmount = 50 * 1e6;

        // 1. Create order and lock funds
        vm.prank(buyer);
        bytes32 orderId = escrow.createOnRampOrder(address(usdc), tokenAmount, 75000, "NGN");
        vm.prank(lp);
        usdc.approve(address(escrow), tokenAmount);
        vm.prank(lp);
        escrow.lockFunds(orderId);

        // 2. Advance time past the user's payment deadline
        uint256 deadline = escrow.getOrder(orderId).fundsLockedAt + escrow.paymentDeadline();
        vm.warp(deadline + 1);

        // 3. LP reclaims their locked funds
        uint256 lpInitialBalance = usdc.balanceOf(lp);
        vm.prank(lp);
        escrow.reclaimLockedFunds(orderId);

        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.Cancelled));
        assertEq(usdc.balanceOf(lp), lpInitialBalance + tokenAmount, "LP did not get funds back");
        assertEq(usdc.balanceOf(address(escrow)), 0);
    }

    /// Test that the buyer can cancel if the LP never locks funds
    function test_Cancel_If_LP_Times_Out() public {
        // 1. Buyer creates an order
        vm.prank(buyer);
        bytes32 orderId = escrow.createOnRampOrder(address(usdc), 100 * 1e6, 150000, "NGN");

        // 2. Advance time past the LP's lock deadline
        uint256 deadline = escrow.getOrder(orderId).createdAt + escrow.lockDeadline();
        vm.warp(deadline + 1);

        // 3. Buyer cancels the order
        vm.prank(buyer);
        escrow.cancelOrder(orderId);

        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.Cancelled));
    }

    /// Test the full dispute and resolution flow
    function test_Dispute_And_Resolve() public {
        uint256 tokenAmount = 200 * 1e6;

        // 1. Go through flow until fiat is marked as sent
        vm.prank(buyer);
        bytes32 orderId = escrow.createOnRampOrder(address(usdc), tokenAmount, 300000, "NGN");
        vm.prank(lp);
        usdc.approve(address(escrow), tokenAmount);
        vm.prank(lp);
        escrow.lockFunds(orderId);
        vm.prank(buyer);
        escrow.confirmFiatSent(orderId, keccak256("proof"));

        // 2. LP disputes the payment (e.g., claims they never received fiat)
        vm.prank(lp);
        escrow.disputeOrder(orderId);
        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.Disputed));

        // 3. Admin resolves in favor of the LP (gives crypto back to LP)
        uint256 lpInitialBalance = usdc.balanceOf(lp);
        vm.prank(admin);
        escrow.resolveDispute(orderId, false); // false = release to LP

        assertEq(uint(escrow.getOrder(orderId).status), uint(IOnRampEscrow.OrderStatus.Cancelled));
        assertEq(usdc.balanceOf(lp), lpInitialBalance + tokenAmount, "LP did not get dispute refund");
        assertEq(usdc.balanceOf(address(escrow)), 0);

        // --- New scenario: Resolve in favor of buyer ---
        
        // 1. Create and process another payment
        vm.prank(buyer);
        bytes32 orderId2 = escrow.createOnRampOrder(address(usdc), tokenAmount, 300000, "NGN");
        vm.prank(lp);
        usdc.approve(address(escrow), tokenAmount);
        vm.prank(lp);
        escrow.lockFunds(orderId2);
        vm.prank(buyer);
        escrow.confirmFiatSent(orderId2, keccak256("proof2"));

        // 2. LP disputes
        vm.prank(lp);
        escrow.disputeOrder(orderId2);

        // 3. Admin resolves in favor of the buyer
        uint256 buyerInitialBalance = usdc.balanceOf(buyer);
        vm.prank(admin);
        escrow.resolveDispute(orderId2, true); // true = release to buyer

        assertEq(uint(escrow.getOrder(orderId2).status), uint(IOnRampEscrow.OrderStatus.Completed));
        assertEq(usdc.balanceOf(buyer), buyerInitialBalance + tokenAmount, "Buyer did not get dispute win funds");
        assertEq(usdc.balanceOf(address(escrow)), 0);
    }
}
