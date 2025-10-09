// SPDX-License-Identifier: UNLICENSED
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
            uint256(1279482), // Epoch 0
            uint256(1292277), // Epoch 1
            uint256(1305200), // Epoch 2
            uint256(1318253), // Epoch 3
            uint256(1331435), // Epoch 4
            uint256(1344750), // Epoch 5
            uint256(1358197), // Epoch 6
            uint256(1371779), // Epoch 7
            uint256(1385497), // Epoch 8
            uint256(1399352), // Epoch 9
            uint256(1413345), // Epoch 10
            uint256(1427479), // Epoch 11
            uint256(1441754), // Epoch 12
            uint256(1456171), // Epoch 13
            uint256(1470732), // Epoch 14
            uint256(1485440), // Epoch 15
            uint256(1500294), // Epoch 16
            uint256(1515297), // Epoch 17
            uint256(1530451), // Epoch 18
            uint256(1545755), // Epoch 19
            uint256(1561212), // Epoch 20
            uint256(1576825), // Epoch 21
            uint256(1592594), // Epoch 22
            uint256(1608519), // Epoch 23
            uint256(1624604), // Epoch 24
            uint256(1640850), // Epoch 25
            uint256(1657259), // Epoch 26
            uint256(1673831), // Epoch 27
            uint256(1690570), // Epoch 28
            uint256(1707475), // Epoch 29
            uint256(1724550), // Epoch 30
            uint256(1741796), // Epoch 31
            uint256(1759214), // Epoch 32
            uint256(1776806), // Epoch 33
            uint256(1794574), // Epoch 34
            uint256(1812520), // Epoch 35
            uint256(1830646), // Epoch 36
            uint256(1848952), // Epoch 37
            uint256(1867441), // Epoch 38
            uint256(1886115), // Epoch 39
            uint256(1904977), // Epoch 40
            uint256(1924027), // Epoch 41
            uint256(1943267), // Epoch 42
            uint256(1962700), // Epoch 43
            uint256(1982327), // Epoch 44
            uint256(2002151), // Epoch 45
            uint256(2022173), // Epoch 46
            uint256(2042394) // Epoch 47
        ];

        if (epoch >= 48) {
            return 0;
        }

        return tokensPerEpoch[epoch];
    }

    function expectedMintAmount_ChangeFactor098(uint256 epoch) public pure returns (uint256) {
        // Token amounts per epoch based on the provided data (in wei)
        uint256[48] memory tokensPerEpoch = [
            uint256(2523566), // Epoch 0
            uint256(2473093), // Epoch 1
            uint256(2423632), // Epoch 2
            uint256(2375159), // Epoch 3
            uint256(2327656), // Epoch 4
            uint256(2281103), // Epoch 5
            uint256(2235481), // Epoch 6
            uint256(2190771), // Epoch 7
            uint256(2146956), // Epoch 8
            uint256(2104017), // Epoch 9
            uint256(2061937), // Epoch 10
            uint256(2020698), // Epoch 11
            uint256(1980284), // Epoch 12
            uint256(1940678), // Epoch 13
            uint256(1901864), // Epoch 14
            uint256(1863828), // Epoch 15
            uint256(1826551), // Epoch 16
            uint256(1790020), // Epoch 17
            uint256(1754220), // Epoch 18
            uint256(1719135), // Epoch 19
            uint256(1684752), // Epoch 20
            uint256(1651058), // Epoch 21
            uint256(1618036), // Epoch 22
            uint256(1585676), // Epoch 23
            uint256(1553961), // Epoch 24
            uint256(1522883), // Epoch 25
            uint256(1492425), // Epoch 26
            uint256(1462577), // Epoch 27
            uint256(1433325), // Epoch 28
            uint256(1404658), // Epoch 29
            uint256(1376565), // Epoch 30
            uint256(1349034), // Epoch 31
            uint256(1322054), // Epoch 32
            uint256(1295612), // Epoch 33
            uint256(1269700), // Epoch 34
            uint256(1244306), // Epoch 35
            uint256(1219420), // Epoch 36
            uint256(1195031), // Epoch 37
            uint256(1171131), // Epoch 38
            uint256(1147708), // Epoch 39
            uint256(1124755), // Epoch 40
            uint256(1102259), // Epoch 41
            uint256(1080214), // Epoch 42
            uint256(1058610), // Epoch 43
            uint256(1037437), // Epoch 44
            uint256(1016689), // Epoch 45
            uint256(996356), // Epoch 46
            uint256(976429) // Epoch 47
        ];

        if (epoch >= 48) {
            return 0;
        }

        return tokensPerEpoch[epoch];
    }

    function expectedMintAmount_ChangeFactorMultiple(uint256 epoch) public pure returns (uint256) {
        // Token amounts per epoch based on the provided data (in wei)
        uint256[48] memory tokensPerEpoch = [
            uint256(1631944), // Epoch 0
            uint256(1631944), // Epoch 1
            uint256(1631944), // Epoch 2
            uint256(1631944), // Epoch 3
            uint256(1163995), // Epoch 4
            uint256(1181454), // Epoch 5
            uint256(1199176), // Epoch 6
            uint256(1217163), // Epoch 7
            uint256(1235421), // Epoch 8
            uint256(1253953), // Epoch 9
            uint256(1272762), // Epoch 10
            uint256(1291853), // Epoch 11
            uint256(1311231), // Epoch 12
            uint256(1330899), // Epoch 13
            uint256(1350864), // Epoch 14
            uint256(1371127), // Epoch 15
            uint256(1391693), // Epoch 16
            uint256(1412569), // Epoch 17
            uint256(1433757), // Epoch 18
            uint256(1455263), // Epoch 19
            uint256(1477092), // Epoch 20
            uint256(1953519), // Epoch 21
            uint256(1943751), // Epoch 22
            uint256(1934033), // Epoch 23
            uint256(1924363), // Epoch 24
            uint256(1914740), // Epoch 25
            uint256(1905167), // Epoch 26
            uint256(1895642), // Epoch 27
            uint256(1886163), // Epoch 28
            uint256(1876732), // Epoch 29
            uint256(1867349), // Epoch 30
            uint256(1858012), // Epoch 31
            uint256(1848722), // Epoch 32
            uint256(1839479), // Epoch 33
            uint256(1830280), // Epoch 34
            uint256(1821129), // Epoch 35
            uint256(1812024), // Epoch 36
            uint256(1802964), // Epoch 37
            uint256(1793949), // Epoch 38
            uint256(1784979), // Epoch 39
            uint256(1776054), // Epoch 40
            uint256(1767174), // Epoch 41
            uint256(1758339), // Epoch 42
            uint256(1749547), // Epoch 43
            uint256(1740799), // Epoch 44
            uint256(1732095), // Epoch 45
            uint256(1723436), // Epoch 46
            uint256(1714819) // Epoch 47
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
            uint256 expected = uint256(1631944);
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
