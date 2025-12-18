// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "src/core/PaymentEscrow.sol";
import "src/core/LPRegistry.sol";
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
    LPRegistry public lpRegistry;
    MockERC20 public usdc;

    // --- Actors ---
    address public admin = address(0x1);
    address public sender = address(0x2); // The user selling USDC
    address public lp = address(0x3); // The Liquidity Provider buying the USDC
    address public feeCollector = address(0x5);
    address public oracleWallet = address(0x6); // The multi-sig oracle wallet
    address public treasury = address(0x8); // Treasury for the LP Registry

    // --- Permission Slip Signer ---
    uint256 public constant PERMISSION_SIGNER_PK = 0xABCD;
    address public permissionSlipSigner = vm.addr(PERMISSION_SIGNER_PK);

    // --- Balances & Amounts ---
    uint256 public constant INITIAL_SENDER_BALANCE = 1_000_000 * 1e6;
    uint256 public constant INITIAL_LP_BALANCE = 100_000 * 1e6;
    uint256 public constant MIN_STAKE_AMOUNT = 1_000 * 1e6;

    function setUp() public {
        // --- Deploy Contracts ---
        vm.startPrank(admin);
        usdc = new MockERC20("USD Coin", "USDC");
        lpRegistry = new LPRegistry(address(usdc), MIN_STAKE_AMOUNT, 500, treasury);
        escrow = new PaymentEscrow(
            feeCollector,
            address(lpRegistry),
            oracleWallet,
            permissionSlipSigner
        );
        escrow.addSupportedToken(address(usdc));
        vm.stopPrank();

        // --- Fund Sender ---
        usdc.mint(sender, INITIAL_SENDER_BALANCE);

        // --- Register LP ---
        usdc.mint(lp, INITIAL_LP_BALANCE);
        vm.startPrank(lp);
        usdc.approve(address(lpRegistry), MIN_STAKE_AMOUNT);
        lpRegistry.registerLP(MIN_STAKE_AMOUNT);
        vm.stopPrank();
    }

    /// @notice Tests the full, successful payment lifecycle with the new decentralized architecture.
    function test_Full_Payment_Lifecycle() public {
        // 1. --- Create Payment ---
        uint256 paymentAmount = 100 * 1e6; // 100 USDC
        bytes32 recipientHash = keccak256("test_recipient");

        uint256 fee = (paymentAmount * escrow.platformFeePercent()) / 10000;
        uint256 netAmount = paymentAmount - fee;

        vm.startPrank(sender);
        usdc.approve(address(escrow), paymentAmount);

        bytes32 paymentId = escrow.createPayment(address(usdc), paymentAmount, 150000, "NGN", recipientHash);
        vm.stopPrank();

        IPaymentEscrow.Payment memory p = escrow.getPayment(paymentId);
        assertEq(uint(p.status), uint(IPaymentEscrow.PaymentStatus.Pending));
        assertEq(p.amount, netAmount);

        // 2. --- Claim Payment ---
        // Generate a valid permission slip
        bytes32 messageHash = keccak256(abi.encodePacked(paymentId, lp));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(PERMISSION_SIGNER_PK, messageHash);
        bytes memory permissionSlip = abi.encodePacked(r, s, v);

        vm.startPrank(lp);
        escrow.claimPayment(paymentId, permissionSlip);
        vm.stopPrank();

        p = escrow.getPayment(paymentId);
        assertEq(uint(p.status), uint(IPaymentEscrow.PaymentStatus.Processing));
        assertEq(p.operator, lp);

        // 3. --- Confirm Settlement ---
        uint256 lpInitialBalance = usdc.balanceOf(lp);

        vm.startPrank(oracleWallet);
        escrow.confirmSettlement(paymentId);
        vm.stopPrank();

        p = escrow.getPayment(paymentId);
        assertEq(uint(p.status), uint(IPaymentEscrow.PaymentStatus.Completed));
        assertEq(usdc.balanceOf(address(escrow)), 0, "Escrow contract should be empty");
        assertEq(usdc.balanceOf(lp), lpInitialBalance + netAmount, "LP did not receive funds");
    }

    function test_Fail_Claim_With_Invalid_Slip() public {
        vm.startPrank(sender);
        usdc.approve(address(escrow), 100 * 1e6);
        bytes32 paymentId = escrow.createPayment(address(usdc), 100 * 1e6, 150000, "NGN", keccak256("r"));
        vm.stopPrank();

        bytes32 messageHash = keccak256(abi.encodePacked(paymentId, lp));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(0xDEADBEEF, messageHash);
        bytes memory invalidSlip = abi.encodePacked(r, s, v);

        vm.startPrank(lp);
        vm.expectRevert("INVALID_SLIP");
        escrow.claimPayment(paymentId, invalidSlip);
        vm.stopPrank();
    }

    function test_Fail_Claim_When_Not_Active_LP() public {
        address notAnLp = address(0x99);

        vm.startPrank(sender);
        usdc.approve(address(escrow), 100 * 1e6);
        bytes32 paymentId = escrow.createPayment(address(usdc), 100 * 1e6, 150000, "NGN", keccak256("r"));
        vm.stopPrank();

        bytes32 messageHash = keccak256(abi.encodePacked(paymentId, notAnLp));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(PERMISSION_SIGNER_PK, messageHash);
        bytes memory permissionSlip = abi.encodePacked(r, s, v);

        vm.startPrank(notAnLp);
        vm.expectRevert("NOT_ACTIVE_LP");
        escrow.claimPayment(paymentId, permissionSlip);
        vm.stopPrank();
    }

    function test_Fail_Confirm_From_Non_Oracle() public {
        bytes32 paymentId;
        // Scope to create and claim payment
        {
            vm.startPrank(sender);
            usdc.approve(address(escrow), 100 * 1e6);
            paymentId = escrow.createPayment(address(usdc), 100 * 1e6, 150000, "NGN", keccak256("r"));
            vm.stopPrank();

            bytes32 messageHash = keccak256(abi.encodePacked(paymentId, lp));
            (uint8 v, bytes32 r, bytes32 s) = vm.sign(PERMISSION_SIGNER_PK, messageHash);
            bytes memory permissionSlip = abi.encodePacked(r, s, v);
            
            vm.startPrank(lp);
            escrow.claimPayment(paymentId, permissionSlip);
            vm.stopPrank();
        }

        vm.startPrank(admin); // admin is not the oracleWallet
        vm.expectRevert("UNAUTHORIZED_ORACLE");
        escrow.confirmSettlement(paymentId);
        vm.stopPrank();
    }

    function test_User_Can_Claim_Refund_After_Deadline() public {
        uint256 paymentAmount = 50 * 1e6;
        bytes32 paymentId;
        uint256 netAmount;

        vm.startPrank(sender);
        usdc.approve(address(escrow), paymentAmount);
        paymentId = escrow.createPayment(address(usdc), paymentAmount, 75000, "NGN", keccak256("r"));
        vm.stopPrank();
        
        netAmount = escrow.getPayment(paymentId).amount;

        uint256 deadline = escrow.getPayment(paymentId).deadline;
        vm.warp(deadline + 1);

        uint256 senderInitialBalance = usdc.balanceOf(sender);
        vm.startPrank(sender);
        escrow.claimRefund(paymentId);
        vm.stopPrank();

        assertEq(uint(escrow.getPayment(paymentId).status), uint(IPaymentEscrow.PaymentStatus.Refunded));
        assertEq(usdc.balanceOf(address(escrow)), 0);
        assertEq(usdc.balanceOf(sender), senderInitialBalance + netAmount, "Sender did not get refund");
    }
}