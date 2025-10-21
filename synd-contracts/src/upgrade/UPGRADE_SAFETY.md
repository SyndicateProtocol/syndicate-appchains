# 🛡️ Upgrade Safety Guide

Learn how to safely upgrade contracts without breaking storage.

## ⚠️ Critical Rules

### 1. ALWAYS Validate Storage Layout

```bash
make storage-layout-check
```

Run this **BEFORE EVERY UPGRADE**. No exceptions.

### 2. Understand Storage Layout

Solidity stores variables in **sequential slots**:

```solidity
contract Example {
    uint256 a;      // slot 0
    address b;      // slot 1
    mapping c;      // slot 2
}
```

---

## ✅ Safe Changes

### Adding Variables at the End

**✅ SAFE:**

```solidity
// Before
contract Example {
    uint256 a;
    address b;
}

// After - Adding at end
contract Example {
    uint256 a;
    address b;
    uint256 c;  // ✅ Safe - new slot
}
```

### Adding New Functions

**✅ SAFE:**

```solidity
// Adding new functions is always safe
function newFunction() external { }
```

### Modifying Function Logic

**✅ SAFE:**

```solidity
// Changing function code is safe
function existing() external {
    // New logic here
}
```

---

## ❌ Dangerous Changes

### Removing Variables

**❌ DANGEROUS:**

```solidity
// Before
contract Example {
    uint256 a;
    address b;  // ❌ Don't remove!
    uint256 c;
}

// After
contract Example {
    uint256 a;
    uint256 c;  // ❌ Now in wrong slot!
}
```

### Reordering Variables

**❌ DANGEROUS:**

```solidity
// Before
contract Example {
    uint256 a;
    address b;
}

// After
contract Example {
    address b;  // ❌ Now in wrong slot!
    uint256 a;  // ❌ Now in wrong slot!
}
```

### Changing Variable Types

**❌ DANGEROUS:**

```solidity
// Before
contract Example {
    uint256 a;
}

// After
contract Example {
    address a;  // ❌ Wrong type, same slot!
}
```

### Inserting Variables

**❌ DANGEROUS:**

```solidity
// Before
contract Example {
    uint256 a;
    uint256 c;
}

// After
contract Example {
    uint256 a;
    uint256 b;  // ❌ Inserted, shifts everything!
    uint256 c;  // ❌ Now in wrong slot!
}
```

---

## 🔍 Storage Layout Validation

### Automated Checking

Our CI automatically checks storage layouts:

```yaml
# .github/workflows/storage-layout.yaml
- Compares against baseline
- Fails if storage changed
- Prevents accidental corruption
```

### Manual Check

```bash
make storage-layout-check
```

**Good output:**

```
✅ SyndicateFactory: Storage layout unchanged
✅ SyndicateSequencingChain: Storage layout unchanged
✅ All storage layouts are safe for upgrades
```

**Bad output:**

```
❌ STORAGE LAYOUT CHANGED: SyndicateFactory
This change could corrupt storage during upgrades!
```

### View Storage Layout

```bash
# See current layout
forge inspect SyndicateFactory storageLayout

# See specific contract
forge inspect SyndicateSequencingChain storageLayout
```

---

## 🏗️ Safe Upgrade Patterns

### Pattern 1: Add at End

```solidity
contract MyContract {
    // Existing variables
    uint256 public version;
    address public admin;

    // ✅ Add new variables here
    uint256 public newFeature;
    mapping(address => uint256) public newMapping;
}
```

### Pattern 2: Use Namespaced Storage (ERC-7201)

```solidity
// SyndicateSequencingChain uses this pattern
struct MyStorage {
    uint256 newVar;
    address newAddr;
}

function _getStorage() private pure returns (MyStorage storage $) {
    assembly {
        $.slot := keccak256("my.namespace.storage")
    }
}
```

### Pattern 3: Reserved Slots

```solidity
contract Upgradeable {
    uint256 public variable1;
    uint256 public variable2;

    // Reserve slots for future use
    uint256[50] private __gap;
}
```

