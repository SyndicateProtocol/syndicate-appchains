// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {EmissionsCalculator} from "src/token/emissions/EmissionsCalculator.sol";
import {EmissionsScheduler} from "src/token/emissions/EmissionsScheduler.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {console2} from "forge-std/console2.sol";
import {EpochTracker} from "src/staking/EpochTracker.sol";

contract MockRelayer {
    function relay(address destinationL3, uint256 epochIndex) external {}
}

contract EmissionsForkTest is Test, EpochTracker {
    uint256 public startEpoch;

    uint256 public acceptedDiff = 10;

    SyndicateToken syndToken = SyndicateToken(0x1bAB804803159aD84b8854581AA53AC72455614E);
    address public syndTokenAdmin = address(0x243c63d5DBcF619ee36Fde7fF63D1564d5665b41);
    EmissionsCalculator public emissionsCalculator = EmissionsCalculator(0x0000000000000000000000000000000000000000);
    EmissionsScheduler public emissionsScheduler = EmissionsScheduler(0x0000000000000000000000000000000000000000);

    // Actual deployments:
    // EmissionsCalculator public emissionsCalculator = EmissionsCalculator(0x7CC604b2e117693fE214b8253504eC29BE9Ecf0a);
    // EmissionsScheduler public emissionsScheduler = EmissionsScheduler(0xcD3602332fA70191A0e1A1b49aC9873aD4D87E0e);

    function setUp() public {
        // Start fork
        vm.createSelectFork("https://0xrpc.io/eth");

        startEpoch = getCurrentEpoch() + 1;

        if (address(emissionsCalculator) == address(0) || address(emissionsScheduler) == address(0)) {
            console2.log("Emissions contracts not found, deploying ones to fork");
            MockRelayer relayer = new MockRelayer();
            emissionsCalculator = new EmissionsCalculator(address(syndToken), syndTokenAdmin, syndTokenAdmin);
            emissionsScheduler = new EmissionsScheduler(
                startEpoch,
                address(emissionsCalculator),
                address(relayer),
                syndTokenAdmin,
                syndTokenAdmin,
                syndTokenAdmin
            );

            // Grant emissions role to scheduler
            bytes32 emissionsRole = emissionsCalculator.EMISSIONS_ROLE();
            vm.prank(syndTokenAdmin);
            emissionsCalculator.grantRole(emissionsRole, address(emissionsScheduler));
            vm.stopPrank();
        }

        // Grant emission minter role to calculator
        bytes32 emissionMinterRole = syndToken.EMISSION_MINTER_ROLE();
        vm.prank(syndTokenAdmin);
        syndToken.grantRole(emissionMinterRole, address(emissionsCalculator));
        vm.stopPrank();
    }

    function expectedMintAmount_ChangeFactor101(uint256 epoch) public pure returns (uint256) {
        // Token amounts per epoch based on the provided data (in wei)

        uint256[48] memory tokensPerEpoch = [
            uint256(1306706), // Epoch 0
            uint256(1319773), // Epoch 1
            uint256(1332971), // Epoch 2
            uint256(1346301), // Epoch 3
            uint256(1359764), // Epoch 4
            uint256(1373362), // Epoch 5
            uint256(1387095), // Epoch 6
            uint256(1400966), // Epoch 7
            uint256(1414976), // Epoch 8
            uint256(1429126), // Epoch 9
            uint256(1443417), // Epoch 10
            uint256(1457851), // Epoch 11
            uint256(1472430), // Epoch 12
            uint256(1487154), // Epoch 13
            uint256(1502025), // Epoch 14
            uint256(1517046), // Epoch 15
            uint256(1532216), // Epoch 16
            uint256(1547538), // Epoch 17
            uint256(1563014), // Epoch 18
            uint256(1578644), // Epoch 19
            uint256(1594430), // Epoch 20
            uint256(1610375), // Epoch 21
            uint256(1626479), // Epoch 22
            uint256(1642743), // Epoch 23
            uint256(1659171), // Epoch 24
            uint256(1675762), // Epoch 25
            uint256(1692520), // Epoch 26
            uint256(1709445), // Epoch 27
            uint256(1726540), // Epoch 28
            uint256(1743805), // Epoch 29
            uint256(1761243), // Epoch 30
            uint256(1778856), // Epoch 31
            uint256(1796645), // Epoch 32
            uint256(1814611), // Epoch 33
            uint256(1832757), // Epoch 34
            uint256(1851085), // Epoch 35
            uint256(1869596), // Epoch 36
            uint256(1888292), // Epoch 37
            uint256(1907174), // Epoch 38
            uint256(1926246), // Epoch 39
            uint256(1945509), // Epoch 40
            uint256(1964964), // Epoch 41
            uint256(1984614), // Epoch 42
            uint256(2004460), // Epoch 43
            uint256(2024505), // Epoch 44
            uint256(2044750), // Epoch 45
            uint256(2065198), // Epoch 46
            uint256(2085850) // Epoch 47
        ];

        if (epoch >= 48) {
            return 0;
        }

        return tokensPerEpoch[epoch];
    }

    function expectedMintAmount_ChangeFactor098(uint256 epoch) public pure returns (uint256) {
        // Token amounts per epoch based on the provided data (in wei)
        uint256[48] memory tokensPerEpoch = [
            uint256(2577259), // Epoch 0
            uint256(2525713), // Epoch 1
            uint256(2475199), // Epoch 2
            uint256(2425695), // Epoch 3
            uint256(2377181), // Epoch 4
            uint256(2329638), // Epoch 5
            uint256(2283045), // Epoch 6
            uint256(2237384), // Epoch 7
            uint256(2192636), // Epoch 8
            uint256(2148784), // Epoch 9
            uint256(2105808), // Epoch 10
            uint256(2063692), // Epoch 11
            uint256(2022418), // Epoch 12
            uint256(1981970), // Epoch 13
            uint256(1942330), // Epoch 14
            uint256(1903484), // Epoch 15
            uint256(1865414), // Epoch 16
            uint256(1828106), // Epoch 17
            uint256(1791544), // Epoch 18
            uint256(1755713), // Epoch 19
            uint256(1720598), // Epoch 20
            uint256(1686187), // Epoch 21
            uint256(1652463), // Epoch 22
            uint256(1619414), // Epoch 23
            uint256(1587025), // Epoch 24
            uint256(1555285), // Epoch 25
            uint256(1524179), // Epoch 26
            uint256(1493696), // Epoch 27
            uint256(1463822), // Epoch 28
            uint256(1434545), // Epoch 29
            uint256(1405854), // Epoch 30
            uint256(1377737), // Epoch 31
            uint256(1350183), // Epoch 32
            uint256(1323179), // Epoch 33
            uint256(1296715), // Epoch 34
            uint256(1270781), // Epoch 35
            uint256(1245366), // Epoch 36
            uint256(1220458), // Epoch 37
            uint256(1196049), // Epoch 38
            uint256(1172128), // Epoch 39
            uint256(1148686), // Epoch 40
            uint256(1125712), // Epoch 41
            uint256(1103198), // Epoch 42
            uint256(1081134), // Epoch 43
            uint256(1059511), // Epoch 44
            uint256(1038321), // Epoch 45
            uint256(1017556), // Epoch 46
            uint256(997205) // Epoch 47
        ];

        if (epoch >= 48) {
            return 0;
        }

        return tokensPerEpoch[epoch];
    }

    function expectedMintAmount_ChangeFactorMultiple(uint256 epoch) public pure returns (uint256) {
        // Token amounts per epoch based on the provided data (in wei)
        uint256[48] memory tokensPerEpoch = [
            uint256(1666667), // Epoch 0
            uint256(1666667), // Epoch 1
            uint256(1666667), // Epoch 2
            uint256(1666667), // Epoch 3
            // CHANGE FACTOR: 1.015
            uint256(1188761), // Epoch 4
            uint256(1206592), // Epoch 5
            uint256(1224691), // Epoch 6
            uint256(1243061), // Epoch 7
            uint256(1261707), // Epoch 8
            uint256(1280633), // Epoch 9
            uint256(1299843), // Epoch 10
            uint256(1319340), // Epoch 11
            uint256(1339130), // Epoch 12
            uint256(1359217), // Epoch 13
            uint256(1379606), // Epoch 14
            uint256(1400300), // Epoch 15
            uint256(1421304), // Epoch 16
            uint256(1442624), // Epoch 17
            uint256(1464263), // Epoch 18
            uint256(1486227), // Epoch 19
            uint256(1508520), // Epoch 20
            // CHANGE FACTOR: 0.995
            uint256(1995084), // Epoch 21
            uint256(1985108), // Epoch 22
            uint256(1975183), // Epoch 23
            uint256(1965307), // Epoch 24
            uint256(1955480), // Epoch 25
            uint256(1945703), // Epoch 26
            uint256(1935975), // Epoch 27
            uint256(1926295), // Epoch 28
            uint256(1916663), // Epoch 29
            uint256(1907080), // Epoch 30
            uint256(1897545), // Epoch 31
            uint256(1888057), // Epoch 32
            uint256(1878617), // Epoch 33
            uint256(1869223), // Epoch 34
            uint256(1859877), // Epoch 35
            uint256(1850578), // Epoch 36
            uint256(1841325), // Epoch 37
            uint256(1832119), // Epoch 38
            uint256(1822958), // Epoch 39
            uint256(1813843), // Epoch 40
            uint256(1804774), // Epoch 41
            uint256(1795751), // Epoch 42
            uint256(1786772), // Epoch 43
            uint256(1777838), // Epoch 44
            uint256(1768949), // Epoch 45
            uint256(1760105), // Epoch 46
            uint256(1751305) // Epoch 47
        ];

        if (epoch >= 48) {
            return 0;
        }

        return tokensPerEpoch[epoch];
    }

    function round(uint256 _weiAmount) public pure returns (uint256) {
        // Standard integer rounding: add half the divisor before dividing.
        return (_weiAmount + (1 ether / 2)) / 1 ether;
    }

    function test_emissions_ChangeFactor101() public {
        vm.skip(true);
        // Initialize emissions calculator
        vm.prank(syndTokenAdmin);
        emissionsCalculator.initializeEmissions(1.01e18);
        vm.stopPrank();
        uint256 totalMinted = emissionsScheduler.totalEmissionsMinted();

        // Initial checks
        assertEq(emissionsCalculator.currentEpoch(), 0);
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch - 1);
        assertEq(totalMinted, 0);
        assertFalse(emissionsScheduler.emissionsStarted());

        // Confirm we cant mint yet
        vm.expectRevert(EmissionsScheduler.NoEmissionsToMint.selector);
        emissionsScheduler.mintEmission();

        for (uint256 i = 0; i <= 47; i++) {
            vm.warp(emissionsScheduler.getEpochStart(i + startEpoch));
            emissionsScheduler.mintEmission();
            uint256 mintAmount = round(emissionsScheduler.totalEmissionsMinted() - totalMinted);
            uint256 expected = expectedMintAmount_ChangeFactor101(i);
            if (mintAmount != expected) {
                console2.log("Epoch %s: Expected %s, Actual %s", i, expected, mintAmount);
                if (mintAmount > expected + acceptedDiff || mintAmount < expected - acceptedDiff) {
                    revert("Mint amount is not within accepted diff");
                }
            }
            totalMinted = emissionsScheduler.totalEmissionsMinted();
        }

        // Final checks
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch + 47);
        assertEq(emissionsScheduler.totalEmissionsMinted(), totalMinted);
        assertTrue(emissionsScheduler.emissionsEnded());

        // Confirm we cant mint after all epochs are minted
        vm.expectRevert(EmissionsScheduler.AllEmissionsCompleted.selector);
        emissionsScheduler.mintEmission();
    }

    function test_emissions_ChangeFactor098() public {
        vm.skip(true);
        // Initialize emissions calculator
        vm.prank(syndTokenAdmin);
        emissionsCalculator.initializeEmissions(0.98e18);
        vm.stopPrank();

        uint256 totalMinted = emissionsScheduler.totalEmissionsMinted();

        // Initial checks
        assertEq(emissionsCalculator.currentEpoch(), 0);
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch - 1);
        assertEq(totalMinted, 0);
        assertFalse(emissionsScheduler.emissionsStarted());

        // Confirm we cant mint yet
        vm.expectRevert(EmissionsScheduler.NoEmissionsToMint.selector);
        emissionsScheduler.mintEmission();

        for (uint256 i = 0; i <= 47; i++) {
            vm.warp(emissionsScheduler.getEpochStart(i + startEpoch));
            emissionsScheduler.mintEmission();
            uint256 mintAmount = round(emissionsScheduler.totalEmissionsMinted() - totalMinted);
            uint256 expected = expectedMintAmount_ChangeFactor098(i);
            if (mintAmount != expected) {
                console2.log("Epoch %s: Expected %s, Actual %s", i, expected, mintAmount);
                if (mintAmount > expected + acceptedDiff || mintAmount < expected - acceptedDiff) {
                    revert("Mint amount is not within accepted diff");
                }
            }
            totalMinted = emissionsScheduler.totalEmissionsMinted();
        }

        // Final checks
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch + 47);
        assertEq(emissionsScheduler.totalEmissionsMinted(), totalMinted);
        assertTrue(emissionsScheduler.emissionsEnded());

        // Confirm we cant mint after all epochs are minted
        vm.expectRevert(EmissionsScheduler.AllEmissionsCompleted.selector);
        emissionsScheduler.mintEmission();
    }

    function test_emissions_ChangeFactorFlat() public {
        vm.skip(true);
        uint256 totalMinted = emissionsScheduler.totalEmissionsMinted();
        vm.prank(syndTokenAdmin);
        emissionsCalculator.initializeEmissions(1e18);
        vm.stopPrank();

        // Initial checks
        assertEq(emissionsCalculator.currentEpoch(), 0);
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch - 1);
        assertEq(totalMinted, 0);
        assertFalse(emissionsScheduler.emissionsStarted());

        // Confirm we cant mint yet
        vm.expectRevert(EmissionsScheduler.NoEmissionsToMint.selector);
        emissionsScheduler.mintEmission();

        uint256 blockNumber = block.number;
        for (uint256 i = 0; i <= 47; i++) {
            vm.warp(emissionsScheduler.getEpochStart(i + startEpoch));
            vm.roll(blockNumber + i);
            emissionsScheduler.mintEmission();
            uint256 mintAmount = round(emissionsScheduler.totalEmissionsMinted() - totalMinted);
            uint256 expected = uint256(1666667);
            if (mintAmount != expected) {
                console2.log("Epoch %s: Expected %s, Actual %s", i, expected, mintAmount);
                if (mintAmount > expected + acceptedDiff || mintAmount < expected - acceptedDiff) {
                    revert("Mint amount is not within accepted diff");
                }
            }
            totalMinted = emissionsScheduler.totalEmissionsMinted();
            console2.log("Next block number", block.number);
        }

        // Final checks
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch + 47);
        assertEq(emissionsScheduler.totalEmissionsMinted(), totalMinted);
        assertTrue(emissionsScheduler.emissionsEnded());

        // Confirm we cant mint after all epochs are minted
        vm.expectRevert(EmissionsScheduler.AllEmissionsCompleted.selector);
        emissionsScheduler.mintEmission();
    }

    function test_emissions_ChangeFactorMultiple() public {
        vm.skip(true);
        // Initialize emissions calculator
        vm.prank(syndTokenAdmin);
        emissionsCalculator.initializeEmissions(1e18);
        vm.stopPrank();

        uint256 totalMinted = emissionsScheduler.totalEmissionsMinted();

        // Initial checks
        assertEq(emissionsCalculator.currentEpoch(), 0);
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch - 1);
        assertEq(totalMinted, 0);
        assertFalse(emissionsScheduler.emissionsStarted());

        // Confirm we cant mint yet
        vm.expectRevert(EmissionsScheduler.NoEmissionsToMint.selector);
        emissionsScheduler.mintEmission();

        for (uint256 i = 0; i <= 47; i++) {
            if (i == 4) {
                vm.prank(syndTokenAdmin);
                emissionsCalculator.setChangeFactor(1.015e18);
            }
            if (i == 21) {
                vm.prank(syndTokenAdmin);
                emissionsCalculator.setChangeFactor(0.995e18);
            }
            vm.warp(emissionsScheduler.getEpochStart(i + startEpoch));
            emissionsScheduler.mintEmission();
            uint256 mintAmount = round(emissionsScheduler.totalEmissionsMinted() - totalMinted);
            uint256 expected = expectedMintAmount_ChangeFactorMultiple(i);
            if (mintAmount != expected) {
                console2.log("Epoch %s: Expected %s, Actual %s", i, expected, mintAmount);
                if (mintAmount > expected + acceptedDiff || mintAmount < expected - acceptedDiff) {
                    revert("Mint amount is not within accepted diff");
                }
            }
            totalMinted = emissionsScheduler.totalEmissionsMinted();
        }

        // Final checks
        assertEq(emissionsScheduler.getCurrentEpoch(), startEpoch + 47);
        assertEq(emissionsScheduler.totalEmissionsMinted(), totalMinted);
        assertTrue(emissionsScheduler.emissionsEnded());

        // Confirm we cant mint after all epochs are minted
        vm.expectRevert(EmissionsScheduler.AllEmissionsCompleted.selector);
        emissionsScheduler.mintEmission();
    }
}
