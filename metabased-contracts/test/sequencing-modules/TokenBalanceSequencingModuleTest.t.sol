// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {TokenBalanceSequencingModule, IERC20} from "src/sequencing-modules/TokenBalanceSequencingModule.sol";

contract MockERC20 is IERC20 {
    mapping(address => uint256) private balances;

    function mint(address to, uint256 amount) public {
        balances[to] += amount;
    }

    function balanceOf(address account) external view override returns (uint256) {
        return balances[account];
    }
}

contract TokenBalanceSequencingModuleTest is Test {
    MockERC20 token;
    TokenBalanceSequencingModule tokenBalanceSequencer;
    address user1 = address(0x2);
    address user2 = address(0x3);
    uint256 minimumBalance = 100 * 10 ** 18;

    function setUp() public {
        token = new MockERC20();
        tokenBalanceSequencer = new TokenBalanceSequencingModule(address(token), minimumBalance);
    }

    function testIsAllowedWhenBalanceIsSufficient() public {
        token.mint(user1, minimumBalance);
        vm.prank(user1);
        assertTrue(tokenBalanceSequencer.isAllowed(user1));
    }

    function testIsNotAllowedWhenBalanceIsInsufficient() public {
        token.mint(user1, minimumBalance - 1);
        vm.prank(user1);
        assertFalse(tokenBalanceSequencer.isAllowed(user1));
    }

    function testIsAllowedWhenBalanceEqualsMinimum() public {
        token.mint(user1, minimumBalance);
        vm.prank(user1);
        assertTrue(tokenBalanceSequencer.isAllowed(user1));
    }

    function testIsNotAllowedWhenBalanceIsZero() public {
        vm.prank(user1);
        assertFalse(tokenBalanceSequencer.isAllowed(user1));
    }
}