---

## 🧪 Testing Upgrades

### 1. Test on Local Fork

```bash
# Terminal 1
anvil --fork-url $RPC_URL

# Terminal 2
export RPC_URL=http://localhost:8545

# Deploy
make deploy-factory
APPCHAIN_ID=1 make create-sequencing-chain

# Store some data
cast send $CHAIN_ADDRESS "someFunction()" --rpc-url $RPC_URL

# Upgrade
make upgrade-sequencing-chain

# Verify data intact
cast call $CHAIN_ADDRESS "someGetter()" --rpc-url $RPC_URL
```

### 2. Test Storage Persistence

```solidity
// Before upgrade
uint256 before = chain.someValue();

// Upgrade
chain.upgradeToAndCall(newImpl, "");

// After upgrade
uint256 after = chain.someValue();
assert(before == after);  // Storage persisted!
```

### 3. Test New Functionality

```solidity
// After upgrade
chain.newFunction();  // Should work
chain.oldFunction();  // Should still work
```

---

## 📊 Understanding Storage Slots

### How Storage Works

Each storage slot is 32 bytes (256 bits):

```
Slot 0: [32 bytes] uint256 or address (20 bytes) + padding
Slot 1: [32 bytes] next variable
Slot 2: [32 bytes] mapping location (key => slot)
```

### Packed Storage

Small variables can share slots:

```solidity
contract Packed {
    uint128 a;  // First 16 bytes of slot 0
    uint128 b;  // Last 16 bytes of slot 0
    uint256 c;  // Slot 1
}
```

### Dynamic Types

Arrays and mappings:

```solidity
contract Dynamic {
    uint256[] public array;  // Slot 0: length
                              // Elements: keccak256(slot) + index

    mapping(uint => uint) public map;  // Slot 1: empty
                                        // Values: keccak256(key, slot)
}
```

---

## 🔄 Upgrade Process Checklist

### Before Upgrade

- [ ] Run `make storage-layout-check`
- [ ] Review all storage changes
- [ ] Test on local fork
- [ ] Test on testnet
- [ ] Document changes

### During Upgrade

- [ ] Deploy new implementation
- [ ] Verify implementation on explorer
- [ ] Call `upgradeToAndCall()`
- [ ] Wait for confirmations

### After Upgrade

- [ ] Verify storage intact
- [ ] Test existing functionality
- [ ] Test new functionality
- [ ] Monitor for issues

---

## 🚨 Emergency Procedures

### If Storage Corrupted

**Prevention is key** - there's no safe recovery from storage corruption.

If it happens:

1. Pause contracts immediately
2. Assess damage extent
3. Consider redeployment with migration
4. Communicate with users

### Rollback (Only if Safe)

Rollback is only safe if:

- Old implementation still available
- No storage layout changes
- No breaking changes in data

```bash
# Redeploy old implementation
OLD_IMPL=0xPreviousImplementation

# Downgrade
cast send $FACTORY_ADDRESS \
  "upgradeToAndCall(address,bytes)" \
  $OLD_IMPL \
  0x \
  --rpc-url $RPC_URL
```

---

## 📚 Resources

- [OpenZeppelin Upgrades Guide](https://docs.openzeppelin.com/upgrades-plugins/1.x/writing-upgradeable)
- [EIP-1967: Proxy Storage Slots](https://eips.ethereum.org/EIPS/eip-1967)
- [ERC-7201: Namespaced Storage](https://eips.ethereum.org/EIPS/eip-7201)

---

## 💡 Key Takeaways

✅ **Always validate storage layouts**
✅ **Only add variables at the end**
✅ **Test thoroughly before mainnet**
✅ **Use namespaced storage for complex contracts**
✅ **Document all storage changes**

❌ **Never remove variables**
❌ **Never reorder variables**
❌ **Never change variable types**
❌ **Never insert variables in the middle**

---

Remember: **Storage corruption is not recoverable.** Always validate before upgrading!
