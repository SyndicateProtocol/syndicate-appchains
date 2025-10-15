// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {SyndicateProxy} from "../SyndicateProxy.sol";

contract SyndicateForwarder {
    address public immutable source;
    address private immutable sender;

    /// @notice Emitted when a new SyndicateSequencingChain is created
    /// @param appchainId The unique identifier for the appchain
    /// @param sequencingChainAddress The address of the deployed sequencing chain contract
    event SyndicateSequencingChainCreated(uint256 indexed appchainId, address indexed sequencingChainAddress);

    constructor(address _source) {
        source = _source;
        if (block.chainid == 1) {
            sender = source;
        } else {
            // alias the contract address to receive messages from the same contract on the parent chain
            sender = address(uint160(address(this)) + uint160(0x1111000000000000000000000000000000001111));
        }
    }

    function call(address dest, bytes calldata data) external payable {
        require(msg.sender == sender);
        (bool success,) = payable(dest).call{value: msg.value}(data);
        require(success);
    }

    function deploy(uint256 chainId, address impl, uint128 prevGasUsed, uint128 gasUsed, bytes calldata init)
        external
    {
        require(msg.sender == sender);

        // Deploy the sequencing chain using the syndicate proxy
        address sequencingChain = Create2.deploy(0, bytes32(chainId), type(SyndicateProxy).creationCode);

        // Initialize the proxy
        SyndicateProxy(payable(sequencingChain)).initializeProxy(impl, prevGasUsed, gasUsed);

        // Initialize the implementation
        (bool success,) = sequencingChain.call(init);
        require(success);

        emit SyndicateSequencingChainCreated(chainId, sequencingChain);
    }
}
