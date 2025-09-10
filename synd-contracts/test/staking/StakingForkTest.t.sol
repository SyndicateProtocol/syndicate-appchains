// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {BasePool} from "src/staking/BasePool.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {console2} from "forge-std/console2.sol";

contract StakingForkTest is Test {
    SyndStaking public staking = SyndStaking(0x0000000000000000000000000000000000000000);
    BasePool public basePool = BasePool(0x0000000000000000000000000000000000000000);
    address public admin = 0x0000000000000000000000000000000000000000;

    address public user1 = makeAddr("user1");
    address public user2 = makeAddr("user2");
    address public user3 = makeAddr("user3");
    address public user4 = makeAddr("user4");
    address public user5 = makeAddr("user5");
    address public user6 = makeAddr("user6");
    address public user7 = makeAddr("user7");
    address public user8 = makeAddr("user8");
    address public user9 = makeAddr("user9");
    address public user10 = makeAddr("user10");

    uint256 public appchainId = 111;

    address[] public users = [user1, user2, user3, user4, user5, user6, user7, user8, user9, user10];

    function setUp() public {
        // Start fork
        vm.createSelectFork("https://commons.rpc.syndicate.io");
        
        if (address(staking) == address(0) || address(basePool) == address(0)) {
            admin = makeAddr("admin");
            console2.log("Staking contracts not found, deploying ones to fork");
            staking = new SyndStaking(admin);
            basePool = new BasePool(address(staking));

            for (uint256 i = 0; i < users.length; i++) {
                vm.deal(users[i], 100 ether);
            }
        }
    }

    function stepEpoch(uint256 epochsToStep) public {
        vm.warp(block.timestamp + epochsToStep * 30 days);
    }

    function stake(address user, uint256 amount) public {
        vm.startPrank(user);
        staking.stakeSynd{value: amount}(appchainId);
        vm.stopPrank();
    }

    function checkStake(address user, uint256 epoch, uint256 amount) public {
        assertEq(staking.getUserStake(epoch, user), amount);
    }

    function initWithdrawal(address user) public {
        vm.startPrank(user);
        staking.initializeWithdrawal(appchainId);
        vm.stopPrank();
    }

    function withdraw(address user, uint256 epoch) public {
        vm.startPrank(user);
        staking.withdraw(epoch, user);
        vm.stopPrank();
    }

    function test_stake() public {
        uint256 currentEpoch = staking.getCurrentEpoch();

        for (uint256 i = 0; i < users.length; i++) {
            stake(users[i], (i + 1) * 1 ether);
        }

        stepEpoch(1);
        currentEpoch++;

        for (uint256 i = 0; i < users.length; i++) {
            checkStake(users[i], currentEpoch, (i + 1) * 1 ether);
        }
    }
}