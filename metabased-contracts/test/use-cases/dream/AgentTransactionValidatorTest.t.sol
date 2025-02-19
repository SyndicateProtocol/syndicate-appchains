// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {
    AgentTransactionValidator,
    IMetabasedSequencerChain,
    IAgentApplication
} from "src/use-cases/dream/AgentTransactionValidator.sol";

contract MockAgentApplication {
    address public owner;
    mapping(address => bool) public isPermitted;

    constructor(address _owner) {
        owner = _owner;
    }

    function setPermission(address agent, bool permitted) external {
        isPermitted[agent] = permitted;
    }

    function isPermittedByAddress(address agentAddress) external view returns (bool) {
        return isPermitted[agentAddress];
    }
}

contract MockSequencerChain {
    bytes public lastProcessedTx;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function processTransaction(bytes calldata data) external {
        if (shouldRevert) {
            revert("Sequencer error");
        }
        lastProcessedTx = data;
    }
}

contract AgentTransactionValidatorTest is Test {
    AgentTransactionValidator public validator;
    MockAgentApplication public agentApplication;
    MockSequencerChain public sequencerChain;
    address public admin;

    // Use the valid raw transaction data (which begins with 0x02 and contains 12 RLP items)
    // This is the same raw tx as used in RLPTxDecoder's examples.
    bytes public validRawTx =
        hex"02f90ccb8307c83005808405f5e100840bebc2008080b90c7260806040523480156200001157600080fd5b506040518060400160405280600e81526020016d546865204269672043686565736560901b815250604051806040016040528060048152602001634348454560e01b815250816003908162000067919062000126565b50600462000076828262000126565b505050620001f2565b634e487b7160e01b600052604160045260246000fd5b600181811c90821680620000aa57607f821691505b602082108103620000cb57634e487b7160e01b600052602260045260246000fd5b50919050565b601f82111562000121576000816000526020600020601f850160051c81016020861015620000fc5750805b601f850160051c820191505b818110156200011d5782815560010162000108565b5050505b505050565b81516001600160401b038111156200014257620001426200007f565b6200015a8162000153845462000095565b84620000d1565b602080601f831160018114620001925760008415620001795750858301515b600019600386901b1c1916600185901b1785556200011d565b600085815260208120601f198616915b82811015620001c357888601518255948401946001909101908401620001a2565b5085821015620001e25787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b610a7080620002026000396000f3fe608060405234801561001057600080fd5b50600436106100f55760003560e01c80636a62784211610097578063a9059cbb11610066578063a9059cbb146101ee578063aae27e2014610201578063d40dc87014610214578063dd62ed3e1461022457600080fd5b80636a6278421461019757806370a08231146101aa57806395d89b41146101d3578063a457c2d7146101db57600080fd5b806323b872dd116100d357806323b872dd1461014d578063313ce56714610160578063395093511461016f57806340c10f191461018257600080fd5b806306fdde03146100fa578063095ea7b31461011857806318160ddd1461013b575b600080fd5b610102610237565b60405161010f9190610881565b60405180910390f35b61012b6101263660046108ec565b6102c9565b604051901515815260200161010f565b6002545b60405190815260200161010f565b61012b61015b366004610916565b6102e3565b6040516012815260200161010f565b61012b61017d3660046108ec565b610307565b6101956101903660046108ec565b610329565b005b6101956101a5366004610952565b610394565b61013f6101b8366004610952565b6001600160a01b031660009081526020819052604090205490565b6101026103a9565b61012b6101e93660046108ec565b6103b8565b61012b6101fc3660046108ec565b610433565b61013f61020f366004610974565b610441565b61013f683635c9adc5dea0000081565b61013f61023236600461098d565b610455565b606060038054610246906109c0565b80601f0160208091040260200160405190810160405280929190818152602001828054610272906109c0565b80156102bf5780601f10610294576101008083540402835291602001916102bf565b820191906000526020600020905b8154815290600101906020018083116102a257829003601f168201915b5050505050905090565b6000336102d7818585610480565b60019150505b92915050565b6000336102f18582856105a4565b6102fc85858561061e565b506001949350505050565b6000336102d781858561031a8383610455565b6103249190610a10565b610480565b683635c9adc5dea0000081106103865760405162461bcd60e51b815260206004820181905260248201527f5465737445524332303a204d6178206d696e7420697320312c3030302045544860448201526064015b60405180910390fd5b61039082826107c2565b5050565b6103a681670de0b6b3a76400006107c2565b50565b606060048054610246906109c0565b600033816103c68286610455565b9050838110156104265760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b606482015260840161037d565b6102fc8286868403610480565b6000336102d781858561061e565b60006102dd82670de0b6b3a7640000610a23565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b6001600160a01b0383166104e25760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b606482015260840161037d565b6001600160a01b0382166105435760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b606482015260840161037d565b6001600160a01b0383811660008181526001602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b60006105b08484610455565b90506000198114610618578181101561060b5760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e6365000000604482015260640161037d565b6106188484848403610480565b50505050565b6001600160a01b0383166106825760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b606482015260840161037d565b6001600160a01b0382166106e45760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b606482015260840161037d565b6001600160a01b0383166000908152602081905260409020548181101561075c5760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b606482015260840161037d565b6001600160a01b03848116600081815260208181526040808320878703905593871680835291849020805487019055925185815290927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a3610618565b6001600160a01b0382166108185760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f206164647265737300604482015260640161037d565b806002600082825461082a9190610a10565b90915550506001600160a01b038216600081815260208181526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a35050565b60006020808352835180602085015260005b818110156108af57858101830151858201604001528201610893565b506000604082860101526040601f19601f8301168501019250505092915050565b80356001600160a01b03811681146108e757600080fd5b919050565b600080604083850312156108ff57600080fd5b610908836108d0565b946020939093013593505050565b60008060006060848603121561092b57600080fd5b610934846108d0565b9250610942602085016108d0565b9150604084013590509250925092565b60006020828403121561096457600080fd5b61096d826108d0565b9392505050565b60006020828403121561098657600080fd5b5035919050565b600080604083850312156109a057600080fd5b6109a9836108d0565b91506109b7602084016108d0565b90509250929050565b600181811c908216806109d457607f821691505b6020821081036109f457634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b808201808211156102dd576102dd6109fa565b80820281158282048414176102dd576102dd6109fa56fea26469706673582212205f0e99edfb7fb809fe9a1f39d0882647f3971dfee1dd1d21529d29688cbbe92664736f6c63430008160033c080a01b635a0f5eed74cbd7c1b442e5aa68cc2a913cf03bb46088d84fe648533824c6a04e86b6dbb5da449a641cf755330800ac98fc826886d8c0cece16059bdf6c1262";

    // For these tests the "from" address is taken from the signature via ecrecover.
    // The above raw tx recovers to: 0xb6235EAEADfA5839CdA207B454d98b328dFE2F3A.
    address constant VALID_FROM = 0xb6235EAEADfA5839CdA207B454d98b328dFE2F3A;

    // Additional transactions from the JSON data
    bytes public additionalTx1 =
        hex"02f86c8307c83080808405f5e10082c350949a37e57d177c5ff8817b55da36f2a2b3532cde3f830186a080c080a089839adf5c5f5d8e2cdf354462e819086c7f4bd75271f8d720e32cd62f133961a06a9e224eba080dd83a192dcb014f0e8f14ecc466b847dbc23ad76f9d4caf13be";
    address constant ADDITIONAL_TX1_FROM = 0x5409c11C900aF5FBbc6ab80139A73d62Ec70a603;

    bytes public additionalTx2 =
        hex"02f8738307c8300284068e778084068e7780825208949a37e57d177c5ff8817b55da36f2a2b3532cde3f865af3107a400080c080a0c309c50038849f960bab0188dcddcb98fa614f5a444c3c42da01e41babd1d36ba04b03dbe5801cadbe90e1c04c9bf727014aa3894cd6935d133143b7c46adacf46";
    address constant ADDITIONAL_TX2_FROM = 0x18F33CEf45817C428d98C4E188A770191fDD4B79;

    bytes public additionalTx3 =
        hex"02f8738307c830028405f5e1008405f5e100825208949a37e57d177c5ff8817b55da36f2a2b3532cde3f865af3107a400080c080a08c8b3ce3316e63cc9c993d6b6444f1759d28d191eb87fa95d3c2310955d08f9ca00a9965092b217d22d6701bd39ba8c76aed0c2e64dcfdd8122e70a9bb50665322";
    address constant ADDITIONAL_TX3_FROM = 0x18F33CEf45817C428d98C4E188A770191fDD4B79;

    bytes public additionalTx4 =
        hex"02f88f8307c83008808405f5e100840bebc20094c08bc955da8968327405642d65a7513ce5eb31ed80a46a627842000000000000000000000000b6235eaeadfa5839cda207b454d98b328dfe2f3ac080a02c7bcba73ec713bed975cec06b5b424dec0b9134f0a154d4e5b265225e3b17b0a008907d32b8166e38a7e0a44e295105fb8712d128ef4c1e0ccbc7a215549d5492";
    address constant ADDITIONAL_TX4_FROM = 0xb6235EAEADfA5839CdA207B454d98b328dFE2F3A;

    bytes public additionalTx5 =
        hex"02f86f8307c8300501840bebc201825208949a37e57d177c5ff8817b55da36f2a2b3532cde3f865af3107a400080c001a03bb3154f4d1e37e6ee9a0c88b4b14ce7f3e8a952a11e989e7b5ba586ddc26b5ba03762cb077b9e9f57652d636d5fc1413d06e392f95d1b04fbc960d866de88c074";
    address constant ADDITIONAL_TX5_FROM = 0x18F33CEf45817C428d98C4E188A770191fDD4B79;

    event TransactionProcessed(address indexed sender, bytes data);

    function setUp() public {
        admin = makeAddr("admin");
        sequencerChain = new MockSequencerChain();
        agentApplication = new MockAgentApplication(admin);

        // Permit the VALID_FROM address and the admin
        agentApplication.setPermission(VALID_FROM, true);
        agentApplication.setPermission(admin, true);

        validator = new AgentTransactionValidator(address(agentApplication), address(sequencerChain));
    }

    function test_Constructor() public view {
        assertEq(address(validator.agentApplication()), address(agentApplication));
        assertEq(address(validator.sequencerChain()), address(sequencerChain));
    }

    function test_ProcessTransactionForApprovedAgent() public {
        // Use the valid raw tx
        bytes memory txData = validRawTx;
        validator.processTransaction(txData);

        // Verify the transaction was forwarded to the sequencer
        assertEq(sequencerChain.lastProcessedTx(), txData);
    }

    function test_ProcessTransactionForAdmin() public {
        // Revoke permission for the VALID_FROM address
        agentApplication.setPermission(VALID_FROM, false);

        // Mock that VALID_FROM is the owner
        vm.mockCall(
            address(agentApplication), abi.encodeWithSelector(IAgentApplication.owner.selector), abi.encode(VALID_FROM)
        );

        bytes memory txData = validRawTx;
        validator.processTransaction(txData);

        // Verify the transaction was forwarded to the sequencer
        assertEq(sequencerChain.lastProcessedTx(), txData);
    }

    function test_RevertWhen_UnauthorizedAgent() public {
        // Revoke permission for the valid sender
        agentApplication.setPermission(VALID_FROM, false);

        bytes memory txData = validRawTx;
        vm.expectRevert(AgentTransactionValidator.Unauthorized.selector);
        validator.processTransaction(txData);
    }

    function test_RevertWhen_InvalidTransactionData() public {
        bytes memory invalidData = bytes("invalid");
        // Expect the revert string from RLPTxDecoder
        vm.expectRevert("Not EIP-1559");
        validator.processTransaction(invalidData);
    }

    // Additional tests with the new transaction data
    function test_ProcessTransaction_AdditionalTx1() public {
        // Permit the sender
        agentApplication.setPermission(ADDITIONAL_TX1_FROM, true);

        validator.processTransaction(additionalTx1);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx1);
    }

    function test_ProcessTransaction_AdditionalTx2() public {
        // Permit the sender
        agentApplication.setPermission(ADDITIONAL_TX2_FROM, true);

        validator.processTransaction(additionalTx2);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx2);
    }

    function test_RevertWhen_AdditionalTx3_Unauthorized() public {
        // Do not permit this sender
        agentApplication.setPermission(ADDITIONAL_TX3_FROM, false);

        vm.expectRevert(AgentTransactionValidator.Unauthorized.selector);
        validator.processTransaction(additionalTx3);
    }

    function test_ProcessTransaction_AdditionalTx4_SameAsOriginalValid() public {
        // This uses the same from address as validRawTx
        validator.processTransaction(additionalTx4);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx4);
    }

    function test_ProcessTransaction_AdditionalTx5_AsOwner() public {
        // Do not permit the sender directly
        agentApplication.setPermission(ADDITIONAL_TX5_FROM, false);

        // But make it the owner
        vm.mockCall(
            address(agentApplication),
            abi.encodeWithSelector(IAgentApplication.owner.selector),
            abi.encode(ADDITIONAL_TX5_FROM)
        );

        validator.processTransaction(additionalTx5);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx5);
    }

    function test_BatchProcessTransactions() public {
        // Set up permissions for each transaction
        agentApplication.setPermission(ADDITIONAL_TX1_FROM, false);
        agentApplication.setPermission(ADDITIONAL_TX2_FROM, true);
        agentApplication.setPermission(ADDITIONAL_TX3_FROM, true);

        // TX4 already has permission (same as VALID_FROM)
        agentApplication.setPermission(ADDITIONAL_TX5_FROM, false);

        // Make TX5 sender the owner
        vm.mockCall(
            address(agentApplication),
            abi.encodeWithSelector(IAgentApplication.owner.selector),
            abi.encode(ADDITIONAL_TX5_FROM)
        );

        // Process each transaction and verify
        vm.expectRevert(AgentTransactionValidator.Unauthorized.selector);
        validator.processTransaction(additionalTx1);

        validator.processTransaction(additionalTx2);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx2);

        validator.processTransaction(additionalTx3);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx3);

        validator.processTransaction(additionalTx4);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx4);

        validator.processTransaction(additionalTx5);
        assertEq(sequencerChain.lastProcessedTx(), additionalTx5);
    }

    function test_SequencerError() public {
        // Permit the sender
        agentApplication.setPermission(VALID_FROM, true);

        // Make the sequencer revert
        sequencerChain.setShouldRevert(true);

        // The error should propagate from the sequencer
        vm.expectRevert("Sequencer error");
        validator.processTransaction(validRawTx);
    }
}
