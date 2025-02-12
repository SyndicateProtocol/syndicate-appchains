// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {DreamAgentNFT} from "src/use-cases/dream/execution-chain/DreamAgentNFT.sol";
import {AgentClaimNFT, Ownable} from "src/use-cases/dream/execution-chain/AgentClaimNFT.sol";

contract AgentClaimNFTTest is Test {
    DreamAgentNFT public nft;
    AgentClaimNFT public claimContract;

    address public admin;
    address public agent1;
    address public agent2;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    event ClaimPermissionGranted(address indexed agent);
    event ClaimPermissionRevoked(address indexed agent);
    event NFTClaimed(address indexed agent, uint256 tokenId);

    function setUp() public {
        admin = makeAddr("admin");
        agent1 = makeAddr("agent1");
        agent2 = makeAddr("agent2");

        vm.startPrank(admin);
        // Deploy NFT contract with claim contract as minter
        nft = new DreamAgentNFT(admin, address(this));
        claimContract = new AgentClaimNFT(address(nft), admin);

        // Grant minter role to claim contract
        nft.grantRole(MINTER_ROLE, address(claimContract));
        vm.stopPrank();
    }

    function test_Constructor() public view {
        assertEq(address(claimContract.dreamAgentNFT()), address(nft));
        assertEq(claimContract.owner(), admin);
        assertTrue(nft.hasRole(MINTER_ROLE, address(claimContract)));
    }

    function test_GrantClaimPermission() public {
        vm.startPrank(admin);
        vm.expectEmit(true, false, false, false);
        emit ClaimPermissionGranted(agent1);

        claimContract.grantClaimPermission(agent1);
        assertTrue(claimContract.canClaim(agent1));
        vm.stopPrank();
    }

    function test_RevokeClaimPermission() public {
        vm.startPrank(admin);
        claimContract.grantClaimPermission(agent1);

        vm.expectEmit(true, false, false, false);
        emit ClaimPermissionRevoked(agent1);

        claimContract.revokeClaimPermission(agent1);
        assertFalse(claimContract.canClaim(agent1));
        vm.stopPrank();
    }

    function test_BatchGrantClaimPermission() public {
        address[] memory agents = new address[](2);
        agents[0] = agent1;
        agents[1] = agent2;

        vm.startPrank(admin);
        vm.expectEmit(true, false, false, false);
        emit ClaimPermissionGranted(agent1);
        vm.expectEmit(true, false, false, false);
        emit ClaimPermissionGranted(agent2);

        claimContract.batchGrantClaimPermission(agents);
        assertTrue(claimContract.canClaim(agent1));
        assertTrue(claimContract.canClaim(agent2));
        vm.stopPrank();
    }

    function test_ClaimNFT() public {
        vm.startPrank(admin);
        claimContract.grantClaimPermission(agent1);
        vm.stopPrank();

        vm.startPrank(agent1);
        vm.expectEmit(true, false, false, true);
        emit NFTClaimed(agent1, 1);

        claimContract.claimNFT();

        assertTrue(claimContract.hasClaimed(agent1));
        assertEq(nft.balanceOf(agent1), 1);
        assertEq(nft.ownerOf(0), agent1);
        vm.stopPrank();
    }

    function test_RevertWhen_UnauthorizedClaim() public {
        vm.startPrank(agent1);
        vm.expectRevert(AgentClaimNFT.NotAllowedToClaim.selector);
        claimContract.claimNFT();
        vm.stopPrank();
    }

    function test_RevertWhen_DoubleClaim() public {
        vm.startPrank(admin);
        claimContract.grantClaimPermission(agent1);
        vm.stopPrank();

        vm.startPrank(agent1);
        claimContract.claimNFT();

        vm.expectRevert(AgentClaimNFT.AlreadyClaimed.selector);
        claimContract.claimNFT();
        vm.stopPrank();
    }

    function test_IsEligibleToClaim() public {
        vm.startPrank(admin);
        claimContract.grantClaimPermission(agent1);
        vm.stopPrank();

        assertTrue(claimContract.isEligibleToClaim(agent1));

        vm.prank(agent1);
        claimContract.claimNFT();

        assertFalse(claimContract.isEligibleToClaim(agent1));
    }

    function test_RevertWhen_NonOwnerGrantsPermission() public {
        vm.startPrank(agent1);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, agent1));
        claimContract.grantClaimPermission(agent2);
        vm.stopPrank();
    }
}
