// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {DreamAgentNFT} from "src/dream/execution-chain/DreamAgentNFT.sol";
import {IAccessControl} from "lib/openzeppelin-contracts/contracts/access/IAccessControl.sol";

contract DreamAgentNFTTest is Test {
    DreamAgentNFT public nft;

    address public admin;
    address public minter;
    address public user;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;

    function setUp() public {
        admin = makeAddr("admin");
        minter = makeAddr("minter");
        user = makeAddr("user");

        vm.startPrank(admin);
        nft = new DreamAgentNFT(admin, minter);
        vm.stopPrank();
    }

    function test_Constructor() public view {
        assertTrue(nft.hasRole(DEFAULT_ADMIN_ROLE, admin));
        assertTrue(nft.hasRole(MINTER_ROLE, minter));
        assertEq(nft.name(), "DreamAgentNFT");
        assertEq(nft.symbol(), "DREAM");
    }

    function test_SafeMint() public {
        vm.startPrank(minter);
        nft.safeMint(user);
        assertEq(nft.balanceOf(user), 1);
        assertEq(nft.ownerOf(0), user);
        vm.stopPrank();
    }

    function test_RevertWhen_NonMinterCallsSafeMint() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, MINTER_ROLE)
        );
        nft.safeMint(user);
        vm.stopPrank();
    }

    function test_SetBaseAgentNFTURI() public {
        string memory newURI = "ipfs://newuri/";
        vm.startPrank(admin);
        nft.setBaseAgentNFTURI(newURI);
        assertEq(nft.baseAgentNFTURI(), newURI);
        vm.stopPrank();
    }

    function test_RevertWhen_NonAdminCallsSetBaseURI() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, DEFAULT_ADMIN_ROLE)
        );
        nft.setBaseAgentNFTURI("ipfs://newuri/");
        vm.stopPrank();
    }

    function test_TokenURIAppending() public {
        string memory baseURI = "ipfs://baseuri/";
        vm.startPrank(admin);
        nft.setBaseAgentNFTURI(baseURI);
        vm.stopPrank();

        vm.startPrank(minter);
        nft.safeMint(user);
        assertEq(nft.baseAgentNFTURI(), baseURI);
        vm.stopPrank();
    }

    function test_SupportsInterface() public view {
        // Test ERC721 interface support
        bytes4 erc721InterfaceId = 0x80ac58cd;
        assertTrue(nft.supportsInterface(erc721InterfaceId));

        // Test ERC165 interface support
        bytes4 erc165InterfaceId = 0x01ffc9a7;
        assertTrue(nft.supportsInterface(erc165InterfaceId));

        // Test AccessControl interface support
        bytes4 accessControlInterfaceId = 0x7965db0b;
        assertTrue(nft.supportsInterface(accessControlInterfaceId));
    }
}
