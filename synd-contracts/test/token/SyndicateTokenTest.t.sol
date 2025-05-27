// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract SyndicateTokenTest is Test {
    SyndicateToken public token;

    address public defaultAdmin;
    address public minter;
    address public user;

    // Events from the contract that we'll test
    event Transfer(address indexed from, address indexed to, uint256 value);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);

    function setUp() public {
        defaultAdmin = address(0x1);
        minter = address(0x2);
        user = address(0x3);

        token = new SyndicateToken(defaultAdmin, minter);
    }

    // test that constructor cannot be called with zero addresses
    function test_ConstructorWithZeroAddresseForAdmin() public {
        vm.expectRevert("Minter cannot be zero address");
        new SyndicateToken(defaultAdmin, address(0));
    }

    function test_ConstructorWithZeroAddresseForMinter() public {
        vm.expectRevert("Default admin cannot be zero address");
        new SyndicateToken(address(0), minter);
    }

    // Test initial setup
    function test_InitialSetup() public view {
        assertEq(token.name(), "Syndicate");
        assertEq(token.symbol(), "SYND");
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), defaultAdmin));
        assertTrue(token.hasRole(token.MINTER_ROLE(), minter));
    }

    // Test minting
    function test_MintingWithMinterRole() public {
        vm.prank(minter);
        token.mint(user, 1000);
        assertEq(token.balanceOf(user), 1000);
    }

    function test_MintingEmitsTransferEvent() public {
        vm.prank(minter);
        vm.expectEmit(true, true, false, true);
        emit Transfer(address(0), user, 1000);
        token.mint(user, 1000);
    }

    function test_RevertWhenMintingWithoutMinterRole() public {
        vm.startPrank(user);
        bytes memory errorMessage =
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.MINTER_ROLE());
        vm.expectRevert(errorMessage);
        token.mint(user, 1000);

        vm.stopPrank();
    }

    function test_MintingRevertMessage() public {
        bytes32 minterRole = token.MINTER_ROLE();
        vm.prank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, minterRole)
        );
        token.mint(user, 1000);
    }

    // Test role management
    function test_AdminCanGrantMinterRole() public {
        vm.startPrank(defaultAdmin);
        vm.expectEmit(true, true, true, true);
        emit RoleGranted(token.MINTER_ROLE(), user, defaultAdmin);
        token.grantRole(token.MINTER_ROLE(), user);
        assertTrue(token.hasRole(token.MINTER_ROLE(), user));
        vm.stopPrank();
    }

    function test_AdminCanRevokeMinterRole() public {
        vm.startPrank(defaultAdmin);
        vm.expectEmit(true, true, true, true);
        emit RoleRevoked(token.MINTER_ROLE(), minter, defaultAdmin);
        token.revokeRole(token.MINTER_ROLE(), minter);
        assertFalse(token.hasRole(token.MINTER_ROLE(), minter));
        vm.stopPrank();
    }

    function test_RevertIfNonAdminGrantsRole() public {
        bytes32 minterRole = token.MINTER_ROLE();

        vm.startPrank(user);

        bytes memory errorMessage = abi.encodeWithSelector(
            IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.DEFAULT_ADMIN_ROLE()
        );
        vm.expectRevert(errorMessage);
        token.grantRole(minterRole, user);

        assertFalse(token.hasRole(minterRole, user), "User should not have the minter role");

        vm.stopPrank();
    }

    // Test permit functionality
    function test_Permit() public {
        uint256 privateKey = 0xA11CE;
        address owner = vm.addr(privateKey);
        address spender = address(0x4);
        uint256 value = 1000;
        uint256 deadline = block.timestamp + 1 hours;

        // Get the current nonce
        uint256 nonce = token.nonces(owner);

        // Get the chain ID
        uint256 chainId;
        assembly {
            chainId := chainid()
        }

        // Prepare permit signature
        bytes32 structHash = keccak256(
            abi.encode(
                keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"),
                owner,
                spender,
                value,
                nonce,
                deadline
            )
        );

        bytes32 domainSeparator = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(token.name())),
                keccak256(bytes("1")),
                chainId,
                address(token)
            )
        );

        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);

        // Execute permit
        token.permit(owner, spender, value, deadline, v, r, s);

        // Verify allowance
        assertEq(token.allowance(owner, spender), value);
    }

    // Test transfer functionality
    function test_Transfer() public {
        // First mint some tokens to the sender
        vm.prank(minter);
        token.mint(user, 1000);

        // Then test transfer
        vm.prank(user);
        token.transfer(address(0x4), 500);

        assertEq(token.balanceOf(user), 500);
        assertEq(token.balanceOf(address(0x4)), 500);
    }

    // Test approve and transferFrom
    function test_ApproveAndTransferFrom() public {
        // Mint tokens to the owner
        vm.prank(minter);
        token.mint(user, 1000);

        // Approve spending
        vm.prank(user);
        token.approve(address(0x4), 500);

        // Transfer using transferFrom
        vm.prank(address(0x4));
        token.transferFrom(user, address(0x4), 500);

        assertEq(token.balanceOf(user), 500);
        assertEq(token.balanceOf(address(0x4)), 500);
    }

    // Fuzz testing for minting
    function testFuzz_Mint(address to, uint256 amount) public {
        vm.assume(to != address(0));
        vm.assume(amount > 0 && amount < type(uint256).max);

        vm.prank(minter);
        token.mint(to, amount);

        assertEq(token.balanceOf(to), amount);
    }

    // Invariant test helper - total supply should always equal sum of balances
    function test_SupplyMatchesBalances() public {
        // Mint to multiple addresses
        vm.startPrank(minter);
        token.mint(address(0xA), 1000);
        token.mint(address(0xB), 2000);
        token.mint(address(0xC), 3000);
        vm.stopPrank();

        uint256 totalSupply = token.totalSupply();
        uint256 sumOfBalances =
            token.balanceOf(address(0xA)) + token.balanceOf(address(0xB)) + token.balanceOf(address(0xC));

        assertEq(totalSupply, sumOfBalances);
    }
}
