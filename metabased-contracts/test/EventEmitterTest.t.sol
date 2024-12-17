// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {EventEmitter} from "src/EventEmitter.sol";

contract EventEmitterTest is Test {
    EventEmitter public eventEmitter;
    event Foo1( uint256 nonIndexed);
    event Foo2( uint256 indexed indexed1, uint256 nonIndexed);
    event Foo3( uint256 indexed indexed1, uint256 indexed indexed2, uint256 nonIndexed);
    event Foo4( uint256 indexed indexed1, uint256 indexed indexed2, uint256 indexed indexed3, uint256 nonIndexed);

    function setUp() public {
        eventEmitter = new EventEmitter();
    }

    function testEmitEvent1() public {
        vm.expectEmit(false, false, false, true);
        emit Foo1(123);
        eventEmitter.emitEvent1(Foo1.selector, bytes32(uint256(123)));
    }

    function testEmitEvent2() public {
        vm.expectEmit(true, false, false, true);
        emit Foo2(123, 456);
        eventEmitter.emitEvent2(Foo2.selector, bytes32(uint256(123)), bytes32(uint256(456)));
    }

    function testEmitEvent3() public {
        vm.expectEmit(true, true, false, true);
        emit Foo3(123, 456, 789);
        eventEmitter.emitEvent3(Foo3.selector, bytes32(uint256(123)), bytes32(uint256(456)), bytes32(uint256(789)));
    }

    function testEmitEvent4() public {
        vm.expectEmit(true, true, true, true);
        emit Foo4(123, 456, 789, 101112);
        eventEmitter.emitEvent4(Foo4.selector, bytes32(uint256(123)), bytes32(uint256(456)), bytes32(uint256(789)), bytes32(uint256(101112)));
    }
}
