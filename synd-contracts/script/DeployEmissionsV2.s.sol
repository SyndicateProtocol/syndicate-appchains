// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {EmissionsCalculator} from "src/token/EmissionsCalculator.sol";
import {SyndicateTokenEmissionSchedulerV2} from "src/token/SyndicateTokenEmissionSchedulerV2.sol";

/**
 * @title Deploy Emissions V2 System
 * @notice Deploy the new piece-wise geometric decay emissions system
 */
contract DeployEmissionsV2 is Script {
    function run() public {
        vm.startBroadcast();

        address admin = vm.envAddress("ADMIN_ADDR");
        address treasury = vm.envAddress("MANAGER_ADDR");

        require(admin != address(0), "ADMIN_ADDR not set");
        require(treasury != address(0), "MANAGER_ADDR not set");

        console.log("Deploying Emissions V2 System...");
        console.log("Admin:", admin);
        console.log("Treasury:", treasury);

        // 1. Deploy SyndicateToken (if not already deployed)
        SyndicateToken token = new SyndicateToken(admin, treasury);
        console.log("SyndicateToken deployed at:", address(token));

        // 2. Deploy EmissionsCalculator
        EmissionsCalculator calculator = new EmissionsCalculator(
            address(token),
            admin,      // admin
            admin       // decay manager (same as admin for simplicity)
        );
        console.log("EmissionsCalculator deployed at:", address(calculator));

        // 3. Deploy EmissionSchedulerV2
        SyndicateTokenEmissionSchedulerV2 scheduler = new SyndicateTokenEmissionSchedulerV2(
            address(calculator),
            admin,      // admin
            admin,      // emissions manager
            admin       // pauser
        );
        console.log("EmissionSchedulerV2 deployed at:", address(scheduler));

        // 4. Setup permissions
        console.log("Setting up permissions...");
        
        // Grant emission minter role to calculator
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(calculator));
        
        // Grant emissions role to scheduler
        calculator.grantRole(calculator.EMISSIONS_ROLE(), address(scheduler));

        console.log("Permissions configured successfully");

        vm.stopBroadcast();
    }
}

/**
 * @title Initialize Emissions V2
 * @notice Initialize the emissions system with default decay factor
 */
contract InitializeEmissionsV2 is Script {
    function run() public {
        vm.startBroadcast();

        address calculatorAddr = vm.envAddress("CALCULATOR_ADDR");
        require(calculatorAddr != address(0), "CALCULATOR_ADDR not set");

        EmissionsCalculator calculator = EmissionsCalculator(calculatorAddr);

        // Initialize with a reasonable default decay factor (95%)
        uint256 defaultDecay = 0.95e18;
        console.log("Initializing emissions with default decay factor:", defaultDecay);

        calculator.initializeEmissions(defaultDecay);

        console.log("Emissions initialized successfully");
        console.log("Total epochs:", calculator.TOTAL_EPOCHS());
        console.log("Emissions cap:", calculator.EMISSIONS_CAP());

        vm.stopBroadcast();
    }
}

/**
 * @title Demo Decay Factor Updates
 * @notice Demonstrate how to update decay factors for different epochs
 */
contract DemoDecayUpdates is Script {
    function run() public view {
        address calculatorAddr = vm.envAddress("CALCULATOR_ADDR");
        require(calculatorAddr != address(0), "CALCULATOR_ADDR not set");

        EmissionsCalculator calculator = EmissionsCalculator(calculatorAddr);

        console.log("=== Emissions Calculator Demo ===");
        console.log("Calculator address:", calculatorAddr);
        console.log("Current epoch:", calculator.currentEpoch());
        console.log("Total emitted:", calculator.totalEmitted());
        console.log("Remaining supply:", calculator.getRemainingSupply());
        console.log("Is completed:", calculator.isCompleted());

        // Preview current emission
        uint256 preview = calculator.previewCurrentEmission();
        console.log("Next emission preview:", preview);

        // Show cumulative product calculation
        uint256 product = calculator.calculateCumulativeProduct(0);
        console.log("Cumulative product P_0:", product);

        console.log("=== Example Decay Factor Updates ===");
        console.log("To set epoch 10 to 90% decay:");
        console.log("calculator.setDecayFactor(10, 0.90e18)");
        
        console.log("To set epochs 20-25 with varying decay:");
        console.log("uint256[] memory decays = [0.85e18, 0.80e18, 0.75e18, 0.70e18, 0.65e18, 0.60e18];");
        console.log("calculator.setDecayFactors(20, decays);");
    }
}

/**
 * @title Simulate Emissions
 * @notice Simulate the emission process (read-only)
 */
contract SimulateEmissions is Script {
    function run() public view {
        address calculatorAddr = vm.envAddress("CALCULATOR_ADDR");
        require(calculatorAddr != address(0), "CALCULATOR_ADDR not set");

        EmissionsCalculator calculator = EmissionsCalculator(calculatorAddr);

        if (!calculator.initialized()) {
            console.log("Calculator not initialized yet");
            return;
        }

        console.log("=== Emission Simulation ===");
        console.log("Simulating first 10 epochs...");

        uint256 currentEpoch = calculator.currentEpoch();
        uint256 totalSimulated = 0;

        for (uint256 i = currentEpoch; i < currentEpoch + 10 && i < 48; i++) {
            // Calculate what the emission would be for epoch i
            // Note: This is a simulation - we can't actually call calculateAndMintEmission in a view function
            
            uint256 remainingSupply = calculator.getRemainingSupply();
            if (remainingSupply == 0) break;

            // Get decay factor for this epoch
            uint256 decayFactor = calculator.getDecayFactor(i);
            
            console.log("Epoch", i, "- Decay factor:", decayFactor);
            console.log("  Remaining supply:", remainingSupply);

            if (i == 47) {
                // Final epoch sweeps remainder
                console.log("  Final epoch - would sweep:", remainingSupply);
                totalSimulated += remainingSupply;
                break;
            } else {
                // Calculate cumulative product from this epoch
                uint256 cumulativeProduct = calculator.calculateCumulativeProduct(i);
                uint256 estimatedEmission = (remainingSupply * (1e18 - decayFactor)) / (1e18 - cumulativeProduct);
                
                console.log("  Estimated emission:", estimatedEmission);
                totalSimulated += estimatedEmission;
            }
        }

        console.log("Total simulated for next 10 epochs:", totalSimulated);
        console.log("Would leave remaining:", calculator.getRemainingSupply() - totalSimulated);
    }
}