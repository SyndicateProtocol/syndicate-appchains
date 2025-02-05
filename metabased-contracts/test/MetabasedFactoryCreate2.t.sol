// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {MetabasedFactoryCreate2} from "src/MetabasedFactoryCreate2.sol";

contract MetabasedFactoryCreate2Test is Test {
    MetabasedFactoryCreate2 public factory;
    address public admin;
    address public manager;
    uint256 public l3ChainId = 10042001;

    function setUp() public {
        admin = address(0x1);
        manager = address(0x2);
        factory = new MetabasedFactoryCreate2();
    }

    function testCreateSequencerChainInitable() public {
        bytes32 salt = bytes32(l3ChainId);
        address sequencerChainAddress = factory.createSequencerChainInitable(salt, l3ChainId, admin, manager);
        assertEq(sequencerChainAddress, factory.computeSequencerChainAddress(salt, l3ChainId));
    }
}
