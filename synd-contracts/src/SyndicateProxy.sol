// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

// solhint-disable private-vars-leading-underscore
contract SyndicateProxy {
    address private immutable admin;

    // keccak256("eip1967.proxy.implementation") - 1
    bytes32 private constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    // keccak256("syndicate.proxy.storage") - 1
    // stores gasUsed | prevGasUsedHighBits | epoch packed together into a single storage slot
    // gasUsed is 128 bits in size, prevGasUsedHighBits is 112 bits, and epoch is 16 bits. As the name
    // suggests, prevGasUsedHighBits is missing the last 16 low bits of the uint128 value which are set
    // to 0 when returning a uint128 or uint256 value. This corresponds to a granularity of 0.000066 gwei.
    bytes32 private constant STORAGE_SLOT = 0xd877d5a24209b8677258dc4c4521b9bd8c2e2ce630415c4e6e6d7ed4760d9821;

    // epoch constants. note that START_TIMESTAMP is the timestamp of epoch 0, not epoch 1.
    // uint constants are padded from the left with leading zeroes
    uint32 private constant START_TIMESTAMP = 1751497200;
    uint32 private constant EPOCH_DURATION = 30 days;
    uint16 private constant EPOCH_MASK = 0xffff;
    uint128 private constant PREV_TOKENS_MASK = 0xffffffffffffffffffffffffffff0000;

    // bytes constants are padded from the right with trailing zeroes
    bytes4 private constant REVERT_IF_STATIC_CALL_SELECTOR = 0x776922c7;

    // get full gas tracking info, for debug purposes
    function gasInfo() external view returns (uint16 epoch, uint128 prevTokens, uint128 tokens) {
        assembly {
            let data := sload(STORAGE_SLOT)
            epoch := and(data, EPOCH_MASK)
            prevTokens := and(data, PREV_TOKENS_MASK)
            tokens := shr(128, data)
        }
    }

    // note that although this function returns a uint256 for compatibility purposes,
    // the return value will never exceed type(uint128).max
    function tokensUsedPerEpoch(uint256 epoch) external view returns (uint256 tokens) {
        assembly {
            let data := sload(STORAGE_SLOT)
            switch sub(and(data, EPOCH_MASK), epoch)
            case 0 { tokens := shl(16, shr(144, data)) }
            case 1 { tokens := and(data, PREV_TOKENS_MASK) }
        }
    }

    function initializeProxy(address implementation, uint128 prevTokens, uint128 tokens) external {
        assembly {
            if gt(sload(STORAGE_SLOT), 0) { revert(0, 0) }
            sstore(IMPLEMENTATION_SLOT, implementation)

            let epoch := div(sub(timestamp(), START_TIMESTAMP), EPOCH_DURATION)
            sstore(STORAGE_SLOT, add(add(epoch, shl(128, tokens)), and(prevTokens, PREV_TOKENS_MASK)))
        }
    }

    // reverts if in a static call
    // for internal use only
    // note that the revert uses gas up to the gas limit
    function _revertIfStaticCall() external payable {
        assembly {
            tstore(0, 0)
        }
    }

    fallback() external payable {
        assembly {
            // copy gas remaining to stack
            let new_gas := gas()
            let data := 0

            // check if we are in a static call
            // note: as we overwrite the memory layout in
            // https://docs.soliditylang.org/en/v0.8.7/internals/layout_in_memory.html
            // it is unsafe to use non-assembly code in this function
            mstore(0, REVERT_IF_STATIC_CALL_SELECTOR)
            // this delegatecall uses 238 gas. we provide 500 just in case the gas cost increases in a future hardfork.
            if eq(delegatecall(500, address(), 0, 4, 0, 0), 1) { data := sload(STORAGE_SLOT) }

            // copy calldata to memory
            calldatacopy(0, 0, calldatasize())

            // forward call to logic contract
            let result := delegatecall(gas(), sload(IMPLEMENTATION_SLOT), 0, calldatasize(), 0, 0)

            // copy return data to memory
            returndatacopy(0, 0, returndatasize())

            // revert with return value if an error occurs
            if eq(result, 0) { revert(0, returndatasize()) }

            // return early if this is a static call or the contract is uninitialized
            if eq(data, 0) { return(0, returndatasize()) }

            // update gas usage stats if the epoch changed
            let epoch := div(sub(timestamp(), START_TIMESTAMP), EPOCH_DURATION)

            // use xor instead of neq as yul does not have a neq operator
            if xor(epoch, and(data, EPOCH_MASK)) {
                switch sub(epoch, and(data, EPOCH_MASK))
                case 1 { data := add(epoch, shl(16, shr(144, data))) }
                default { data := epoch }
            }

            // compute gas used
            // note: this does not include l1 gas fees for rollups or gas refunds
            new_gas := mul(sub(new_gas, gas()), gasprice())

            // write to storage slot
            sstore(STORAGE_SLOT, add(data, shl(128, new_gas)))

            // return return data
            return(0, returndatasize())
        }
    }
}
