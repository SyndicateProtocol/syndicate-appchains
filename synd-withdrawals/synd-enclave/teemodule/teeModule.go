// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package teemodule

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
)

// PendingAssertion is an auto generated low-level Go binding around an user-defined struct.
type PendingAssertion struct {
	AppBlockHash [32]byte
	AppSendRoot  [32]byte
	SeqBlockHash [32]byte
	L1BatchAcc   [32]byte
}

// TeeTrustedInput is an auto generated low-level Go binding around an user-defined struct.
type TeeTrustedInput struct {
	ConfigHash           [32]byte
	AppStartBlockHash    [32]byte
	SeqStartBlockHash    [32]byte
	SetDelayedMessageAcc [32]byte
	L1StartBatchAcc      [32]byte
	L1EndHash            [32]byte
}

// TeemoduleMetaData contains all meta data concerning the Teemodule contract.
var TeemoduleMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"poster_\",\"type\":\"address\",\"internalType\":\"contractIAssertionPoster\"},{\"name\":\"bridge_\",\"type\":\"address\",\"internalType\":\"contractIBridge\"},{\"name\":\"configHash_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appStartBlockHash_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqStartBlockHash_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1StartBatchAcc_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BlockOrBridge_\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"isL1Chain_\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"challengeWindowDuration_\",\"type\":\"uint64\",\"internalType\":\"uint64\"},{\"name\":\"teeKeyManager_\",\"type\":\"address\",\"internalType\":\"contractITeeKeyManager\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"receive\",\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"bridge\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBridge\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"challengeWindowDuration\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint64\",\"internalType\":\"uint64\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"challengeWindowEnd\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint64\",\"internalType\":\"uint64\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"closeChallengeWindow\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isL1Chain\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"l1BlockOrBridge\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pendingAssertions\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pendingAssertionsCount\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"poster\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIAssertionPoster\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"resolveChallenge\",\"inputs\":[{\"name\":\"assertion\",\"type\":\"tuple\",\"internalType\":\"structPendingAssertion\",\"components\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"submitAssertion\",\"inputs\":[{\"name\":\"assertion\",\"type\":\"tuple\",\"internalType\":\"structPendingAssertion\",\"components\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"rewardAddr\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"teeHackCount\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"teeKeyManager\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractITeeKeyManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"teeTrustedInput\",\"inputs\":[],\"outputs\":[{\"name\":\"configHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"setDelayedMessageAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1StartBatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1EndHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferAssertionPosterOwner\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferFunds\",\"inputs\":[{\"name\":\"dest\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateKeyManager\",\"inputs\":[{\"name\":\"newTeeKeyManager\",\"type\":\"address\",\"internalType\":\"contractITeeKeyManager\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"ChallengeResolved\",\"inputs\":[{\"name\":\"\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structPendingAssertion\",\"components\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"KeyManagerUpdate\",\"inputs\":[{\"name\":\"newTeeKeyManager\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractITeeKeyManager\"},{\"name\":\"oldTeeKeyManager\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractITeeKeyManager\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TeeConfigHash\",\"inputs\":[{\"name\":\"configHash\",\"type\":\"bytes32\",\"indexed\":false,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TeeHacked\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TeeInput\",\"inputs\":[{\"name\":\"input\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structTeeTrustedInput\",\"components\":[{\"name\":\"configHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"setDelayedMessageAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1StartBatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1EndHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"ECDSAInvalidSignature\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"ECDSAInvalidSignatureLength\",\"inputs\":[{\"name\":\"length\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"ECDSAInvalidSignatureS\",\"inputs\":[{\"name\":\"s\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"OwnableInvalidOwner\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"type\":\"error\",\"name\":\"OwnableUnauthorizedAccount\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}]}]",
	Bin: "0x61010080604052346104b3575f9061014081612aa780380380916100238285610b79565b8339810103126104b3578051906001600160a01b038216908183036104b3576020810151926001600160a01b0384168085036104b35760408301519460608401519560808501519160a08601519360c08701519560018060a01b03871687036104b35760e0880151988915158a036104b3576101206100a56101008b01610bb0565b9901516001600160a01b03811695908690036104b3573315610b66575f8054336001600160a01b03198216811783556040517fd266bca6281b20459ae52407bea3d134d9017bf8f3ba803cb7a11d724e2b2da69460209491939092916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3600a549d8e9d68010000000000000000600160801b039060401b16809e68010000000000000000600160801b031916179c8d600a5560c05260e052806002558152a160e05115610a2f5760c0516001600160a01b0316734200000000000000000000000000000000000015146109ea5760c0516040516221048360e21b815290602090829060049082906001600160a01b03165afa9081156106df575f916109b8575b501561095d575b3b1561090a5760805260405163eca067ad60e01b815290602090829060049082905afa9081156106df575f916108d8575b50156108835760a052803b1561082957600180546001600160a01b031916919091179055600386905560045560065560e051156107b557426001600160401b03165b6001600160401b038481169116111561075d5760401c6001600160401b0390811642821601908111610749576001600160401b03166001600160801b0319929092161717600a556008545f1981016106ea575060036102ab610bc4565b50015460065560026102bb610bc4565b5001546004556102c9610bc4565b505403610661575b60085481600855806105f8575b505b60a05160405163eca067ad60e01b81526001600160a01b0390911690602081600481855afa9081156105115783916105c6575b505f1981019081116104cb57602090602460405180948193636ab8cee160e11b835260048301525afa9081156105bb578291610589575b5060055560e0511561051c5760c0516040516221048360e21b81526001600160a01b0390911690602081600481855afa9081156105115783916104df575b505f1981019081116104cb576020906024604051809481936316bf557960e01b835260048301525afa9182156104bf5791610489575b506007555b7f55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca34960c06040516002548152600354602082015260045460408201526005546060820152600654608082015260075460a0820152a1604051611e6f9081610bf882396080518181816104f6015281816106330152611a19015260a05181818161022701526115f3015260c051818181610947015281816116df015281816118ef0152611c09015260e0518181816106e801528181610d81015281816112e701526115080152f35b90506020813d6020116104b7575b816104a460209383610b79565b810103126104b357515f6103be565b5f80fd5b3d9150610497565b604051903d90823e3d90fd5b634e487b7160e01b83526011600452602483fd5b90506020813d602011610509575b816104fa60209383610b79565b810103126104b357515f610388565b3d91506104ed565b6040513d85823e3d90fd5b60c051604051624dead360e51b815290602090829060049082906001600160a01b03165afa9182156104bf5791610557575b506007556103c3565b90506020813d602011610581575b8161057260209383610b79565b810103126104b357515f61054e565b3d9150610565565b90506020813d6020116105b3575b816105a460209383610b79565b810103126104b357515f61034a565b3d9150610597565b6040513d84823e3d90fd5b90506020813d6020116105f0575b816105e160209383610b79565b810103126104b357515f610313565b3d91506105d4565b6001600160fe1b038116810361064d576008825260021b5f516020612a875f395f51905f52908101905b81811061062f57506102de565b80836004925583600182015583600282015583600382015501610622565b634e487b7160e01b82526011600452602482fd5b610669610bc4565b50546003556080516001600160a01b0316610682610bc4565b5054600161068e610bc4565b500154823b156104b35760445f92836040519586948593636d755a0960e11b8552600485015260248401525af180156106df576106cc575b506102d1565b6106d891505f90610b79565b5f5f6106c6565b6040513d5f823e3d90fd5b9050156102e05760405162461bcd60e51b815260206004820152603360248201525f516020612a675f395f51905f5260448201527f746f6f206d616e7920617373657274696f6e73000000000000000000000000006064820152608490fd5b634e487b7160e01b5f52601160045260245ffd5b60405162461bcd60e51b815260206004820152603c60248201525f516020612a675f395f51905f5260448201527f696e73756666696369656e742074696d652068617320706173736564000000006064820152608490fd5b60c051604051635c03bbf560e11b815290602090829060049082906001600160a01b03165afa9081156106df575f916107ef575b5061024e565b90506020813d602011610821575b8161080a60209383610b79565b810103126104b35761081b90610bb0565b5f6107e9565b3d91506107fd565b60405162461bcd60e51b815260206004820152602c60248201527f7465654b65794d616e61676572206164647265737320646f6573206e6f74206860448201526b61766520616e7920636f646560a01b6064820152608490fd5b60405162461bcd60e51b815260206004820152602760248201527f696e73756666696369656e742064656c61796564206d6573736167657320696e6044820152662062726964676560c81b6064820152608490fd5b90506020813d602011610902575b816108f360209383610b79565b810103126104b357515f61020c565b3d91506108e6565b60405162461bcd60e51b815260206004820152602560248201527f706f73746572206164647265737320646f6573206e6f74206861766520616e7960448201526420636f646560d81b6064820152608490fd5b60405162461bcd60e51b815260206004820152602d60248201527f73657175656e63696e6720636861696e206d7573742068617665206174206c6560448201526c0c2e6e840dedcca40c4c2e8c6d609b1b6064820152608490fd5b90506020813d6020116109e2575b816109d360209383610b79565b810103126104b357515f6101d4565b3d91506109c6565b60405162461bcd60e51b815260206004820152601d60248201527f756e6578706563746564207365712062726964676520616464726573730000006044820152606490fd5b60c051604051635c03bbf560e11b815290602090829060049082906001600160a01b03165afa9081156106df575f91610b2c575b506001600160401b0316151580610abe575b6101db5760405162461bcd60e51b815260206004820152601960248201527f6c3120626c6f636b20636f6e747261637420696e76616c6964000000000000006044820152606490fd5b5060c051604051624dead360e51b815290602090829060049082906001600160a01b03165afa9081156106df575f91610afa575b501515610a75565b90506020813d602011610b24575b81610b1560209383610b79565b810103126104b357515f610af2565b3d9150610b08565b90506020813d602011610b5e575b81610b4760209383610b79565b810103126104b357610b5890610bb0565b5f610a63565b3d9150610b3a565b631e4fbdf760e01b5f525f60045260245ffd5b601f909101601f19168101906001600160401b03821190821017610b9c57604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160401b03821682036104b357565b60085415610be35760085f9081525f516020612a875f395f51905f5291565b634e487b7160e01b5f52603260045260245ffdfe608080604052600436101561001c575b50361561001a575f80fd5b005b5f905f3560e01c9081632521c5351461096b5750806327d402991461091b5780633183baac146108b8578063350bd6a3146107875780633a009a06146107545780633ceaae7d1461070d578063470b9b1a146106d1578063478bf556146105fa5780634bd167c9146105cf578063697b5e62146105b15780636c4c206014610598578063715018a61461051a57806380959721146104c95780638da5cb5b146104965780639b79e0c21461035b578063a56ec6cd14610304578063e39ff19f1461024b578063e78cea92146101fa578063ee1c28b8146101d25763f2fde38b0361000f57346101cf5760206003193601126101cf5773ffffffffffffffffffffffffffffffffffffffff61012e6109b3565b610136611c82565b1680156101a35773ffffffffffffffffffffffffffffffffffffffff8254827fffffffffffffffffffffffff00000000000000000000000000000000000000008216178455167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08380a380f35b6024827f1e4fbdf700000000000000000000000000000000000000000000000000000000815280600452fd5b80fd5b50346101cf57806003193601126101cf57602067ffffffffffffffff600a5416604051908152f35b50346101cf57806003193601126101cf57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101cf5760206003193601126101cf5773ffffffffffffffffffffffffffffffffffffffff61027a6109b3565b610282611c82565b1680156102a657818080806102a39447905af161029d610b48565b50610b77565b80f35b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f64657374696e6174696f6e2061646472657373206973207a65726f00000000006044820152fd5b50346101cf5760206003193601126101cf57600435906008548210156101cf57608061032f836109d6565b508054906001810154906003600282015491015491604051938452602084015260408301526060820152f35b50346101cf5760206003193601126101cf5760043573ffffffffffffffffffffffffffffffffffffffff811680910361049257610396611c82565b803b1561040e577fffffffffffffffffffffffff00000000000000000000000000000000000000006001547ff0993f232dc1fec9928385ddc3794d109479cdee2d14bf929a000bb3a448d70c6040805185815273ffffffffffffffffffffffffffffffffffffffff84166020820152a1161760015580f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602c60248201527f7465654b65794d616e61676572206164647265737320646f6573206e6f74206860448201527f61766520616e7920636f646500000000000000000000000000000000000000006064820152fd5b5080fd5b50346101cf57806003193601126101cf5773ffffffffffffffffffffffffffffffffffffffff6020915416604051908152f35b50346101cf57806003193601126101cf57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101cf57806003193601126101cf57610533611c82565b8073ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101cf57806003193601126101cf576102a3611505565b50346101cf57806003193601126101cf576020600954604051908152f35b50346101cf57806003193601126101cf57602067ffffffffffffffff600a5460401c16604051908152f35b50346106cd5760206003193601126106cd576106146109b3565b61061c611c82565b73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690813b156106cd5773ffffffffffffffffffffffffffffffffffffffff60245f928360405195869485937ff2fde38b0000000000000000000000000000000000000000000000000000000085521660048401525af180156106c2576106b6575080f35b61001a91505f90610a51565b6040513d5f823e3d90fd5b5f80fd5b346106cd575f6003193601126106cd5760206040517f000000000000000000000000000000000000000000000000000000000000000015158152f35b346106cd575f6003193601126106cd5760c06002546003546004546005546006549160075493604051958652602086015260408501526060840152608083015260a0820152f35b346106cd575f6003193601126106cd57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b346106cd57600319360160a081126106cd576080136106cd5760843567ffffffffffffffff81116106cd576107c0903690600401610985565b6107c8611c82565b6001600854111561085a576107e4916107df611477565b611177565b7fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000600a5416600a55610814611505565b7f2020542b6e6b951d4c0736eed2a4d762d20bb1ba579f99feffae9b1dea24088360806040516004358152602435602082015260443560408201526064356060820152a1005b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f6368616c6c656e676520646f6573206e6f7420657869737400000000000000006044820152fd5b346106cd57600319360160c081126106cd576080136106cd5760843567ffffffffffffffff81116106cd576108f1903690600401610985565b60a4359073ffffffffffffffffffffffffffffffffffffffff821682036106cd5761001a92610bdc565b346106cd575f6003193601126106cd57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346106cd575f6003193601126106cd576020906008548152f35b9181601f840112156106cd5782359167ffffffffffffffff83116106cd57602083818601950101116106cd57565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036106cd57565b6008548110156109f25760085f5260205f209060021b01905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b600854156109f25760085f9081527ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee391565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610a9257604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff8111610a9257601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b9067ffffffffffffffff8091169116019067ffffffffffffffff8211610b1b57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b3d15610b72573d90610b5982610abf565b91610b676040519384610a51565b82523d5f602084013e565b606090565b15610b7e57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f7061796d656e74206661696c65640000000000000000000000000000000000006044820152fd5b906041810361111957600435602435604435606435936040516020810190610c4e81610c228987898b889290916080949284526020840152604083015260608201520190565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282610a51565b519020956002546003546004546005546006549060075492604051946020860196875260408601526060850152608084015260a083015260c082015260c08152610c9960e082610a51565b5190206040516020810191825288604082015260408152610cbb606082610a51565b5190209173ffffffffffffffffffffffffffffffffffffffff6001541692610ce282610abf565b91610cf06040519384610a51565b80835236818501116106cd57610d2f836024935f602085610d3896829a8373ffffffffffffffffffffffffffffffffffffffff9b013784010152611cce565b90929192611d08565b60405194859384927f7217efcd0000000000000000000000000000000000000000000000000000000084521660048301525afa9081156106c2575f916110de575b5015611080577f0000000000000000000000000000000000000000000000000000000000000000158015611075575b156110175760085468010000000000000000811015610a9257806001610dd192016008556109d6565b929092610feb57600393835560018301556002820155015560085460018114610fa157600203610f1d57610e03610a1f565b50805490610e446001820154610c22600360028501549401546040519485936020850197889290916080949284526020840152604083015260608201520190565b51902014610ebf576009549060018201809211610b1b577f37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a760208373ffffffffffffffffffffffffffffffffffffffff94600955604051908152a1168015610ebc575f808080610eba9447905af161029d610b48565b565b50565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f617373657274696f6e20616c72656164792065786973747300000000000000006044820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f5465654d6f64756c653a20546f6f206d616e792070656e64696e67206173736560448201527f7274696f6e7300000000000000000000000000000000000000000000000000006064820152fd5b505050600a547fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000067ffffffffffffffff610fe2818460401c16824216610af9565b16911617600a55565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f756e6578706563746564206c3120656e642062617463682061636300000000006044820152fd5b506007548414610da8565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601560248201527f696e76616c696420746565207369676e617475726500000000000000000000006044820152fd5b90506020813d602011611111575b816110f960209383610a51565b810103126106cd575180151581036106cd575f610d79565b3d91506110ec565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f696e76616c6964207369676e6174757265206c656e67746800000000000000006044820152fd5b9060418103611119576004356024356044356064359360405160208101906111bd81610c228987898b889290916080949284526020840152604083015260608201520190565b519020956002546003546004546005546006549060075492604051946020860196875260408601526060850152608084015260a083015260c082015260c0815261120860e082610a51565b519020604051602081019182528860408201526040815261122a606082610a51565b5190209173ffffffffffffffffffffffffffffffffffffffff600154169261125182610abf565b9161125f6040519384610a51565b80835236818501116106cd57610d2f836024935f60208561129e96829a8373ffffffffffffffffffffffffffffffffffffffff9b013784010152611cce565b60405194859384927f7217efcd0000000000000000000000000000000000000000000000000000000084521660048301525afa9081156106c2575f9161143c575b5015611080577f0000000000000000000000000000000000000000000000000000000000000000158015611431575b156110175760085468010000000000000000811015610a925780600161133792016008556109d6565b929092610feb576003938355600183015560028201550155600854600181146113f157600203610f1d57611369610a1f565b508054906113aa6001820154610c22600360028501549401546040519485936020850197889290916080949284526020840152604083015260608201520190565b51902014610ebf5760095460018101809111610b1b576020817f37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a792600955604051908152a1565b5050600a547fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000067ffffffffffffffff610fe2818460401c16824216610af9565b50600754841461130e565b90506020813d60201161146f575b8161145760209383610a51565b810103126106cd575180151581036106cd575f6112df565b3d915061144a565b6008545f600855806114865750565b7f3fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81168103610b1b5760085f5260021b7ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee3908101905b8181106114e7575050565b805f600492555f60018201555f60028201555f6003820155016114dc565b5f7f00000000000000000000000000000000000000000000000000000000000000008015611bc65767ffffffffffffffff42165b600a549067ffffffffffffffff80831691161115611b42577fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000067ffffffffffffffff61158c818460401c16824216610af9565b16911617600a55600854600181145f14611ab9575060036115ab610a1f565b50015460065560026115bb610a1f565b5001546004556003546115cc610a1f565b5054036119f5575b6115dc611477565b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517feca067ad000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156119ea5784916119b8575b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff810190811161198b576020906024604051809481937fd5719dc200000000000000000000000000000000000000000000000000000000835260048301525afa9081156118a1578391611959575b50600555156118ac5773ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517e84120c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156118a157839161186f575b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908111611842576020906024604051809481937f16bf557900000000000000000000000000000000000000000000000000000000835260048301525afa9182156118365791611804575b506007555b7f55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca34960c06040516002548152600354602082015260045460408201526005546060820152600654608082015260075460a0820152a1565b90506020813d60201161182e575b8161181f60209383610a51565b810103126106cd57515f6117a9565b3d9150611812565b604051903d90823e3d90fd5b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d602011611899575b8161188a60209383610a51565b810103126106cd57515f61173b565b3d915061187d565b6040513d85823e3d90fd5b6040517f09bd5a6000000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa9182156118365791611927575b506007556117ae565b90506020813d602011611951575b8161194260209383610a51565b810103126106cd57515f61191e565b3d9150611935565b90506020813d602011611983575b8161197460209383610a51565b810103126106cd57515f6116bf565b3d9150611967565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b90506020813d6020116119e2575b816119d360209383610a51565b810103126106cd57515f611650565b3d91506119c6565b6040513d86823e3d90fd5b6119fd610a1f565b505460035573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016611a41610a1f565b50546001611a4d610a1f565b500154823b156106cd5760445f928360405195869485937fdaeab412000000000000000000000000000000000000000000000000000000008552600485015260248401525af180156106c257611aa4575b506115d4565b611ab19192505f90610a51565b5f905f611a9e565b156115dc5760846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603360248201527f63616e6e6f7420636c6f7365206368616c6c656e67652077696e646f77202d2060448201527f746f6f206d616e7920617373657274696f6e73000000000000000000000000006064820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603c60248201527f63616e6e6f7420636c6f7365206368616c6c656e67652077696e646f77202d2060448201527f696e73756666696369656e742074696d652068617320706173736564000000006064820152fd5b6040517fb80777ea00000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa9081156106c2575f91611c3f575b50611539565b90506020813d602011611c7a575b81611c5a60209383610a51565b810103126106cd575167ffffffffffffffff811681036106cd575f611c39565b3d9150611c4d565b73ffffffffffffffffffffffffffffffffffffffff5f54163303611ca257565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b8151919060418303611cfe57611cf79250602082015190606060408401519301515f1a90611de0565b9192909190565b50505f9160029190565b6004811015611db35780611d1a575050565b60018103611d4a577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b60028103611d7e57507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b600314611d885750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411611e64579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156106c2575f5173ffffffffffffffffffffffffffffffffffffffff811615611e5a57905f905f90565b505f906001905f90565b5050505f91600391905663616e6e6f7420636c6f7365206368616c6c656e67652077696e646f77202d20f3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee3",
}

