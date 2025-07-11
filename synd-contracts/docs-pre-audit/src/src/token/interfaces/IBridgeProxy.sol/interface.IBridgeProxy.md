# IBridgeProxy
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/interfaces/IBridgeProxy.sol)

**Author:**
Syndicate Protocol

Generic interface for all bridge proxy implementations

*This interface provides a standardized way for the SyndicateToken contract
to interact with different bridge providers without knowing their specific
implementation details.*

**Note:**
security-contact: security@syndicate.io


## Functions
### executeBridge

Execute a bridge operation using the configured bridge target and parameters

*This function should handle all bridge-specific logic internally, including
token approvals, fee calculations, and the actual bridge call.
Requirements:
- Only authorized callers should be able to execute bridge operations
- The function should revert if the bridge operation fails
- Token approvals should be handled safely*


```solidity
function executeBridge(address token, uint256 amount, bytes calldata dynamicData) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`token`|`address`|The token contract address to bridge|
|`amount`|`uint256`|The amount of tokens to bridge|
|`dynamicData`|`bytes`|Additional bridge-specific data (ABI-encoded parameters) This can include recipient addresses, gas limits, chain IDs, etc.|


### getBridgeInfo

Get information about this bridge proxy

*Useful for monitoring and debugging bridge configurations*


```solidity
function getBridgeInfo() external view returns (string memory name, address target, bool active);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`name`|`string`|Human-readable name of the bridge (e.g., "Optimism Bridge")|
|`target`|`address`|The actual bridge contract address that this proxy calls|
|`active`|`bool`|Whether this bridge is currently active and accepting transactions|


