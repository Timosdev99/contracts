// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "src/core/PaymentEscrow.sol";
import "src/interfaces/IPaymentEscrow.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// A mock ERC20 token for testing purposes
contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}

    function mint(address to, uint256 amount) public {
        _mint(to, amount);
    }
}

contract PaymentEscrowTest is Test {
    PaymentEscrow public escrow;
    MockERC20 public usdc;

    address public admin = address(0x1);
    address public sender = address(0x2);
    address public operator = address(0x3);
    address public operator2 = address(0x4);
    address public feeCollector = address(0x5);
    address public rateOracle = address(0x6);

    uint256 public constant INITIAL_SENDER_BALANCE = 1_000_000 * 1e6; // 1M USDC

    function setUp() public {
        // Use prank to set msg.sender for constructor calls
        vm.startPrank(admin);

        // Deploy mock USDC token
        usdc = new MockERC20("USD Coin", "USDC");

        // Deploy PaymentEscrow contract
        escrow = new PaymentEscrow(feeCollector);

        // Grant roles
        escrow.grantRole(escrow.OPERATOR_ROLE(), operator);
        escrow.grantRole(escrow.OPERATOR_ROLE(), operator2);
        escrow.grantRole(escrow.RATE_ORACLE_ROLE(), rateOracle);

        // Configure escrow settings
        escrow.addSupportedToken(address(usdc));
        
        vm.stopPrank();

        // Switch to the rate oracle to set an initial rate
        vm.startPrank(rateOracle);
        escrow.updateExchangeRate("NGN", 1500 * 1e6); // 1 USDC = 1500 NGN
        vm.stopPrank();

        // Fund the sender's wallet
        usdc.mint(sender, INITIAL_SENDER_BALANCE);
    }

    /// Test the successful creation of a payment
    function test_CreatePayment() public {
        uint256 paymentAmount = 100 * 1e6; // 100 USDC
        bytes32 recipientHash = keccak256("test_recipient");

        // Sender approves the escrow contract
        vm.prank(sender);
        usdc.approve(address(escrow), paymentAmount);

        // Calculate expected values
        uint256 fee = (paymentAmount * 50) / 10000;
        uint256 netAmount = paymentAmount - fee;
        uint256 rate = 1500 * 1e6;
        uint256 netFiatAmount = (netAmount * rate) / 1e6;

        // Expect the PaymentCreated event
        // We don't check paymentId (topic1), but we check sender (topic2) and the data payload.
        vm.expectEmit(false, true, false, true);
        emit IPaymentEscrow.PaymentCreated(
            bytes32(0), // This value is ignored
            sender,
            address(usdc),
            netAmount,
            "NGN",
            netFiatAmount
        );

        // Sender creates the payment
        vm.prank(sender);
        bytes32 paymentId = escrow.createPayment(
            address(usdc),
            paymentAmount,
            "NGN",
            recipientHash
        );

        // Verify balances
        assertEq(usdc.balanceOf(feeCollector), fee, "Fee collector balance incorrect");
        assertEq(usdc.balanceOf(address(escrow)), netAmount, "Escrow balance incorrect");
        assertEq(usdc.balanceOf(sender), INITIAL_SENDER_BALANCE - paymentAmount, "Sender balance incorrect");

        // Verify payment struct
        IPaymentEscrow.Payment memory p = escrow.getPayment(paymentId);
        assertEq(p.sender, sender);
        assertEq(p.amount, netAmount);
        assertEq(uint(p.status), uint(IPaymentEscrow.PaymentStatus.Pending));
    }

    /// Test the full lifecycle: Create -> Process -> Complete
    function test_Full_Payment_Lifecycle() public {
        uint256 paymentAmount = 100 * 1e6;
        bytes32 recipientHash = keccak256("test_recipient");

        // 1. Create Payment
        vm.prank(sender);
        usdc.approve(address(escrow), paymentAmount);
        vm.prank(sender);
        bytes32 paymentId = escrow.createPayment(address(usdc), paymentAmount, "NGN", recipientHash);
        uint256 netAmount = escrow.getPayment(paymentId).amount;

        // 2. Mark as Processing
        vm.prank(operator);
        escrow.markProcessing(paymentId, "bank_ref_123");
        assertEq(uint(escrow.getPayment(paymentId).status), uint(IPaymentEscrow.PaymentStatus.Processing));
        assertEq(escrow.getPayment(paymentId).operator, operator);

        // 3. Complete Payment
        uint256 operatorInitialBalance = usdc.balanceOf(operator);
        vm.prank(operator);
        escrow.completePayment(paymentId, keccak256("proof"));
        
        assertEq(uint(escrow.getPayment(paymentId).status), uint(IPaymentEscrow.PaymentStatus.Completed));
        assertEq(usdc.balanceOf(address(escrow)), 0, "Escrow should be empty");
        assertEq(usdc.balanceOf(operator), operatorInitialBalance + netAmount, "Operator did not receive funds");
    }

    /// Test that a user can claim a refund after the deadline
    function test_Refund_After_Deadline() public {
        uint256 paymentAmount = 50 * 1e6;
        
        // Create payment
        vm.prank(sender);
        usdc.approve(address(escrow), paymentAmount);
        vm.prank(sender);
        bytes32 paymentId = escrow.createPayment(address(usdc), paymentAmount, "NGN", keccak256("recipient"));
        uint256 netAmount = escrow.getPayment(paymentId).amount;

        // Advance time past the deadline
        uint256 deadline = escrow.getPayment(paymentId).deadline;
        vm.warp(deadline + 1);

        // Sender claims refund
        uint256 senderInitialBalance = usdc.balanceOf(sender);
        vm.prank(sender);
        escrow.claimRefund(paymentId);

        // Verify state
        assertEq(uint(escrow.getPayment(paymentId).status), uint(IPaymentEscrow.PaymentStatus.Refunded));
        assertEq(usdc.balanceOf(address(escrow)), 0);
        assertEq(usdc.balanceOf(sender), senderInitialBalance + netAmount, "Sender did not get refund");
    }

    /// Test the full dispute and resolution flow
    function test_Dispute_And_Resolve() public {
        uint256 paymentAmount = 200 * 1e6;

        // 1. Create and process payment
        vm.prank(sender);
        usdc.approve(address(escrow), paymentAmount);
        vm.prank(sender);
        bytes32 paymentId = escrow.createPayment(address(usdc), paymentAmount, "NGN", keccak256("recipient"));
        uint256 netAmount = escrow.getPayment(paymentId).amount;

        vm.prank(operator);
        escrow.markProcessing(paymentId, "bank_ref_dispute");

        // 2. Sender disputes the payment
        vm.prank(sender);
        escrow.disputePayment(paymentId);
        assertEq(uint(escrow.getPayment(paymentId).status), uint(IPaymentEscrow.PaymentStatus.Disputed));

        // 3. Admin resolves the dispute in favor of the sender
        uint256 senderInitialBalance = usdc.balanceOf(sender);
        vm.prank(admin);
        escrow.resolveDispute(paymentId, true); // true = refund to sender

        assertEq(uint(escrow.getPayment(paymentId).status), uint(IPaymentEscrow.PaymentStatus.Refunded));
        assertEq(usdc.balanceOf(sender), senderInitialBalance + netAmount, "Sender did not get dispute refund");
        assertEq(usdc.balanceOf(address(escrow)), 0);

        // --- New scenario: Resolve in favor of operator ---
        
        // 1. Create and process another payment
        vm.prank(sender);
        usdc.approve(address(escrow), paymentAmount);
        vm.prank(sender);
        bytes32 paymentId2 = escrow.createPayment(address(usdc), paymentAmount, "NGN", keccak256("recipient2"));
        uint256 netAmount2 = escrow.getPayment(paymentId2).amount;

        vm.prank(operator);
        escrow.markProcessing(paymentId2, "bank_ref_dispute2");

        // 2. Sender disputes
        vm.prank(sender);
        escrow.disputePayment(paymentId2);

        // 3. Admin resolves in favor of the operator
        uint256 operatorInitialBalance = usdc.balanceOf(operator);
        vm.prank(admin);
        escrow.resolveDispute(paymentId2, false); // false = complete to operator

        assertEq(uint(escrow.getPayment(paymentId2).status), uint(IPaymentEscrow.PaymentStatus.Completed));
        assertEq(usdc.balanceOf(operator), operatorInitialBalance + netAmount2, "Operator did not get dispute win funds");
        assertEq(usdc.balanceOf(address(escrow)), 0);
    }

    /// Test failure cases

    function test_Fail_CreatePayment_UnsupportedToken() public {
        vm.prank(sender);
        vm.expectRevert("Token not supported");
        escrow.createPayment(address(this), 100, "NGN", keccak256("r"));
    }

    function test_Fail_CompletePayment_WrongOperator() public {
        // Create and process with `operator`
        vm.prank(sender);
        usdc.approve(address(escrow), 100);
        vm.prank(sender);
        bytes32 paymentId = escrow.createPayment(address(usdc), 100, "NGN", keccak256("r"));
        vm.prank(operator);
        escrow.markProcessing(paymentId, "ref");

        // Try to complete with `operator2`
        vm.prank(operator2);
        vm.expectRevert("Not assigned operator");
        escrow.completePayment(paymentId, keccak256("proof"));
    }

    function test_Fail_ClaimRefund_BeforeDeadline() public {
        vm.prank(sender);
        usdc.approve(address(escrow), 100);
        vm.prank(sender);
        bytes32 paymentId = escrow.createPayment(address(usdc), 100, "NGN", keccak256("r"));

        vm.prank(sender);
        vm.expectRevert("Deadline not passed");
        escrow.claimRefund(paymentId);
    }
}