// TeemoduleABI is the input ABI used to generate the binding from.
// Deprecated: Use TeemoduleMetaData.ABI instead.
var TeemoduleABI = TeemoduleMetaData.ABI

// TeemoduleBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use TeemoduleMetaData.Bin instead.
var TeemoduleBin = TeemoduleMetaData.Bin

// DeployTeemodule deploys a new Ethereum contract, binding an instance of Teemodule to it.
func DeployTeemodule(auth *bind.TransactOpts, backend bind.ContractBackend, poster_ common.Address, bridge_ common.Address, configHash_ [32]byte, appStartBlockHash_ [32]byte, seqStartBlockHash_ [32]byte, l1StartBatchAcc_ [32]byte, l1BlockOrBridge_ common.Address, isL1Chain_ bool, challengeWindowDuration_ uint64, teeKeyManager_ common.Address) (common.Address, *types.Transaction, *Teemodule, error) {
	parsed, err := TeemoduleMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(TeemoduleBin), backend, poster_, bridge_, configHash_, appStartBlockHash_, seqStartBlockHash_, l1StartBatchAcc_, l1BlockOrBridge_, isL1Chain_, challengeWindowDuration_, teeKeyManager_)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &Teemodule{TeemoduleCaller: TeemoduleCaller{contract: contract}, TeemoduleTransactor: TeemoduleTransactor{contract: contract}, TeemoduleFilterer: TeemoduleFilterer{contract: contract}}, nil
}

