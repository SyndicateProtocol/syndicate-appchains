#!/usr/bin/env node
/**
 * CSV to Solidity Array Converter for Airdrop Script
 *
 * This script converts a CSV file with address,amount pairs into Solidity array format
 * that can be copy-pasted into the AirdropScript.s.sol file.
 *
 * Usage:
 *     node script/airdrop/process_csv.js script/airdrop/airdrop_data.csv
 *
 * CSV Format:
 *     address,amount
 *     0x1234...,1000000000000000000
 *     0x5678...,2000000000000000000
 *
 * Output:
 *     Solidity code that can be copied into the loadAirdropData() function
 */

const fs = require('fs');
const path = require('path');

function isValidAddress(address) {
    return typeof address === 'string' && 
           address.startsWith('0x') && 
           address.length === 42 &&
           /^0x[a-fA-F0-9]{40}$/.test(address);
}

function isValidAmount(amount) {
    try {
        const num = BigInt(amount);
        return num > 0n;
    } catch {
        return false;
    }
}

function parseCsv(content) {
    const lines = content.trim().split('\n');
    
    if (lines.length < 2) {
        throw new Error('CSV must have at least a header and one data row');
    }
    
    const header = lines[0].toLowerCase().split(',').map(h => h.trim());
    const addressIndex = header.indexOf('address');
    const amountIndex = header.indexOf('amount');
    
    if (addressIndex === -1 || amountIndex === -1) {
        throw new Error("CSV must have 'address' and 'amount' columns");
    }
    
    const data = [];
    
    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue; // Skip empty lines
        
        const columns = line.split(',').map(col => col.trim());
        
        if (columns.length < Math.max(addressIndex, amountIndex) + 1) {
            throw new Error(`Row ${i + 1}: Not enough columns`);
        }
        
        const address = columns[addressIndex];
        const amount = columns[amountIndex];
        
        if (!isValidAddress(address)) {
            throw new Error(`Row ${i + 1}: Invalid address format: ${address}`);
        }
        
        if (!isValidAmount(amount)) {
            throw new Error(`Row ${i + 1}: Invalid amount format: ${amount}`);
        }
        
        data.push({ address, amount });
    }
    
    return data;
}

function generateSolidityCode(data, filename) {
    const totalAmount = data.reduce((sum, item) => sum + BigInt(item.amount), 0n);
    
    console.log(`// Generated from ${filename}`);
    console.log(`// Total recipients: ${data.length}`);
    console.log('');
    console.log('function loadAirdropData() internal pure returns (AirdropData[] memory) {');
    console.log(`    AirdropData[] memory data = new AirdropData[](${data.length});`);
    console.log('');
    
    data.forEach((item, i) => {
        console.log(`    data[${i}] = AirdropData({`);
        console.log(`        recipient: ${item.address},`);
        console.log(`        amount: ${item.amount}`);
        console.log('    });');
        if (i < data.length - 1) {
            console.log('');
        }
    });
    
    console.log('');
    console.log('    return data;');
    console.log('}');
    console.log('');
    
    // Calculate total amount info
    console.log(`// Total amount to distribute: ${totalAmount.toString()} wei`);
    console.log(`// In ETH (assuming 18 decimals): ${(Number(totalAmount) / 1e18).toFixed(4)}`);
    console.log(`// Recipients: ${data.length}`);
}

function main() {
    if (process.argv.length !== 3) {
        console.log('Usage: node script/airdrop/process_csv.js <csv_file>');
        console.log('Example: node script/airdrop/process_csv.js script/airdrop/airdrop_data.csv');
        process.exit(1);
    }
    
    const filepath = process.argv[2];
    const filename = path.basename(filepath);
    
    if (!fs.existsSync(filepath)) {
        console.error(`Error: File ${filepath} not found`);
        process.exit(1);
    }
    
    try {
        const content = fs.readFileSync(filepath, 'utf8');
        const data = parseCsv(content);
        
        if (data.length === 0) {
            console.error('Error: No valid data found in CSV');
            process.exit(1);
        }
        
        generateSolidityCode(data, filename);
        
    } catch (error) {
        console.error(`Error: ${error.message}`);
        process.exit(1);
    }
}

if (require.main === module) {
    main();
}

module.exports = { parseCsv, generateSolidityCode, isValidAddress, isValidAmount };