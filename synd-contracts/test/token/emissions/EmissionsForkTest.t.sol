// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EmissionsCalculator} from "src/token/emissions/EmissionsCalculator.sol";
import {EmissionsScheduler} from "src/token/emissions/EmissionsScheduler.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {console2} from "forge-std/console2.sol";

contract MockRelayer {
    function relay(address destinationL3, uint256 epochIndex) external {}
}

contract EmissionsForkTest is Test {
    uint256 public startEpoch = 3;

    uint256 public acceptedDiff = 10;

    SyndicateToken syndToken = SyndicateToken(0x1bAB804803159aD84b8854581AA53AC72455614E);
    address public syndTokenAdmin = address(0x243c63d5DBcF619ee36Fde7fF63D1564d5665b41);
    uint256 public decayFactor = 0.98e18;
    EmissionsCalculator public emissionsCalculator = EmissionsCalculator(0x0000000000000000000000000000000000000000);
    EmissionsScheduler public emissionsScheduler = EmissionsScheduler(0x0000000000000000000000000000000000000000);

    function setUp() public {
        // Start fork
        vm.createSelectFork("https://0xrpc.io/eth");

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

            // Grant emission minter role to calculator
            bytes32 emissionMinterRole = syndToken.EMISSION_MINTER_ROLE();
            vm.prank(syndTokenAdmin);
            syndToken.grantRole(emissionMinterRole, address(emissionsCalculator));
            vm.stopPrank();

            // Initialize emissions calculator
            vm.prank(syndTokenAdmin);
            emissionsCalculator.initializeEmissions(decayFactor);
            vm.stopPrank();

            // Grant emissions role to scheduler
            bytes32 emissionsRole = emissionsCalculator.EMISSIONS_ROLE();
            vm.prank(syndTokenAdmin);
            emissionsCalculator.grantRole(emissionsRole, address(emissionsScheduler));
            vm.stopPrank();
        }
    }

    function expectedMintAmount(uint256 epoch) public pure returns (uint256) {
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

    function round(uint256 _weiAmount) public pure returns (uint256) {
        // Standard integer rounding: add half the divisor before dividing.
        return (_weiAmount + (1 ether / 2)) / 1 ether;
    }

    function test_emissions() public {
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
            uint256 expected = expectedMintAmount(i);
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