// Teemodule is an auto generated Go binding around an Ethereum contract.
type Teemodule struct {
	TeemoduleCaller     // Read-only binding to the contract
	TeemoduleTransactor // Write-only binding to the contract
	TeemoduleFilterer   // Log filterer for contract events
}

// TeemoduleCaller is an auto generated read-only Go binding around an Ethereum contract.
type TeemoduleCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// TeemoduleTransactor is an auto generated write-only Go binding around an Ethereum contract.
type TeemoduleTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// TeemoduleFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type TeemoduleFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// TeemoduleSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type TeemoduleSession struct {
	Contract     *Teemodule        // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// TeemoduleCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type TeemoduleCallerSession struct {
	Contract *TeemoduleCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts    // Call options to use throughout this session
}

// TeemoduleTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type TeemoduleTransactorSession struct {
	Contract     *TeemoduleTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts    // Transaction auth options to use throughout this session
}

// TeemoduleRaw is an auto generated low-level Go binding around an Ethereum contract.
type TeemoduleRaw struct {
	Contract *Teemodule // Generic contract binding to access the raw methods on
}

// TeemoduleCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type TeemoduleCallerRaw struct {
	Contract *TeemoduleCaller // Generic read-only contract binding to access the raw methods on
}

// TeemoduleTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type TeemoduleTransactorRaw struct {
	Contract *TeemoduleTransactor // Generic write-only contract binding to access the raw methods on
}

