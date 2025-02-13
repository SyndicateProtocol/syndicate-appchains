// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {
    AgentTransactionValidator,
    IMetabasedSequencerChain,
    IAgentApplication
} from "src/use-cases/dream/AgentTransactionValidator.sol";

contract MockAgentApplication {
    mapping(address => bool) public isPermitted;

    function setPermission(address agent, bool permitted) external {
        isPermitted[agent] = permitted;
    }

    function isPermittedByAddress(address agentAddress) external view returns (bool) {
        return isPermitted[agentAddress];
    }
}

contract AgentTransactionValidatorTest is Test {
    AgentTransactionValidator public validator;
    MockAgentApplication public agentApplication;
    address public sequencerChain;

    address public agent;
    address public nonAgent;
    address public target;

    event TransactionProcessed(address indexed sender, bytes data);

    function setUp() public {
        agent = makeAddr("agent");
        nonAgent = makeAddr("nonAgent");
        target = makeAddr("target");
        sequencerChain = makeAddr("sequencerChain");

        // Deploy and set up mock contracts
        agentApplication = new MockAgentApplication();
        validator = new AgentTransactionValidator(address(agentApplication), sequencerChain);

        // Mock the sequencer chain
        vm.etch(sequencerChain, hex"00");

        // Set up permissions
        agentApplication.setPermission(agent, true);
    }

    function test_Constructor() public view {
        assertEq(address(validator.agentApplication()), address(agentApplication));
        assertEq(address(validator.sequencerChain()), sequencerChain);
    }

    function test_ProcessTransactionForApprovedAgent() public {
        // Create transaction data starting with the agent address
        bytes memory functionCallData = abi.encodeWithSignature("someFunction()");
        bytes memory txData = bytes.concat(
            bytes20(agent), // from address
            bytes20(target), // to address
            bytes32(uint256(0)), // value
            bytes32(functionCallData.length), // length of call data
            functionCallData // actual call data
        );

        // Mock the sequencer call
        vm.mockCall(
            sequencerChain, abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, txData), ""
        );

        // Expect the call to be forwarded to the sequencer
        vm.expectCall(
            sequencerChain, abi.encodeWithSelector(IMetabasedSequencerChain.processTransaction.selector, txData)
        );

        validator.processTransaction(txData);
    }

    function test_RevertWhen_UnauthorizedAgent() public {
        // Create transaction data with unauthorized agent
        bytes memory functionCallData = abi.encodeWithSignature("someFunction()");
        bytes memory txData = bytes.concat(
            bytes20(nonAgent), // from address
            bytes20(target), // to address
            bytes32(uint256(0)), // value
            bytes32(functionCallData.length), // length of call data
            functionCallData // actual call data
        );

        vm.expectRevert(AgentTransactionValidator.Unauthorized.selector);
        validator.processTransaction(txData);
    }

    function test_RevertWhen_InvalidTransactionData() public {
        bytes memory invalidData = bytes("invalid");
        vm.expectRevert(AgentTransactionValidator.InvalidTransactionData.selector);
        validator.processTransaction(invalidData);
    }
}
