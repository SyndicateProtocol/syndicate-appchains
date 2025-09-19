# Contract Upgrade Safety Checklist

This checklist ensures safe upgrades of UUPS-based contracts in the Syndicate system, particularly `SyndicateFactory` and `SyndicateSequencingChain`.

## Storage Layout Safety

### Storage Variables - CRITICAL
**These changes CAN break upgrades:**

- [ ] **DO NOT** reorder existing storage variables
- [ ] **DO NOT** change the type of existing storage variables
- [ ] **DO NOT** remove existing storage variables
- [ ] **DO NOT** insert new variables between existing ones

**Safe storage practices:**
- [ ] New storage variables can be added **at the end** of the contract
- [ ] Use namespaced storage (ERC-7201) for new variables when possible
- [ ] Run storage layout tests before and after changes

### Safe Elements - NO STORAGE IMPACT
**These changes are SAFE and do not affect storage:**

- [ ] **Events** - Can be added, modified, or removed freely
- [ ] **Error definitions** - Can be added, modified, or removed freely  
- [ ] **Modifiers** - Can be added, modified, or removed freely
- [ ] **Functions** - Can be added, modified, or removed freely
- [ ] **Constants** - Do not use storage slots, safe to modify
- [ ] **Immutable variables** - Set in constructor, safe to add/modify
- [ ] **View/Pure functions** - No state changes, completely safe

## Pre-Upgrade Testing

### Storage Compatibility Tests
- [ ] Run `forge inspect ContractName storageLayout` before changes
- [ ] Run `forge inspect ContractName storageLayout` after changes
- [ ] Compare storage layouts to ensure no conflicts
- [ ] Run upgrade tests: `forge test --match-contract StorageUpgradeTest`
- [ ] Test storage preservation with actual proxy upgrade

### Functional Testing  
- [ ] All existing tests pass: `forge test`
- [ ] New functionality tests added and passing
- [ ] Integration tests verify end-to-end functionality
- [ ] Gas usage analysis for new features

## Implementation Requirements

### New Implementation Contract
- [ ] Implementation contract properly inherits from base contracts
- [ ] Constructor calls `_disableInitializers()`
- [ ] No constructor parameters (use initializer instead)
- [ ] `_authorizeUpgrade()` properly restricts upgrade access
- [ ] Version tracking updated if applicable

### Factory Integration (SyndicateSequencingChain)
- [ ] New implementation added to `allowedImplementations` list
- [ ] Factory admin approval for new implementation
- [ ] Decision: Make new implementation the default?
- [ ] Gas tracking compatibility verified

## Upgrade Process

### Pre-Deployment
- [ ] Deploy new implementation contract
- [ ] Verify implementation contract on block explorer
- [ ] Test upgrade on testnet first
- [ ] Prepare upgrade transaction data

### Factory Upgrade (SyndicateFactory)
```solidity
// Admin calls upgradeToAndCall on proxy
factory.upgradeToAndCall(newImplementation, "");
```
- [ ] Verify admin has `DEFAULT_ADMIN_ROLE`
- [ ] Test upgrade call in simulation first
- [ ] Monitor upgrade transaction confirmation

### Sequencing Chain Upgrade
```solidity  
// Owner calls upgradeToAndCall on chain
sequencingChain.upgradeToAndCall(newImplementation, "");
```
- [ ] Verify caller is chain owner
- [ ] Implementation must be in factory's allowed list
- [ ] Chain will be banned from gas tracking if implementation not allowed

## Storage Layout Examples

### Traditional Storage (Current Pattern)
```solidity
contract SyndicateSequencingChain {
    // ❌ NEVER change order of these variables
    uint256 public appchainId;           // Slot 0
    address public emissionsReceiver;    // Slot 1  
    address public factory;             // Slot 2
    bool public allowGasTrackingBanOnUpgrade; // Slot 3
    
    // ✅ New variables can be added here
    uint256 public newVariable; // Slot 4 - SAFE
}
```

### Namespaced Storage (Recommended for New Variables)
```solidity
/// @custom:storage-location erc7201:my.new.namespace
struct MyNewStorage {
    uint256 newField1;
    bool newField2;
    mapping(address => uint256) newMapping;
}

contract UpgradedContract {
    // Deterministic storage location
    bytes32 constant MY_STORAGE_LOCATION = 0x...;
    
    function _getMyStorage() private pure returns (MyNewStorage storage $) {
        assembly { $.slot := MY_STORAGE_LOCATION }
    }
}
```

## Common Pitfalls

### DANGEROUS - Will Break Upgrades
```solidity
// Before upgrade
uint256 public appchainId;
address public factory;

// After upgrade - DON'T DO THIS
address public factory;        // DANGEROUS: Reordered!
uint256 public appchainId;     // DANGEROUS: Will corrupt storage
```

### DANGEROUS - Type Changes
```solidity
// Before
uint256 public myValue;

// After - DON'T DO THIS  
uint128 public myValue;  // DANGEROUS: Type change will corrupt storage
```

### SAFE - Adding New Variables
```solidity
// Before
uint256 public existingVar;

// After - This is SAFE
uint256 public existingVar;
uint256 public newVar;     // SAFE: Added at end
```

## Verification Steps

### Post-Upgrade Validation
- [ ] Verify storage values preserved (appchainId, factory, etc.)
- [ ] Test all existing functionality still works
- [ ] Test new functionality works as expected
- [ ] Verify events are emitted correctly
- [ ] Check gas usage patterns
- [ ] Validate access control still functions

### Monitoring
- [ ] Monitor contract behavior for 24-48 hours
- [ ] Watch for unexpected reverts or errors
- [ ] Verify gas tracking continues working
- [ ] Monitor emission calculations if applicable

## Reference Documentation

- [OpenZeppelin UUPS Guide](https://docs.openzeppelin.com/upgrades-plugins/1.x/proxies#uups-proxies)
- [ERC-7201 Namespaced Storage](https://eips.ethereum.org/EIPS/eip-7201)
- [Storage Layout Tests](./test/upgrade/StorageUpgradeTest.t.sol)
- [Factory Implementation Management](./src/factory/SyndicateFactory.sol#L463-L500)

## Emergency Procedures

### If Upgrade Fails
- [ ] Do not panic - proxy is still on old implementation
- [ ] Investigate failure reason from transaction logs
- [ ] Fix implementation contract issues
- [ ] Deploy new fixed implementation
- [ ] Retry upgrade with fixed implementation

### If Storage Corruption Detected
- [ ] **STOP** - Do not perform any more state changes
- [ ] Assess extent of corruption
- [ ] If possible, upgrade to implementation that fixes corruption
- [ ] Consider emergency pause mechanisms if available
- [ ] Contact development team immediately

## Final Checklist

Before any upgrade:
- [ ] All storage layout checks passed
- [ ] All tests passing on new implementation
- [ ] Testnet upgrade successful
- [ ] Implementation contract deployed and verified
- [ ] Upgrade transaction prepared and tested
- [ ] Monitoring systems ready
- [ ] Emergency procedures reviewed
- [ ] Team notification sent

**Remember: Upgrades are irreversible. When in doubt, don't upgrade.**