// NewTeemodule creates a new instance of Teemodule, bound to a specific deployed contract.
func NewTeemodule(address common.Address, backend bind.ContractBackend) (*Teemodule, error) {
	contract, err := bindTeemodule(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &Teemodule{TeemoduleCaller: TeemoduleCaller{contract: contract}, TeemoduleTransactor: TeemoduleTransactor{contract: contract}, TeemoduleFilterer: TeemoduleFilterer{contract: contract}}, nil
}

// NewTeemoduleCaller creates a new read-only instance of Teemodule, bound to a specific deployed contract.
func NewTeemoduleCaller(address common.Address, caller bind.ContractCaller) (*TeemoduleCaller, error) {
	contract, err := bindTeemodule(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &TeemoduleCaller{contract: contract}, nil
}

// NewTeemoduleTransactor creates a new write-only instance of Teemodule, bound to a specific deployed contract.
func NewTeemoduleTransactor(address common.Address, transactor bind.ContractTransactor) (*TeemoduleTransactor, error) {
	contract, err := bindTeemodule(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &TeemoduleTransactor{contract: contract}, nil
}

// NewTeemoduleFilterer creates a new log filterer instance of Teemodule, bound to a specific deployed contract.
func NewTeemoduleFilterer(address common.Address, filterer bind.ContractFilterer) (*TeemoduleFilterer, error) {
	contract, err := bindTeemodule(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &TeemoduleFilterer{contract: contract}, nil
}

// bindTeemodule binds a generic wrapper to an already deployed contract.
func bindTeemodule(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(TeemoduleABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Teemodule *TeemoduleRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Teemodule.Contract.TeemoduleCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Teemodule *TeemoduleRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Teemodule.Contract.TeemoduleTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Teemodule *TeemoduleRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Teemodule.Contract.TeemoduleTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Teemodule *TeemoduleCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Teemodule.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Teemodule *TeemoduleTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Teemodule.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Teemodule *TeemoduleTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Teemodule.Contract.contract.Transact(opts, method, params...)
}

// Bridge is a free data retrieval call binding the contract method 0xe78cea92.
//
// Solidity: function bridge() view returns(address)
func (_Teemodule *TeemoduleCaller) Bridge(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "bridge")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Bridge is a free data retrieval call binding the contract method 0xe78cea92.
//
// Solidity: function bridge() view returns(address)
func (_Teemodule *TeemoduleSession) Bridge() (common.Address, error) {
	return _Teemodule.Contract.Bridge(&_Teemodule.CallOpts)
}

// Bridge is a free data retrieval call binding the contract method 0xe78cea92.
//
// Solidity: function bridge() view returns(address)
func (_Teemodule *TeemoduleCallerSession) Bridge() (common.Address, error) {
	return _Teemodule.Contract.Bridge(&_Teemodule.CallOpts)
}

// ChallengeWindowDuration is a free data retrieval call binding the contract method 0x4bd167c9.
//
// Solidity: function challengeWindowDuration() view returns(uint64)
func (_Teemodule *TeemoduleCaller) ChallengeWindowDuration(opts *bind.CallOpts) (uint64, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "challengeWindowDuration")

	if err != nil {
		return *new(uint64), err
	}

	out0 := *abi.ConvertType(out[0], new(uint64)).(*uint64)

	return out0, err

}

// ChallengeWindowDuration is a free data retrieval call binding the contract method 0x4bd167c9.
//
// Solidity: function challengeWindowDuration() view returns(uint64)
func (_Teemodule *TeemoduleSession) ChallengeWindowDuration() (uint64, error) {
	return _Teemodule.Contract.ChallengeWindowDuration(&_Teemodule.CallOpts)
}

// ChallengeWindowDuration is a free data retrieval call binding the contract method 0x4bd167c9.
//
// Solidity: function challengeWindowDuration() view returns(uint64)
func (_Teemodule *TeemoduleCallerSession) ChallengeWindowDuration() (uint64, error) {
	return _Teemodule.Contract.ChallengeWindowDuration(&_Teemodule.CallOpts)
}

// ChallengeWindowEnd is a free data retrieval call binding the contract method 0xee1c28b8.
//
// Solidity: function challengeWindowEnd() view returns(uint64)
func (_Teemodule *TeemoduleCaller) ChallengeWindowEnd(opts *bind.CallOpts) (uint64, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "challengeWindowEnd")

	if err != nil {
		return *new(uint64), err
	}

	out0 := *abi.ConvertType(out[0], new(uint64)).(*uint64)

	return out0, err

}

// ChallengeWindowEnd is a free data retrieval call binding the contract method 0xee1c28b8.
//
// Solidity: function challengeWindowEnd() view returns(uint64)
func (_Teemodule *TeemoduleSession) ChallengeWindowEnd() (uint64, error) {
	return _Teemodule.Contract.ChallengeWindowEnd(&_Teemodule.CallOpts)
}

// ChallengeWindowEnd is a free data retrieval call binding the contract method 0xee1c28b8.
//
// Solidity: function challengeWindowEnd() view returns(uint64)
func (_Teemodule *TeemoduleCallerSession) ChallengeWindowEnd() (uint64, error) {
	return _Teemodule.Contract.ChallengeWindowEnd(&_Teemodule.CallOpts)
}

// IsL1Chain is a free data retrieval call binding the contract method 0x470b9b1a.
//
// Solidity: function isL1Chain() view returns(bool)
func (_Teemodule *TeemoduleCaller) IsL1Chain(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "isL1Chain")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsL1Chain is a free data retrieval call binding the contract method 0x470b9b1a.
//
// Solidity: function isL1Chain() view returns(bool)
func (_Teemodule *TeemoduleSession) IsL1Chain() (bool, error) {
	return _Teemodule.Contract.IsL1Chain(&_Teemodule.CallOpts)
}

// IsL1Chain is a free data retrieval call binding the contract method 0x470b9b1a.
//
// Solidity: function isL1Chain() view returns(bool)
func (_Teemodule *TeemoduleCallerSession) IsL1Chain() (bool, error) {
	return _Teemodule.Contract.IsL1Chain(&_Teemodule.CallOpts)
}

// L1BlockOrBridge is a free data retrieval call binding the contract method 0x27d40299.
//
// Solidity: function l1BlockOrBridge() view returns(address)
func (_Teemodule *TeemoduleCaller) L1BlockOrBridge(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "l1BlockOrBridge")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// L1BlockOrBridge is a free data retrieval call binding the contract method 0x27d40299.
//
// Solidity: function l1BlockOrBridge() view returns(address)
func (_Teemodule *TeemoduleSession) L1BlockOrBridge() (common.Address, error) {
	return _Teemodule.Contract.L1BlockOrBridge(&_Teemodule.CallOpts)
}

// L1BlockOrBridge is a free data retrieval call binding the contract method 0x27d40299.
//
// Solidity: function l1BlockOrBridge() view returns(address)
func (_Teemodule *TeemoduleCallerSession) L1BlockOrBridge() (common.Address, error) {
	return _Teemodule.Contract.L1BlockOrBridge(&_Teemodule.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Teemodule *TeemoduleCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Teemodule *TeemoduleSession) Owner() (common.Address, error) {
	return _Teemodule.Contract.Owner(&_Teemodule.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Teemodule *TeemoduleCallerSession) Owner() (common.Address, error) {
	return _Teemodule.Contract.Owner(&_Teemodule.CallOpts)
}

// PendingAssertions is a free data retrieval call binding the contract method 0xa56ec6cd.
//
// Solidity: function pendingAssertions(uint256 ) view returns(bytes32 appBlockHash, bytes32 appSendRoot, bytes32 seqBlockHash, bytes32 l1BatchAcc)
func (_Teemodule *TeemoduleCaller) PendingAssertions(opts *bind.CallOpts, arg0 *big.Int) (struct {
	AppBlockHash [32]byte
	AppSendRoot  [32]byte
	SeqBlockHash [32]byte
	L1BatchAcc   [32]byte
}, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "pendingAssertions", arg0)

	outstruct := new(struct {
		AppBlockHash [32]byte
		AppSendRoot  [32]byte
		SeqBlockHash [32]byte
		L1BatchAcc   [32]byte
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.AppBlockHash = *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)
	outstruct.AppSendRoot = *abi.ConvertType(out[1], new([32]byte)).(*[32]byte)
	outstruct.SeqBlockHash = *abi.ConvertType(out[2], new([32]byte)).(*[32]byte)
	outstruct.L1BatchAcc = *abi.ConvertType(out[3], new([32]byte)).(*[32]byte)

	return *outstruct, err

}

// PendingAssertions is a free data retrieval call binding the contract method 0xa56ec6cd.
//
// Solidity: function pendingAssertions(uint256 ) view returns(bytes32 appBlockHash, bytes32 appSendRoot, bytes32 seqBlockHash, bytes32 l1BatchAcc)
func (_Teemodule *TeemoduleSession) PendingAssertions(arg0 *big.Int) (struct {
	AppBlockHash [32]byte
	AppSendRoot  [32]byte
	SeqBlockHash [32]byte
	L1BatchAcc   [32]byte
}, error) {
	return _Teemodule.Contract.PendingAssertions(&_Teemodule.CallOpts, arg0)
}

// PendingAssertions is a free data retrieval call binding the contract method 0xa56ec6cd.
//
// Solidity: function pendingAssertions(uint256 ) view returns(bytes32 appBlockHash, bytes32 appSendRoot, bytes32 seqBlockHash, bytes32 l1BatchAcc)
func (_Teemodule *TeemoduleCallerSession) PendingAssertions(arg0 *big.Int) (struct {
	AppBlockHash [32]byte
	AppSendRoot  [32]byte
	SeqBlockHash [32]byte
	L1BatchAcc   [32]byte
}, error) {
	return _Teemodule.Contract.PendingAssertions(&_Teemodule.CallOpts, arg0)
}

// PendingAssertionsCount is a free data retrieval call binding the contract method 0x2521c535.
//
// Solidity: function pendingAssertionsCount() view returns(uint256)
func (_Teemodule *TeemoduleCaller) PendingAssertionsCount(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "pendingAssertionsCount")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// PendingAssertionsCount is a free data retrieval call binding the contract method 0x2521c535.
//
// Solidity: function pendingAssertionsCount() view returns(uint256)
func (_Teemodule *TeemoduleSession) PendingAssertionsCount() (*big.Int, error) {
	return _Teemodule.Contract.PendingAssertionsCount(&_Teemodule.CallOpts)
}

// PendingAssertionsCount is a free data retrieval call binding the contract method 0x2521c535.
//
// Solidity: function pendingAssertionsCount() view returns(uint256)
func (_Teemodule *TeemoduleCallerSession) PendingAssertionsCount() (*big.Int, error) {
	return _Teemodule.Contract.PendingAssertionsCount(&_Teemodule.CallOpts)
}

// Poster is a free data retrieval call binding the contract method 0x80959721.
//
// Solidity: function poster() view returns(address)
func (_Teemodule *TeemoduleCaller) Poster(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "poster")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Poster is a free data retrieval call binding the contract method 0x80959721.
//
// Solidity: function poster() view returns(address)
func (_Teemodule *TeemoduleSession) Poster() (common.Address, error) {
	return _Teemodule.Contract.Poster(&_Teemodule.CallOpts)
}

// Poster is a free data retrieval call binding the contract method 0x80959721.
//
// Solidity: function poster() view returns(address)
func (_Teemodule *TeemoduleCallerSession) Poster() (common.Address, error) {
	return _Teemodule.Contract.Poster(&_Teemodule.CallOpts)
}

// TeeHackCount is a free data retrieval call binding the contract method 0x697b5e62.
//
// Solidity: function teeHackCount() view returns(uint256)
func (_Teemodule *TeemoduleCaller) TeeHackCount(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "teeHackCount")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// TeeHackCount is a free data retrieval call binding the contract method 0x697b5e62.
//
// Solidity: function teeHackCount() view returns(uint256)
func (_Teemodule *TeemoduleSession) TeeHackCount() (*big.Int, error) {
	return _Teemodule.Contract.TeeHackCount(&_Teemodule.CallOpts)
}

// TeeHackCount is a free data retrieval call binding the contract method 0x697b5e62.
//
// Solidity: function teeHackCount() view returns(uint256)
func (_Teemodule *TeemoduleCallerSession) TeeHackCount() (*big.Int, error) {
	return _Teemodule.Contract.TeeHackCount(&_Teemodule.CallOpts)
}

// TeeKeyManager is a free data retrieval call binding the contract method 0x3a009a06.
//
// Solidity: function teeKeyManager() view returns(address)
func (_Teemodule *TeemoduleCaller) TeeKeyManager(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "teeKeyManager")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// TeeKeyManager is a free data retrieval call binding the contract method 0x3a009a06.
//
// Solidity: function teeKeyManager() view returns(address)
func (_Teemodule *TeemoduleSession) TeeKeyManager() (common.Address, error) {
	return _Teemodule.Contract.TeeKeyManager(&_Teemodule.CallOpts)
}

// TeeKeyManager is a free data retrieval call binding the contract method 0x3a009a06.
//
// Solidity: function teeKeyManager() view returns(address)
func (_Teemodule *TeemoduleCallerSession) TeeKeyManager() (common.Address, error) {
	return _Teemodule.Contract.TeeKeyManager(&_Teemodule.CallOpts)
}

// TeeTrustedInput is a free data retrieval call binding the contract method 0x3ceaae7d.
//
// Solidity: function teeTrustedInput() view returns(bytes32 configHash, bytes32 appStartBlockHash, bytes32 seqStartBlockHash, bytes32 setDelayedMessageAcc, bytes32 l1StartBatchAcc, bytes32 l1EndHash)
func (_Teemodule *TeemoduleCaller) TeeTrustedInput(opts *bind.CallOpts) (struct {
	ConfigHash           [32]byte
	AppStartBlockHash    [32]byte
	SeqStartBlockHash    [32]byte
	SetDelayedMessageAcc [32]byte
	L1StartBatchAcc      [32]byte
	L1EndHash            [32]byte
}, error) {
	var out []interface{}
	err := _Teemodule.contract.Call(opts, &out, "teeTrustedInput")

	outstruct := new(struct {
		ConfigHash           [32]byte
		AppStartBlockHash    [32]byte
		SeqStartBlockHash    [32]byte
		SetDelayedMessageAcc [32]byte
		L1StartBatchAcc      [32]byte
		L1EndHash            [32]byte
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.ConfigHash = *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)
	outstruct.AppStartBlockHash = *abi.ConvertType(out[1], new([32]byte)).(*[32]byte)
	outstruct.SeqStartBlockHash = *abi.ConvertType(out[2], new([32]byte)).(*[32]byte)
	outstruct.SetDelayedMessageAcc = *abi.ConvertType(out[3], new([32]byte)).(*[32]byte)
	outstruct.L1StartBatchAcc = *abi.ConvertType(out[4], new([32]byte)).(*[32]byte)
	outstruct.L1EndHash = *abi.ConvertType(out[5], new([32]byte)).(*[32]byte)

	return *outstruct, err

}

// TeeTrustedInput is a free data retrieval call binding the contract method 0x3ceaae7d.
//
// Solidity: function teeTrustedInput() view returns(bytes32 configHash, bytes32 appStartBlockHash, bytes32 seqStartBlockHash, bytes32 setDelayedMessageAcc, bytes32 l1StartBatchAcc, bytes32 l1EndHash)
func (_Teemodule *TeemoduleSession) TeeTrustedInput() (struct {
	ConfigHash           [32]byte
	AppStartBlockHash    [32]byte
	SeqStartBlockHash    [32]byte
	SetDelayedMessageAcc [32]byte
	L1StartBatchAcc      [32]byte
	L1EndHash            [32]byte
}, error) {
	return _Teemodule.Contract.TeeTrustedInput(&_Teemodule.CallOpts)
}

// TeeTrustedInput is a free data retrieval call binding the contract method 0x3ceaae7d.
//
// Solidity: function teeTrustedInput() view returns(bytes32 configHash, bytes32 appStartBlockHash, bytes32 seqStartBlockHash, bytes32 setDelayedMessageAcc, bytes32 l1StartBatchAcc, bytes32 l1EndHash)
func (_Teemodule *TeemoduleCallerSession) TeeTrustedInput() (struct {
	ConfigHash           [32]byte
	AppStartBlockHash    [32]byte
	SeqStartBlockHash    [32]byte
	SetDelayedMessageAcc [32]byte
	L1StartBatchAcc      [32]byte
	L1EndHash            [32]byte
}, error) {
	return _Teemodule.Contract.TeeTrustedInput(&_Teemodule.CallOpts)
}

// CloseChallengeWindow is a paid mutator transaction binding the contract method 0x6c4c2060.
//
// Solidity: function closeChallengeWindow() returns()
func (_Teemodule *TeemoduleTransactor) CloseChallengeWindow(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "closeChallengeWindow")
}

// CloseChallengeWindow is a paid mutator transaction binding the contract method 0x6c4c2060.
//
// Solidity: function closeChallengeWindow() returns()
func (_Teemodule *TeemoduleSession) CloseChallengeWindow() (*types.Transaction, error) {
	return _Teemodule.Contract.CloseChallengeWindow(&_Teemodule.TransactOpts)
}

// CloseChallengeWindow is a paid mutator transaction binding the contract method 0x6c4c2060.
//
// Solidity: function closeChallengeWindow() returns()
func (_Teemodule *TeemoduleTransactorSession) CloseChallengeWindow() (*types.Transaction, error) {
	return _Teemodule.Contract.CloseChallengeWindow(&_Teemodule.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_Teemodule *TeemoduleTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_Teemodule *TeemoduleSession) RenounceOwnership() (*types.Transaction, error) {
	return _Teemodule.Contract.RenounceOwnership(&_Teemodule.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_Teemodule *TeemoduleTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _Teemodule.Contract.RenounceOwnership(&_Teemodule.TransactOpts)
}

// ResolveChallenge is a paid mutator transaction binding the contract method 0x350bd6a3.
//
// Solidity: function resolveChallenge((bytes32,bytes32,bytes32,bytes32) assertion, bytes signature) returns()
func (_Teemodule *TeemoduleTransactor) ResolveChallenge(opts *bind.TransactOpts, assertion PendingAssertion, signature []byte) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "resolveChallenge", assertion, signature)
}

// ResolveChallenge is a paid mutator transaction binding the contract method 0x350bd6a3.
//
// Solidity: function resolveChallenge((bytes32,bytes32,bytes32,bytes32) assertion, bytes signature) returns()
func (_Teemodule *TeemoduleSession) ResolveChallenge(assertion PendingAssertion, signature []byte) (*types.Transaction, error) {
	return _Teemodule.Contract.ResolveChallenge(&_Teemodule.TransactOpts, assertion, signature)
}

// ResolveChallenge is a paid mutator transaction binding the contract method 0x350bd6a3.
//
// Solidity: function resolveChallenge((bytes32,bytes32,bytes32,bytes32) assertion, bytes signature) returns()
func (_Teemodule *TeemoduleTransactorSession) ResolveChallenge(assertion PendingAssertion, signature []byte) (*types.Transaction, error) {
	return _Teemodule.Contract.ResolveChallenge(&_Teemodule.TransactOpts, assertion, signature)
}

// SubmitAssertion is a paid mutator transaction binding the contract method 0x3183baac.
//
// Solidity: function submitAssertion((bytes32,bytes32,bytes32,bytes32) assertion, bytes signature, address rewardAddr) returns()
func (_Teemodule *TeemoduleTransactor) SubmitAssertion(opts *bind.TransactOpts, assertion PendingAssertion, signature []byte, rewardAddr common.Address) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "submitAssertion", assertion, signature, rewardAddr)
}

// SubmitAssertion is a paid mutator transaction binding the contract method 0x3183baac.
//
// Solidity: function submitAssertion((bytes32,bytes32,bytes32,bytes32) assertion, bytes signature, address rewardAddr) returns()
func (_Teemodule *TeemoduleSession) SubmitAssertion(assertion PendingAssertion, signature []byte, rewardAddr common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.SubmitAssertion(&_Teemodule.TransactOpts, assertion, signature, rewardAddr)
}

// SubmitAssertion is a paid mutator transaction binding the contract method 0x3183baac.
//
// Solidity: function submitAssertion((bytes32,bytes32,bytes32,bytes32) assertion, bytes signature, address rewardAddr) returns()
func (_Teemodule *TeemoduleTransactorSession) SubmitAssertion(assertion PendingAssertion, signature []byte, rewardAddr common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.SubmitAssertion(&_Teemodule.TransactOpts, assertion, signature, rewardAddr)
}

// TransferAssertionPosterOwner is a paid mutator transaction binding the contract method 0x478bf556.
//
// Solidity: function transferAssertionPosterOwner(address newOwner) returns()
func (_Teemodule *TeemoduleTransactor) TransferAssertionPosterOwner(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "transferAssertionPosterOwner", newOwner)
}

// TransferAssertionPosterOwner is a paid mutator transaction binding the contract method 0x478bf556.
//
// Solidity: function transferAssertionPosterOwner(address newOwner) returns()
func (_Teemodule *TeemoduleSession) TransferAssertionPosterOwner(newOwner common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.TransferAssertionPosterOwner(&_Teemodule.TransactOpts, newOwner)
}

// TransferAssertionPosterOwner is a paid mutator transaction binding the contract method 0x478bf556.
//
// Solidity: function transferAssertionPosterOwner(address newOwner) returns()
func (_Teemodule *TeemoduleTransactorSession) TransferAssertionPosterOwner(newOwner common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.TransferAssertionPosterOwner(&_Teemodule.TransactOpts, newOwner)
}

// TransferFunds is a paid mutator transaction binding the contract method 0xe39ff19f.
//
// Solidity: function transferFunds(address dest) returns()
func (_Teemodule *TeemoduleTransactor) TransferFunds(opts *bind.TransactOpts, dest common.Address) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "transferFunds", dest)
}

