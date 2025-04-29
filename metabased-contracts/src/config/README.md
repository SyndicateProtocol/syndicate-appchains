# Chain Config Contract Upgrade Path

This document outlines the upgrade path for the ArbChainConfig contract, which is a critical component that stores configuration parameters for the Metabased protocol.

## Overview

The ArbChainConfig contract follows the proxy pattern, which allows for upgrading the implementation contract without changing the contract address that users interact with. This is achieved through the following components:

1. **Implementation Contract**: The contract that contains the actual logic and storage layout.
2. **Proxy Contract**: The contract that delegates calls to the implementation contract.
3. **ConfigManager**: A contract that manages the deployment and upgrading of the ArbChainConfig contract.

## Steps for Upgrading the ArbChainConfig Contract

When making changes to the ArbChainConfig contract, follow these steps:

1. **Develop and Test the New Implementation**:
   - Implement the required changes in the ArbChainConfig contract.
   - Write comprehensive tests to ensure the new implementation works as expected.
   - Run all existing tests to ensure backward compatibility.

2. **Deploy the New Implementation**:
   - Deploy the new implementation contract to the target network.
   - The new implementation should maintain the same storage layout to ensure data compatibility.

3. **Update the Proxy**:
   - Use the ConfigManager's upgrade function to point the proxy to the new implementation.
   - This step should be performed by the contract owner (typically the rollup owner).

4. **Verify the Upgrade**:
   - Verify that the proxy is now pointing to the new implementation.
   - Run integration tests to ensure the system works correctly with the new implementation.

## Storage Layout Considerations

When upgrading the ArbChainConfig contract, it's crucial to maintain the same storage layout to ensure data compatibility. This means:

1. **Do Not Remove Existing Storage Variables**: Removing storage variables can cause storage collisions and data corruption.
2. **Only Add New Variables at the End**: New storage variables should be added at the end of the contract to avoid shifting the storage slots of existing variables.
3. **Do Not Change Variable Types**: Changing the type of a storage variable can lead to data corruption.

## Recent Changes

Recent changes to the ArbChainConfig contract include:

- Added `SEQUENCING_CHAIN_ID` to support multiple sequencing chains.
- Added `APPCHAIN_BLOCK_EXPLORER_URL` to provide block explorer information.
- Added `ALLOWED_SETTLEMENT_ADDRESSES` to specify addresses allowed for settlement operations.

These changes follow the upgrade path described above, ensuring backward compatibility and minimal disruption to the system.
