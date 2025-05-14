# WalletPoolWrapperModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/df30b030435a593e97b9e072bc9adc687b8fa1c4/src/sequencing-modules/WalletPoolWrapperModule.sol)

**Inherits:**
[AllowlistSequencingModule](/src/sequencing-modules/AllowlistSequencingModule.sol/contract.AllowlistSequencingModule.md)

*This contract is a wrapper for TC wallet, inheriting from the AllowlistSequencingModule.*


## Functions
### constructor

*Constructor that sets the admin address.*


```solidity
constructor(address _admin) AllowlistSequencingModule(_admin);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_admin`|`address`|The address of the admin who can modify the allowlist.|


### onlyAllowed

*Modifier to check if the caller is allowed to process transactions.*


```solidity
modifier onlyAllowed();
```

### syndicateSequencerChainNotZero

*Modifier to check if the syndicate sequencer chain address is not zero.*


```solidity
modifier syndicateSequencerChainNotZero(address _syndicateSequencerChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_syndicateSequencerChain`|`address`|The syndicate sequencer chain address|


### processTransaction

*Function to process a transaction.*


```solidity
function processTransaction(address _syndicateSequencerChain, bytes calldata data)
    external
    onlyAllowed
    syndicateSequencerChainNotZero(_syndicateSequencerChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_syndicateSequencerChain`|`address`|The syndicate sequencer chain address|
|`data`|`bytes`|The transaction data to process.|


### processTransactionRaw

*Function to process a raw transaction.*


```solidity
function processTransactionRaw(address _syndicateSequencerChain, bytes calldata data)
    external
    onlyAllowed
    syndicateSequencerChainNotZero(_syndicateSequencerChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_syndicateSequencerChain`|`address`|The syndicate sequencer chain address|
|`data`|`bytes`|The transaction data to process.|


### processBulkTransactions

*Function to process bulk transactions.*


```solidity
function processBulkTransactions(address _syndicateSequencerChain, bytes[] calldata data)
    external
    onlyAllowed
    syndicateSequencerChainNotZero(_syndicateSequencerChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_syndicateSequencerChain`|`address`|The syndicate sequencer chain address|
|`data`|`bytes[]`|The array of transaction data to process.|


## Events
### WalletPoolWrapperTransactionSent

```solidity
event WalletPoolWrapperTransactionSent(address indexed from, address indexed syndicateSequencerChain);
```

### WalletPoolWrapperBulkTransactionsSent

```solidity
event WalletPoolWrapperBulkTransactionsSent(
    address indexed from, address indexed syndicateSequencerChain, uint256 count
);
```

## Errors
### ZeroSequencerAddressNotAllowed

```solidity
error ZeroSequencerAddressNotAllowed();
```

