// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// solhint-disable private-vars-leading-underscore
contract SyndicateProxy {
    // keccak256("eip1967.proxy.implementation") - 1
    bytes32 private constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    // keccak256("syndicate.proxy.storage") - 1
    bytes32 private constant EPOCH_SLOT = 0xd877d5a24209b8677258dc4c4521b9bd8c2e2ce630415c4e6e6d7ed4760d9821;
    // keccak256("syndicate.proxy.storage") - 2
    bytes32 private constant PREV_GAS_SLOT = 0xd877d5a24209b8677258dc4c4521b9bd8c2e2ce630415c4e6e6d7ed4760d9820;
    // keccak256("syndicate.proxy.storage") - 3
    bytes32 private constant GAS_SLOT = 0xd877d5a24209b8677258dc4c4521b9bd8c2e2ce630415c4e6e6d7ed4760d981f;

    // epoch constants. note that START_TIMESTAMP is the timestamp of epoch 0, not epoch 1.
    uint256 private constant START_TIMESTAMP = 1748905200;
    uint256 private constant EPOCH_DURATION = 30 days;

    // get gas tracking info
    function gasInfo() external view returns (uint256 epoch, uint256 prevTokens, uint256 tokens) {
        assembly {
            epoch := sload(EPOCH_SLOT)
            prevTokens := sload(PREV_GAS_SLOT)
            tokens := sload(GAS_SLOT)
        }
    }

    // get the gas usage for the previous epoch in tokens
    function tokensUsedPerEpoch(uint256 epochIndex) external view returns (uint256 tokens) {
        assembly {
            let epoch := div(sub(timestamp(), START_TIMESTAMP), EPOCH_DURATION)
            // xor is used since neq does not exist
            if xor(epoch, add(epochIndex, 1)) { revert(0, 0) }
            switch sub(epoch, sload(EPOCH_SLOT))
            case 0 { tokens := sload(PREV_GAS_SLOT) }
            case 1 { tokens := sload(GAS_SLOT) }
        }
    }

    function initializeProxy(address implementation, uint256 prevTokens, uint256 tokens) external {
        assembly {
            if gt(sload(EPOCH_SLOT), 0) { revert(0, 0) }
            sstore(IMPLEMENTATION_SLOT, implementation)
            sstore(EPOCH_SLOT, div(sub(timestamp(), START_TIMESTAMP), EPOCH_DURATION))
            sstore(PREV_GAS_SLOT, prevTokens)
            sstore(GAS_SLOT, tokens)
        }
    }

    fallback() external payable virtual {
        assembly {
            // copy gas remaining to stack
            let new_gas := gas()

            // copy storage slots to stack
            let old_epoch := sload(EPOCH_SLOT)
            let prev_gas := sload(PREV_GAS_SLOT)
            let cur_gas := sload(GAS_SLOT)

            // copy calldata to memory
            // as we overwrite the memory layout in
            // https://docs.soliditylang.org/en/v0.8.7/internals/layout_in_memory.html
            // it is unsafe to use non-assembly code in this function
            calldatacopy(0, 0, calldatasize())

            // forward call to logic contract
            let result := delegatecall(gas(), sload(IMPLEMENTATION_SLOT), 0, calldatasize(), 0, 0)

            // copy return data to memory
            returndatacopy(0, 0, returndatasize())

            // revert with return value if an error occurs
            if eq(result, 0) { revert(0, returndatasize()) }

            // compute epoch
            let epoch := div(sub(timestamp(), START_TIMESTAMP), EPOCH_DURATION)

            // update epoch storage slot
            sstore(EPOCH_SLOT, epoch)

            // compute gas used
            new_gas := mul(sub(new_gas, gas()), gasprice())

            // update gas usage stats
            switch sub(epoch, old_epoch)
            case 0 { new_gas := add(new_gas, cur_gas) }
            case 1 { prev_gas := cur_gas }
            default { prev_gas := 0 }

            // update remaining storage slots
            sstore(PREV_GAS_SLOT, prev_gas)
            sstore(GAS_SLOT, new_gas)

            // return return data
            return(0, returndatasize())
        }
    }
}