// TransferFunds is a paid mutator transaction binding the contract method 0xe39ff19f.
//
// Solidity: function transferFunds(address dest) returns()
func (_Teemodule *TeemoduleSession) TransferFunds(dest common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.TransferFunds(&_Teemodule.TransactOpts, dest)
}

// TransferFunds is a paid mutator transaction binding the contract method 0xe39ff19f.
//
// Solidity: function transferFunds(address dest) returns()
func (_Teemodule *TeemoduleTransactorSession) TransferFunds(dest common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.TransferFunds(&_Teemodule.TransactOpts, dest)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_Teemodule *TeemoduleTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_Teemodule *TeemoduleSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.TransferOwnership(&_Teemodule.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_Teemodule *TeemoduleTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.TransferOwnership(&_Teemodule.TransactOpts, newOwner)
}

// UpdateKeyManager is a paid mutator transaction binding the contract method 0x9b79e0c2.
//
// Solidity: function updateKeyManager(address newTeeKeyManager) returns()
func (_Teemodule *TeemoduleTransactor) UpdateKeyManager(opts *bind.TransactOpts, newTeeKeyManager common.Address) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "updateKeyManager", newTeeKeyManager)
}

// UpdateKeyManager is a paid mutator transaction binding the contract method 0x9b79e0c2.
//
// Solidity: function updateKeyManager(address newTeeKeyManager) returns()
func (_Teemodule *TeemoduleSession) UpdateKeyManager(newTeeKeyManager common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.UpdateKeyManager(&_Teemodule.TransactOpts, newTeeKeyManager)
}

