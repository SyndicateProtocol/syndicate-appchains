// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AtomicSequencerImplementation} from "./AtomicSequencerImplementation.sol";

/// @title AtomicSequencer
/// @notice A wrapper contract that sequences transactions on two MetabasedChain contracts atomically.
/// Uses DELEGATECALL to maintain the original msg.sender context.
contract AtomicSequencer {
    /// @notice The implementation contract containing the sequencing logic
    address public immutable implementation;

    /// @dev Error thrown when delegate call fails
    error DelegateCallFailed();

    constructor() {
        // Deploy the implementation contract and store its address
        implementation = address(new AtomicSequencerImplementation());
    }

    /// @notice Forwards all calls to the implementation contract via DELEGATECALL
    fallback() external {
        address impl = implementation;
        assembly {
            // Copy calldata to memory
            calldatacopy(0, 0, calldatasize())

            // Delegate call to implementation
            let success := delegatecall(gas(), impl, 0, calldatasize(), 0, 0)

            // Copy return data to memory
            returndatacopy(0, 0, returndatasize())

            switch success
            case 0 {
                // Revert with error if delegatecall failed
                revert(0, returndatasize())
            }
            default {
                // Return data if successful
                return(0, returndatasize())
            }
        }
    }

    /// @dev Prevents receiving Ether
    receive() external payable {
        revert();
    }
}
