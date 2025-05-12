# Chain Config Contract Upgrade Path

This document outlines the upgrade path for the ArbChainConfig contract, which is a critical component that stores configuration parameters for the 
<!-- @NOTE: Do we want to use "Syndicate protocol" here? -->
Syndicate protocol.

## Overview

The ArbChainConfig contract follows the proxy pattern, which allows for upgrading the implementation contract without changing the contract address that users interact with. This is achieved through the following components:

1. **Implementation Contract**: The contract that contains the actual logic and storage layout.
2. **Proxy Contract**: The contract that delegates calls to the implementation contract.
3. **ConfigManager**: A contract that manages the deployment and upgrading of the ArbChainConfig contract.

## Upgrading via the Beacon Proxy

The ArbChainConfig contract uses a beacon proxy pattern for upgrades, which allows for multiple proxy instances to be upgraded simultaneously by updating a single beacon contract. Here's how to perform an upgrade using the beacon proxy:

1. **Deploy the New Implementation Contract**:

   ```solidity
   // Deploy the new implementation
   ArbChainConfig newImplementation = new ArbChainConfig();
   ```

2. **Update the Beacon Contract via the ConfigManager**:

   ```solidity
   // Get the ConfigManager instance
   ArbConfigManager configManager = ArbConfigManager(configManagerAddress);

   // Call the upgradeArbChainConfigImplementation function
   // This function updates the beacon to point to the new implementation
   configManager.upgradeImplementation(newImplementation.address);
   ```

3. **Verify the Upgrade**:

   ```solidity
   // Get the current implementation address from the ConfigManager
   address currentImplementation = configManager.implementation();

   // Verify it matches the new implementation
   require(currentImplementation == address(newImplementation), "Upgrade failed");
   ```

4. **Important Considerations**:
   - Only the owner of the ConfigManager can perform upgrades
   - All proxy instances pointing to the beacon will be upgraded simultaneously
   - Ensure the new implementation maintains the same storage layout
   - Test the upgrade process in a test environment before deploying to production
   - Consider using a timelock mechanism for critical upgrades to provide transparency

## Storage Layout Considerations

When upgrading the ArbChainConfig contract, it's crucial to maintain the same storage layout to ensure data compatibility. This means:

1. **Do Not Remove Existing Storage Variables**: Removing storage variables can cause storage collisions and data corruption.
2. **Only Add New Variables at the End**: New storage variables should be added at the end of the contract to avoid shifting the storage slots of existing variables.
3. **Do Not Change Variable Types**: Changing the type of a storage variable can lead to data corruption.
