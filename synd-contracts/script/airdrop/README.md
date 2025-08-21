# Syndicate Airdrop Scripts

This folder contains all the tools needed for executing token airdrops using the Syndicate Airdrop contract.

## Files Structure

```
script/airdrop/
├── AirdropScript.s.sol     # Main Solidity script for executing airdrops
├── process_csv.js          # JavaScript utility to convert CSV → Solidity
├── airdrop_data.csv        # Example CSV format
└── README.md              # This file
```

## Quick Start

### 1. Prepare Your CSV Data

Export from Google Sheets in this format:
```csv
address,amount
0x1234567890123456789012345678901234567890,1000000000000000000
0x2345678901234567890123456789012345678901,2000000000000000000
```

**Note**: Amounts must be in wei (for 18-decimal tokens: 1 token = 1000000000000000000 wei)

### 2. Generate Solidity Code

```bash
# From project root
node script/airdrop/process_csv.js script/airdrop/your_data.csv
```

This outputs Solidity code that you copy into the `loadAirdropData()` function in `AirdropScript.s.sol`.

### 3. Deploy Airdrop Contract

```bash
forge script script/airdrop/AirdropScript.s.sol:DeployAirdrop \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  --broadcast
```

### 4. Configure & Execute

1. Update contract addresses in `AirdropScript.s.sol`
2. Approve the Airdrop contract to spend your tokens
3. Execute the airdrop:

```bash
forge script script/airdrop/AirdropScript.s.sol:ExecuteAirdrop \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  --broadcast
```

## Features

- **Batch Processing**: Automatically splits large lists into gas-efficient batches
- **Safety Validations**: Checks addresses, amounts, and allowances
- **Progress Tracking**: Shows batch progress during execution
- **Gas Optimized**: Uses the assembly-optimized Airdrop contract

## JavaScript Processor Features

- **Robust CSV Parsing**: Handles various CSV formats and edge cases
- **Address Validation**: Ensures all addresses are valid Ethereum addresses
- **Amount Validation**: Checks amounts are positive integers
- **BigInt Support**: Handles large token amounts without precision loss
- **Error Reporting**: Clear error messages with row numbers for debugging

## Testing

Test with the included example data:
```bash
node script/airdrop/process_csv.js script/airdrop/airdrop_data.csv
```

This should output valid Solidity code that you can inspect before using with real data.