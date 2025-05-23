# WalletPoolWrapperModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/f93e91004eb8d04d84acd3b9cb0e8f7e6abfa528/src/sequencing-modules/WalletPoolWrapperModule.sol)

**Inherits:**
[AllowlistSequencingModule](/src/sequencing-modules/AllowlistSequencingModule.sol/contract.AllowlistSequencingModule.md)

*This contract is a wrapper for TC wallet, inheriting from the AllowlistSequencingModule.*


## Functions
### constructor

*Constructor that sets the admin address.*

*param _admin is validated in AllowlistSequencingModule constructor.*


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

### SyndicateSequencingChainNotZero

*Modifier to check if the syndicate sequencer chain address is not zero.*


```solidity
modifier SyndicateSequencingChainNotZero(address _SyndicateSequencingChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_SyndicateSequencingChain`|`address`|The syndicate sequencer chain address|


### processTransactionUncompressed

*Function to process a transaction.*


```solidity
function processTransactionUncompressed(address _SyndicateSequencingChain, bytes calldata data)
    external
    onlyAllowed
    SyndicateSequencingChainNotZero(_SyndicateSequencingChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_SyndicateSequencingChain`|`address`|The syndicate sequencer chain address|
|`data`|`bytes`|The transaction data to process.|


### processTransaction

*Function to process a raw transaction.*


```solidity
function processTransaction(address _SyndicateSequencingChain, bytes calldata data)
    external
    onlyAllowed
    SyndicateSequencingChainNotZero(_SyndicateSequencingChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_SyndicateSequencingChain`|`address`|The syndicate sequencer chain address|
|`data`|`bytes`|The transaction data to process.|


### processTransactionsBulk

*Function to process bulk transactions.*


```solidity
function processTransactionsBulk(address _SyndicateSequencingChain, bytes[] calldata data)
    external
    onlyAllowed
    SyndicateSequencingChainNotZero(_SyndicateSequencingChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_SyndicateSequencingChain`|`address`|The syndicate sequencer chain address|
|`data`|`bytes[]`|The array of transaction data to process.|


## Events
### WalletPoolWrapperTransactionSent

```solidity
event WalletPoolWrapperTransactionSent(address indexed from, address indexed SyndicateSequencingChain);
```

### WalletPoolWrapperBulkTransactionsSent

```solidity
event WalletPoolWrapperBulkTransactionsSent(
    address indexed from, address indexed SyndicateSequencingChain, uint256 count
);
```

## Errors
### ZeroSequencerAddressNotAllowed

```solidity
error ZeroSequencerAddressNotAllowed();
```