// UpdateKeyManager is a paid mutator transaction binding the contract method 0x9b79e0c2.
//
// Solidity: function updateKeyManager(address newTeeKeyManager) returns()
func (_Teemodule *TeemoduleTransactorSession) UpdateKeyManager(newTeeKeyManager common.Address) (*types.Transaction, error) {
	return _Teemodule.Contract.UpdateKeyManager(&_Teemodule.TransactOpts, newTeeKeyManager)
}

// Receive is a paid mutator transaction binding the contract receive function.
//
// Solidity: receive() payable returns()
func (_Teemodule *TeemoduleTransactor) Receive(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Teemodule.contract.RawTransact(opts, nil) // calldata is disallowed for receive function
}

// Receive is a paid mutator transaction binding the contract receive function.
//
// Solidity: receive() payable returns()
func (_Teemodule *TeemoduleSession) Receive() (*types.Transaction, error) {
	return _Teemodule.Contract.Receive(&_Teemodule.TransactOpts)
}

// Receive is a paid mutator transaction binding the contract receive function.
//
// Solidity: receive() payable returns()
func (_Teemodule *TeemoduleTransactorSession) Receive() (*types.Transaction, error) {
	return _Teemodule.Contract.Receive(&_Teemodule.TransactOpts)
}

// TeemoduleChallengeResolvedIterator is returned from FilterChallengeResolved and is used to iterate over the raw logs and unpacked data for ChallengeResolved events raised by the Teemodule contract.
type TeemoduleChallengeResolvedIterator struct {
	Event *TeemoduleChallengeResolved // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *TeemoduleChallengeResolvedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleChallengeResolved)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(TeemoduleChallengeResolved)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *TeemoduleChallengeResolvedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleChallengeResolvedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleChallengeResolved represents a ChallengeResolved event raised by the Teemodule contract.
type TeemoduleChallengeResolved struct {
	Arg0 PendingAssertion
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterChallengeResolved is a free log retrieval operation binding the contract event 0x2020542b6e6b951d4c0736eed2a4d762d20bb1ba579f99feffae9b1dea240883.
//
// Solidity: event ChallengeResolved((bytes32,bytes32,bytes32,bytes32) arg0)
func (_Teemodule *TeemoduleFilterer) FilterChallengeResolved(opts *bind.FilterOpts) (*TeemoduleChallengeResolvedIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "ChallengeResolved")
	if err != nil {
		return nil, err
	}
	return &TeemoduleChallengeResolvedIterator{contract: _Teemodule.contract, event: "ChallengeResolved", logs: logs, sub: sub}, nil
}

// WatchChallengeResolved is a free log subscription operation binding the contract event 0x2020542b6e6b951d4c0736eed2a4d762d20bb1ba579f99feffae9b1dea240883.
//
// Solidity: event ChallengeResolved((bytes32,bytes32,bytes32,bytes32) arg0)
func (_Teemodule *TeemoduleFilterer) WatchChallengeResolved(opts *bind.WatchOpts, sink chan<- *TeemoduleChallengeResolved) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "ChallengeResolved")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleChallengeResolved)
				if err := _Teemodule.contract.UnpackLog(event, "ChallengeResolved", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseChallengeResolved is a log parse operation binding the contract event 0x2020542b6e6b951d4c0736eed2a4d762d20bb1ba579f99feffae9b1dea240883.
//
// Solidity: event ChallengeResolved((bytes32,bytes32,bytes32,bytes32) arg0)
func (_Teemodule *TeemoduleFilterer) ParseChallengeResolved(log types.Log) (*TeemoduleChallengeResolved, error) {
	event := new(TeemoduleChallengeResolved)
	if err := _Teemodule.contract.UnpackLog(event, "ChallengeResolved", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// TeemoduleKeyManagerUpdateIterator is returned from FilterKeyManagerUpdate and is used to iterate over the raw logs and unpacked data for KeyManagerUpdate events raised by the Teemodule contract.
type TeemoduleKeyManagerUpdateIterator struct {
	Event *TeemoduleKeyManagerUpdate // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *TeemoduleKeyManagerUpdateIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleKeyManagerUpdate)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(TeemoduleKeyManagerUpdate)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *TeemoduleKeyManagerUpdateIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleKeyManagerUpdateIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleKeyManagerUpdate represents a KeyManagerUpdate event raised by the Teemodule contract.
type TeemoduleKeyManagerUpdate struct {
	NewTeeKeyManager common.Address
	OldTeeKeyManager common.Address
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterKeyManagerUpdate is a free log retrieval operation binding the contract event 0xf0993f232dc1fec9928385ddc3794d109479cdee2d14bf929a000bb3a448d70c.
//
// Solidity: event KeyManagerUpdate(address newTeeKeyManager, address oldTeeKeyManager)
func (_Teemodule *TeemoduleFilterer) FilterKeyManagerUpdate(opts *bind.FilterOpts) (*TeemoduleKeyManagerUpdateIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "KeyManagerUpdate")
	if err != nil {
		return nil, err
	}
	return &TeemoduleKeyManagerUpdateIterator{contract: _Teemodule.contract, event: "KeyManagerUpdate", logs: logs, sub: sub}, nil
}

// WatchKeyManagerUpdate is a free log subscription operation binding the contract event 0xf0993f232dc1fec9928385ddc3794d109479cdee2d14bf929a000bb3a448d70c.
//
// Solidity: event KeyManagerUpdate(address newTeeKeyManager, address oldTeeKeyManager)
func (_Teemodule *TeemoduleFilterer) WatchKeyManagerUpdate(opts *bind.WatchOpts, sink chan<- *TeemoduleKeyManagerUpdate) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "KeyManagerUpdate")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleKeyManagerUpdate)
				if err := _Teemodule.contract.UnpackLog(event, "KeyManagerUpdate", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseKeyManagerUpdate is a log parse operation binding the contract event 0xf0993f232dc1fec9928385ddc3794d109479cdee2d14bf929a000bb3a448d70c.
//
// Solidity: event KeyManagerUpdate(address newTeeKeyManager, address oldTeeKeyManager)
func (_Teemodule *TeemoduleFilterer) ParseKeyManagerUpdate(log types.Log) (*TeemoduleKeyManagerUpdate, error) {
	event := new(TeemoduleKeyManagerUpdate)
	if err := _Teemodule.contract.UnpackLog(event, "KeyManagerUpdate", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// TeemoduleOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the Teemodule contract.
type TeemoduleOwnershipTransferredIterator struct {
	Event *TeemoduleOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *TeemoduleOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(TeemoduleOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *TeemoduleOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleOwnershipTransferred represents a OwnershipTransferred event raised by the Teemodule contract.
type TeemoduleOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_Teemodule *TeemoduleFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*TeemoduleOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &TeemoduleOwnershipTransferredIterator{contract: _Teemodule.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_Teemodule *TeemoduleFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *TeemoduleOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleOwnershipTransferred)
				if err := _Teemodule.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_Teemodule *TeemoduleFilterer) ParseOwnershipTransferred(log types.Log) (*TeemoduleOwnershipTransferred, error) {
	event := new(TeemoduleOwnershipTransferred)
	if err := _Teemodule.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// TeemoduleTeeConfigHashIterator is returned from FilterTeeConfigHash and is used to iterate over the raw logs and unpacked data for TeeConfigHash events raised by the Teemodule contract.
type TeemoduleTeeConfigHashIterator struct {
	Event *TeemoduleTeeConfigHash // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *TeemoduleTeeConfigHashIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleTeeConfigHash)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(TeemoduleTeeConfigHash)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *TeemoduleTeeConfigHashIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleTeeConfigHashIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleTeeConfigHash represents a TeeConfigHash event raised by the Teemodule contract.
type TeemoduleTeeConfigHash struct {
	ConfigHash [32]byte
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterTeeConfigHash is a free log retrieval operation binding the contract event 0xd266bca6281b20459ae52407bea3d134d9017bf8f3ba803cb7a11d724e2b2da6.
//
// Solidity: event TeeConfigHash(bytes32 configHash)
func (_Teemodule *TeemoduleFilterer) FilterTeeConfigHash(opts *bind.FilterOpts) (*TeemoduleTeeConfigHashIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "TeeConfigHash")
	if err != nil {
		return nil, err
	}
	return &TeemoduleTeeConfigHashIterator{contract: _Teemodule.contract, event: "TeeConfigHash", logs: logs, sub: sub}, nil
}

// WatchTeeConfigHash is a free log subscription operation binding the contract event 0xd266bca6281b20459ae52407bea3d134d9017bf8f3ba803cb7a11d724e2b2da6.
//
// Solidity: event TeeConfigHash(bytes32 configHash)
func (_Teemodule *TeemoduleFilterer) WatchTeeConfigHash(opts *bind.WatchOpts, sink chan<- *TeemoduleTeeConfigHash) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "TeeConfigHash")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleTeeConfigHash)
				if err := _Teemodule.contract.UnpackLog(event, "TeeConfigHash", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTeeConfigHash is a log parse operation binding the contract event 0xd266bca6281b20459ae52407bea3d134d9017bf8f3ba803cb7a11d724e2b2da6.
//
// Solidity: event TeeConfigHash(bytes32 configHash)
func (_Teemodule *TeemoduleFilterer) ParseTeeConfigHash(log types.Log) (*TeemoduleTeeConfigHash, error) {
	event := new(TeemoduleTeeConfigHash)
	if err := _Teemodule.contract.UnpackLog(event, "TeeConfigHash", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// TeemoduleTeeHackedIterator is returned from FilterTeeHacked and is used to iterate over the raw logs and unpacked data for TeeHacked events raised by the Teemodule contract.
type TeemoduleTeeHackedIterator struct {
	Event *TeemoduleTeeHacked // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *TeemoduleTeeHackedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleTeeHacked)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(TeemoduleTeeHacked)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *TeemoduleTeeHackedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleTeeHackedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleTeeHacked represents a TeeHacked event raised by the Teemodule contract.
type TeemoduleTeeHacked struct {
	Arg0 *big.Int
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterTeeHacked is a free log retrieval operation binding the contract event 0x37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a7.
//
// Solidity: event TeeHacked(uint256 arg0)
func (_Teemodule *TeemoduleFilterer) FilterTeeHacked(opts *bind.FilterOpts) (*TeemoduleTeeHackedIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "TeeHacked")
	if err != nil {
		return nil, err
	}
	return &TeemoduleTeeHackedIterator{contract: _Teemodule.contract, event: "TeeHacked", logs: logs, sub: sub}, nil
}

// WatchTeeHacked is a free log subscription operation binding the contract event 0x37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a7.
//
// Solidity: event TeeHacked(uint256 arg0)
func (_Teemodule *TeemoduleFilterer) WatchTeeHacked(opts *bind.WatchOpts, sink chan<- *TeemoduleTeeHacked) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "TeeHacked")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleTeeHacked)
				if err := _Teemodule.contract.UnpackLog(event, "TeeHacked", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTeeHacked is a log parse operation binding the contract event 0x37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a7.
//
// Solidity: event TeeHacked(uint256 arg0)
func (_Teemodule *TeemoduleFilterer) ParseTeeHacked(log types.Log) (*TeemoduleTeeHacked, error) {
	event := new(TeemoduleTeeHacked)
	if err := _Teemodule.contract.UnpackLog(event, "TeeHacked", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// TeemoduleTeeInputIterator is returned from FilterTeeInput and is used to iterate over the raw logs and unpacked data for TeeInput events raised by the Teemodule contract.
type TeemoduleTeeInputIterator struct {
	Event *TeemoduleTeeInput // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *TeemoduleTeeInputIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleTeeInput)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(TeemoduleTeeInput)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *TeemoduleTeeInputIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleTeeInputIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleTeeInput represents a TeeInput event raised by the Teemodule contract.
type TeemoduleTeeInput struct {
	Input TeeTrustedInput
	Raw   types.Log // Blockchain specific contextual infos
}

// FilterTeeInput is a free log retrieval operation binding the contract event 0x55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca349.
//
// Solidity: event TeeInput((bytes32,bytes32,bytes32,bytes32,bytes32,bytes32) input)
func (_Teemodule *TeemoduleFilterer) FilterTeeInput(opts *bind.FilterOpts) (*TeemoduleTeeInputIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "TeeInput")
	if err != nil {
		return nil, err
	}
	return &TeemoduleTeeInputIterator{contract: _Teemodule.contract, event: "TeeInput", logs: logs, sub: sub}, nil
}

// WatchTeeInput is a free log subscription operation binding the contract event 0x55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca349.
//
// Solidity: event TeeInput((bytes32,bytes32,bytes32,bytes32,bytes32,bytes32) input)
func (_Teemodule *TeemoduleFilterer) WatchTeeInput(opts *bind.WatchOpts, sink chan<- *TeemoduleTeeInput) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "TeeInput")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleTeeInput)
				if err := _Teemodule.contract.UnpackLog(event, "TeeInput", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTeeInput is a log parse operation binding the contract event 0x55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca349.
//
// Solidity: event TeeInput((bytes32,bytes32,bytes32,bytes32,bytes32,bytes32) input)
func (_Teemodule *TeemoduleFilterer) ParseTeeInput(log types.Log) (*TeemoduleTeeInput, error) {
	event := new(TeemoduleTeeInput)
	if err := _Teemodule.contract.UnpackLog(event, "TeeInput", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
