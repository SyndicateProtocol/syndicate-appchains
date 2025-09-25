#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');

// Only track upgradeable contracts
const upgradeableContracts = [
  'SyndicateFactory',
  'SyndicateSequencingChain',
  'ArbChainConfig',
  'GasAggregator'
];

function getStorageLayoutFromForge(contractName) {
  try {
    // Get storage layout as JSON from forge
    const output = execSync(`forge inspect ${contractName} storageLayout --json`, { encoding: 'utf8' });
    const layout = JSON.parse(output);

    // Transform to our expected format
    return {
      storage: layout.storage || [],
      types: layout.types || {},
      layoutVersion: "1.2",
      contractPath: `src/${contractName}.sol`, // Will be updated if needed
      contractKind: "contract"
    };
  } catch (error) {
    console.error(`Failed to get storage layout for ${contractName}:`, error.message);
    return null;
  }
}

function main() {
  const result = {};

  for (const contractName of upgradeableContracts) {
    console.error(`Extracting storage layout for ${contractName}...`);
    const layout = getStorageLayoutFromForge(contractName);
    if (layout) {
      result[contractName] = layout;
      console.error(`✅ ${contractName}: ${layout.storage.length} storage slots`);
    } else {
      console.error(`❌ ${contractName}: failed to extract`);
    }
  }

  // Output JSON to stdout
  console.log(JSON.stringify(result, null, 2));
}

main();