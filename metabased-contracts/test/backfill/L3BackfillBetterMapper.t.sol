// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {L3BackfillBetterMapper} from "../../src/backfill/L3BackfillBetterMapper.sol";

contract L3BackfillBetterMapperTest is Test {
    address public manager;
    L3BackfillBetterMapper public mapper;

    function setUp() public {
        mapper = new L3BackfillBetterMapper();
    }

    function test_addInterval() public {
        uint256 gasStart = gasleft();
        mapper.addInterval(100, 1);
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for addInterval:", gasUsed);
    }

    function test_addIntervalAgain() public {
        uint256 gasStart = gasleft();
        mapper.addInterval(1001, 1);
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for addInterval:", gasUsed);
    }

    function test_addIntervalAgainAgain() public {
        uint256 gasStart = gasleft();
        mapper.addInterval(10001, 1);
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for addInterval:", gasUsed);
    }
}
