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
	_ = abi.ConvertType
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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"poster_\",\"type\":\"address\",\"internalType\":\"contractIAssertionPoster\"},{\"name\":\"bridge_\",\"type\":\"address\",\"internalType\":\"contractIBridge\"},{\"name\":\"configHash_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appStartBlockHash_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqStartBlockHash_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1StartBatchAcc_\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BlockOrBridge_\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"isL1Chain_\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"challengeWindowDuration_\",\"type\":\"uint64\",\"internalType\":\"uint64\"},{\"name\":\"teeKeyManager_\",\"type\":\"address\",\"internalType\":\"contractITeeKeyManager\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"receive\",\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"bridge\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIBridge\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"challengeWindowDuration\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint64\",\"internalType\":\"uint64\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"challengeWindowEnd\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint64\",\"internalType\":\"uint64\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"closeChallengeWindow\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isL1Chain\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"l1BlockOrBridge\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pendingAssertions\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pendingAssertionsCount\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"poster\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIAssertionPoster\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"resolveChallenge\",\"inputs\":[{\"name\":\"assertion\",\"type\":\"tuple\",\"internalType\":\"structPendingAssertion\",\"components\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"submitAssertion\",\"inputs\":[{\"name\":\"assertion\",\"type\":\"tuple\",\"internalType\":\"structPendingAssertion\",\"components\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"rewardAddr\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"teeHackCount\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"teeKeyManager\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractITeeKeyManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"teeTrustedInput\",\"inputs\":[],\"outputs\":[{\"name\":\"configHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"setDelayedMessageAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1StartBatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1EndHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferAssertionPosterOwner\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferFunds\",\"inputs\":[{\"name\":\"dest\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateChallengeWindowDuration\",\"inputs\":[{\"name\":\"challengeWindowDuration_\",\"type\":\"uint64\",\"internalType\":\"uint64\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateKeyManager\",\"inputs\":[{\"name\":\"newTeeKeyManager\",\"type\":\"address\",\"internalType\":\"contractITeeKeyManager\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"AssertionPosterTransferred\",\"inputs\":[{\"name\":\"dest\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ChallengeResolved\",\"inputs\":[{\"name\":\"\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structPendingAssertion\",\"components\":[{\"name\":\"appBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appSendRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1BatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ChallengeWindowDurationUpdate\",\"inputs\":[{\"name\":\"newChallengeWindowDuration\",\"type\":\"uint64\",\"indexed\":false,\"internalType\":\"uint64\"},{\"name\":\"oldChallengeWindowDuration\",\"type\":\"uint64\",\"indexed\":false,\"internalType\":\"uint64\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"FundsTransferred\",\"inputs\":[{\"name\":\"dest\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"KeyManagerUpdate\",\"inputs\":[{\"name\":\"newTeeKeyManager\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractITeeKeyManager\"},{\"name\":\"oldTeeKeyManager\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractITeeKeyManager\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TeeHacked\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"TeeInput\",\"inputs\":[{\"name\":\"input\",\"type\":\"tuple\",\"indexed\":false,\"internalType\":\"structTeeTrustedInput\",\"components\":[{\"name\":\"configHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"appStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"seqStartBlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"setDelayedMessageAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1StartBatchAcc\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"l1EndHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"ECDSAInvalidSignature\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"ECDSAInvalidSignatureLength\",\"inputs\":[{\"name\":\"length\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"ECDSAInvalidSignatureS\",\"inputs\":[{\"name\":\"s\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"OwnableInvalidOwner\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"type\":\"error\",\"name\":\"OwnableUnauthorizedAccount\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}]}]",
	Bin: "0x61010080604052346103e5576101408161271e803803809161002182856108aa565b8339810103126103e55780516001600160a01b038116908181036103e55760208301516001600160a01b03811693908481036103e55760408201519460608301519360808401519560a08501519760c086015160018060a01b038116918282036103e55760e0880151801515928382036103e5576101206100a56101008c016108e1565b9a01516001600160a01b0381169a908b90036103e5573315610897575f8054336001600160a01b03198216811783556040519290916001600160a01b0316907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a362093a806001600160401b03831610156108435750600a8054600160401b600160801b03191660409290921b6fffffffffffffffff00000000000000001691909117905560c05260e0526002551561071a575060c0516001600160a01b0316734200000000000000000000000000000000000015146106d55760c0516040516221048360e21b815290602090829060049082906001600160a01b03165afa9081156103f1575f916106a3575b5015610648575b3b156105f55760805260405163eca067ad60e01b815290602090829060049082905afa9081156103f1575f916105c3575b501561056e5760a052803b156105145760018060a01b0319600154161760015560035560045560018060a01b0360a0511660405163eca067ad60e01b8152602081600481855afa9081156103f1575f916104e2575b505f1981019081116103fc57602090602460405180948193636ab8cee160e11b835260048301525afa9081156103f1575f916104b0575b5060055560065560e051156104425760c0516040516221048360e21b81526001600160a01b0390911690602081600481855afa9081156103f1575f91610410575b505f1981019081116103fc576020906024604051809481936316bf557960e01b835260048301525afa9081156103f1575f916103bb575b506007555b7f55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca34960c06040516002548152600354602082015260045460408201526005546060820152600654608082015260075460a0820152a1604051611e2890816108f682396080518181816104f801528181610676015261183c015260a0518181816102330152611733015260c05181818161095b0152818161163c01528181611a400152611b58015260e05181818161071601528181610e530152818161137d01526115c90152f35b90506020813d6020116103e9575b816103d6602093836108aa565b810103126103e557515f6102f0565b5f80fd5b3d91506103c9565b6040513d5f823e3d90fd5b634e487b7160e01b5f52601160045260245ffd5b90506020813d60201161043a575b8161042b602093836108aa565b810103126103e557515f6102b9565b3d915061041e565b60c051604051624dead360e51b815290602090829060049082906001600160a01b03165afa9081156103f1575f9161047e575b506007556102f5565b90506020813d6020116104a8575b81610499602093836108aa565b810103126103e557515f610475565b3d915061048c565b90506020813d6020116104da575b816104cb602093836108aa565b810103126103e557515f610278565b3d91506104be565b90506020813d60201161050c575b816104fd602093836108aa565b810103126103e557515f610241565b3d91506104f0565b60405162461bcd60e51b815260206004820152602c60248201527f7465654b65794d616e61676572206164647265737320646f6573206e6f74206860448201526b61766520616e7920636f646560a01b6064820152608490fd5b60405162461bcd60e51b815260206004820152602760248201527f696e73756666696369656e742064656c61796564206d6573736167657320696e6044820152662062726964676560c81b6064820152608490fd5b90506020813d6020116105ed575b816105de602093836108aa565b810103126103e557515f6101ec565b3d91506105d1565b60405162461bcd60e51b815260206004820152602560248201527f706f73746572206164647265737320646f6573206e6f74206861766520616e7960448201526420636f646560d81b6064820152608490fd5b60405162461bcd60e51b815260206004820152602d60248201527f73657175656e63696e6720636861696e206d7573742068617665206174206c6560448201526c0c2e6e840dedcca40c4c2e8c6d609b1b6064820152608490fd5b90506020813d6020116106cd575b816106be602093836108aa565b810103126103e557515f6101b4565b3d91506106b1565b60405162461bcd60e51b815260206004820152601d60248201527f756e6578706563746564207365712062726964676520616464726573730000006044820152606490fd5b602060049160405192838092635c03bbf560e11b82525afa9081156103f1575f91610809575b506001600160401b031615158061079b575b6101bb5760405162461bcd60e51b815260206004820152601960248201527f6c3120626c6f636b20636f6e747261637420696e76616c6964000000000000006044820152606490fd5b5060c051604051624dead360e51b815290602090829060049082906001600160a01b03165afa9081156103f1575f916107d7575b501515610752565b90506020813d602011610801575b816107f2602093836108aa565b810103126103e557515f6107cf565b3d91506107e5565b90506020813d60201161083b575b81610824602093836108aa565b810103126103e557610835906108e1565b5f610740565b3d9150610817565b62461bcd60e51b815260206004820152602960248201527f6368616c6c656e67652077696e646f77206d757374206265206c657373207468604482015268616e2061207765656b60b81b6064820152608490fd5b631e4fbdf760e01b5f525f60045260245ffd5b601f909101601f19168101906001600160401b038211908210176108cd57604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160401b03821682036103e55756fe6080604052600436101561001a575b3615610018575f80fd5b005b5f5f3560e01c806316275f871461099c5780632521c5351461097f57806327d402991461092f5780633183baac146108cc578063350bd6a3146107b55780633a009a06146107825780633ceaae7d1461073b578063470b9b1a146106ff578063478bf556146105fc5780634bd167c9146105d1578063697b5e62146105b35780636c4c20601461059a578063715018a61461051c57806380959721146104cb5780638da5cb5b146104985780639b79e0c214610377578063a56ec6cd14610320578063e39ff19f14610257578063e78cea9214610206578063ee1c28b8146101de5763f2fde38b1461010c575061000e565b346101db5760206003193601126101db5773ffffffffffffffffffffffffffffffffffffffff61013a610aee565b610142611c3b565b1680156101af5773ffffffffffffffffffffffffffffffffffffffff8254827fffffffffffffffffffffffff00000000000000000000000000000000000000008216178455167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08380a380f35b6024827f1e4fbdf700000000000000000000000000000000000000000000000000000000815280600452fd5b80fd5b50346101db57806003193601126101db57602067ffffffffffffffff600a5416604051908152f35b50346101db57806003193601126101db57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101db5760206003193601126101db5773ffffffffffffffffffffffffffffffffffffffff610286610aee565b61028e611c3b565b1680156102dc57818080806102d9947f17f29f58ff29e58f40fe3fa963a7469e393593784592e72c3b2355f9199776e06020604051838152a147905af16102d3610c34565b50610c63565b80f35b606460405162461bcd60e51b815260206004820152601b60248201527f64657374696e6174696f6e2061646472657373206973207a65726f00000000006044820152fd5b50346101db5760206003193601126101db57600435906008548210156101db57608061034b83610b11565b508054906001810154906003600282015491015491604051938452602084015260408301526060820152f35b50346101db5760206003193601126101db5760043573ffffffffffffffffffffffffffffffffffffffff8116809103610494576103b2611c3b565b803b1561042a577fffffffffffffffffffffffff00000000000000000000000000000000000000006001547ff0993f232dc1fec9928385ddc3794d109479cdee2d14bf929a000bb3a448d70c6040805185815273ffffffffffffffffffffffffffffffffffffffff84166020820152a1161760015580f35b608460405162461bcd60e51b815260206004820152602c60248201527f7465654b65794d616e61676572206164647265737320646f6573206e6f74206860448201527f61766520616e7920636f646500000000000000000000000000000000000000006064820152fd5b5080fd5b50346101db57806003193601126101db5773ffffffffffffffffffffffffffffffffffffffff6020915416604051908152f35b50346101db57806003193601126101db57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101db57806003193601126101db57610535611c3b565b8073ffffffffffffffffffffffffffffffffffffffff81547fffffffffffffffffffffffff000000000000000000000000000000000000000081168355167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101db57806003193601126101db576102d96115bd565b50346101db57806003193601126101db576020600954604051908152f35b50346101db57806003193601126101db57602067ffffffffffffffff600a5460401c16604051908152f35b50346106fb5760206003193601126106fb57610616610aee565b61061e611c3b565b7e2ae90e22e60b8948054f7d1ac3af1d32155f74a4911928decf0c3a6f6351b1602073ffffffffffffffffffffffffffffffffffffffff604051931692838152a173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690813b156106fb575f916024839260405194859384927ff2fde38b00000000000000000000000000000000000000000000000000000000845260048401525af180156106f0576106e4575080f35b61001891505f90610b8c565b6040513d5f823e3d90fd5b5f80fd5b346106fb575f6003193601126106fb5760206040517f000000000000000000000000000000000000000000000000000000000000000015158152f35b346106fb575f6003193601126106fb5760c06002546003546004546005546006549160075493604051958652602086015260408501526060840152608083015260a0820152f35b346106fb575f6003193601126106fb57602073ffffffffffffffffffffffffffffffffffffffff60015416604051908152f35b346106fb57600319360160a081126106fb576080136106fb5760843567ffffffffffffffff81116106fb576107ee903690600401610ac0565b6107f6611c3b565b60016008541115610888576108129161080d61152f565b61120d565b7fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000600a5416600a556108426115bd565b7f2020542b6e6b951d4c0736eed2a4d762d20bb1ba579f99feffae9b1dea24088360806040516004358152602435602082015260443560408201526064356060820152a1005b606460405162461bcd60e51b815260206004820152601860248201527f6368616c6c656e676520646f6573206e6f7420657869737400000000000000006044820152fd5b346106fb57600319360160c081126106fb576080136106fb5760843567ffffffffffffffff81116106fb57610905903690600401610ac0565b60a4359073ffffffffffffffffffffffffffffffffffffffff821682036106fb5761001892610cae565b346106fb575f6003193601126106fb57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346106fb575f6003193601126106fb576020600854604051908152f35b346106fb5760206003193601126106fb5760043567ffffffffffffffff8116908181036106fb576109cb611c3b565b62093a80821015610a56576fffffffffffffffff00000000000000007fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff917f75689a8adaf52fab3f618b2698a3868150b33d8ba13b2f1a3ee2bcc3107336416040600a5495815190815267ffffffffffffffff87831c166020820152a160401b16911617600a555f80f35b608460405162461bcd60e51b815260206004820152602960248201527f6368616c6c656e67652077696e646f77206d757374206265206c65737320746860448201527f616e2061207765656b00000000000000000000000000000000000000000000006064820152fd5b9181601f840112156106fb5782359167ffffffffffffffff83116106fb57602083818601950101116106fb57565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036106fb57565b600854811015610b2d5760085f5260205f209060021b01905f90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60085415610b2d5760085f9081527ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee391565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610bcd57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b67ffffffffffffffff8111610bcd57601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b3d15610c5e573d90610c4582610bfa565b91610c536040519384610b8c565b82523d5f602084013e565b606090565b15610c6a57565b606460405162461bcd60e51b815260206004820152600e60248201527f7061796d656e74206661696c65640000000000000000000000000000000000006044820152fd5b90604181036111c957600435602435604435606435936040516020810190610d2081610cf48987898b889290916080949284526020840152604083015260608201520190565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282610b8c565b519020956002546003546004546005546006549060075492604051946020860196875260408601526060850152608084015260a083015260c082015260c08152610d6b60e082610b8c565b5190206040516020810191825288604082015260408152610d8d606082610b8c565b5190209173ffffffffffffffffffffffffffffffffffffffff6001541692610db482610bfa565b91610dc26040519384610b8c565b80835236818501116106fb57610e01836024935f602085610e0a96829a8373ffffffffffffffffffffffffffffffffffffffff9b013784010152611c87565b90929192611cc1565b60405194859384927f7217efcd0000000000000000000000000000000000000000000000000000000084521660048301525afa9081156106f0575f9161118e575b501561114a577f000000000000000000000000000000000000000000000000000000000000000015801561113f575b156110fb5760085468010000000000000000811015610bcd57806001610ea39201600855610b11565b9290926110cf5760039383556001830155600282015501556008546001811461106c5760020361100257610ed5610b5a565b50805490610f166001820154610cf4600360028501549401546040519485936020850197889290916080949284526020840152604083015260608201520190565b51902014610fbe576009549060018201809211610f91577f37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a760208373ffffffffffffffffffffffffffffffffffffffff94600955604051908152a1168015610f8e575f808080610f8c9447905af16102d3610c34565b565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b606460405162461bcd60e51b815260206004820152601860248201527f617373657274696f6e20616c72656164792065786973747300000000000000006044820152fd5b608460405162461bcd60e51b815260206004820152602660248201527f5465654d6f64756c653a20546f6f206d616e792070656e64696e67206173736560448201527f7274696f6e7300000000000000000000000000000000000000000000000000006064820152fd5b505050600a5467ffffffffffffffff8160401c1667ffffffffffffffff42160167ffffffffffffffff8111610f915767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600a55565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b606460405162461bcd60e51b815260206004820152601b60248201527f756e6578706563746564206c3120656e642062617463682061636300000000006044820152fd5b506007548414610e7a565b606460405162461bcd60e51b815260206004820152601560248201527f696e76616c696420746565207369676e617475726500000000000000000000006044820152fd5b90506020813d6020116111c1575b816111a960209383610b8c565b810103126106fb575180151581036106fb575f610e4b565b3d915061119c565b606460405162461bcd60e51b815260206004820152601860248201527f696e76616c6964207369676e6174757265206c656e67746800000000000000006044820152fd5b90604181036111c95760043560243560443560643593604051602081019061125381610cf48987898b889290916080949284526020840152604083015260608201520190565b519020956002546003546004546005546006549060075492604051946020860196875260408601526060850152608084015260a083015260c082015260c0815261129e60e082610b8c565b51902060405160208101918252886040820152604081526112c0606082610b8c565b5190209173ffffffffffffffffffffffffffffffffffffffff60015416926112e782610bfa565b916112f56040519384610b8c565b80835236818501116106fb57610e01836024935f60208561133496829a8373ffffffffffffffffffffffffffffffffffffffff9b013784010152611c87565b60405194859384927f7217efcd0000000000000000000000000000000000000000000000000000000084521660048301525afa9081156106f0575f916114f4575b501561114a577f00000000000000000000000000000000000000000000000000000000000000001580156114e9575b156110fb5760085468010000000000000000811015610bcd578060016113cd9201600855610b11565b9290926110cf5760039383556001830155600282015501556008546001811461148757600203611002576113ff610b5a565b508054906114406001820154610cf4600360028501549401546040519485936020850197889290916080949284526020840152604083015260608201520190565b51902014610fbe5760095460018101809111610f91576020817f37e8add694c5926d564e971160f5974103cbbbc7c90747c4c6f802031d3567a792600955604051908152a1565b5050600a5467ffffffffffffffff8160401c1667ffffffffffffffff42160167ffffffffffffffff8111610f915767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600a55565b5060075484146113a4565b90506020813d602011611527575b8161150f60209383610b8c565b810103126106fb575180151581036106fb575f611375565b3d9150611502565b6008545f6008558061153e5750565b7f3fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81168103610f915760085f5260021b7ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee3908101905b81811061159f575050565b805f600492555f60018201555f60028201555f600382015501611594565b600160085403611bd1577f00000000000000000000000000000000000000000000000000000000000000008015611b155767ffffffffffffffff42165b67ffffffffffffffff80600a541691161115611aab57600361161a610b5a565b500154600655156119fd5773ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517e84120c000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156106f0575f916119cb575b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908111610f91576020906024604051809481937f16bf557900000000000000000000000000000000000000000000000000000000835260048301525afa9081156106f0575f91611999575b506007555b6002611716610b5a565b50015460045573ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517feca067ad000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156106f0575f91611967575b507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8101908111610f91576020906024604051809481937fd5719dc200000000000000000000000000000000000000000000000000000000835260048301525afa9081156106f0575f91611935575b5060055560035461180e610b5a565b5054146119285761181d610b5a565b5054600355600161182c610b5a565b50015461183761152f565b6003547f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1691823b156106fb5760445f928360405195869485937fdaeab412000000000000000000000000000000000000000000000000000000008552600485015260248401525af180156106f057611918575b505b7f55232299d83faf4dc2c32e228af37632bca7fa6dbc03b41224c100c6c9dca34960c06040516002548152600354602082015260045460408201526005546060820152600654608082015260075460a0820152a1565b5f61192291610b8c565b5f6118c0565b61193061152f565b6118c2565b90506020813d60201161195f575b8161195060209383610b8c565b810103126106fb57515f6117ff565b3d9150611943565b90506020813d602011611991575b8161198260209383610b8c565b810103126106fb57515f611790565b3d9150611975565b90506020813d6020116119c3575b816119b460209383610b8c565b810103126106fb57515f611707565b3d91506119a7565b90506020813d6020116119f5575b816119e660209383610b8c565b810103126106fb57515f611698565b3d91506119d9565b6040517f09bd5a6000000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa9081156106f0575f91611a79575b5060075561170c565b90506020813d602011611aa3575b81611a9460209383610b8c565b810103126106fb57515f611a70565b3d9150611a87565b608460405162461bcd60e51b815260206004820152603c60248201527f63616e6e6f7420636c6f7365206368616c6c656e67652077696e646f77202d2060448201527f696e73756666696369656e742074696d652068617320706173736564000000006064820152fd5b6040517fb80777ea00000000000000000000000000000000000000000000000000000000815260208160048173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa9081156106f0575f91611b8e575b506115fa565b90506020813d602011611bc9575b81611ba960209383610b8c565b810103126106fb575167ffffffffffffffff811681036106fb575f611b88565b3d9150611b9c565b608460405162461bcd60e51b815260206004820152603a60248201527f63616e6e6f7420636c6f7365206368616c6c656e67652077696e646f77202d2060448201527f77726f6e67206e756d626572206f6620617373657274696f6e730000000000006064820152fd5b73ffffffffffffffffffffffffffffffffffffffff5f54163303611c5b57565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b8151919060418303611cb757611cb09250602082015190606060408401519301515f1a90611d99565b9192909190565b50505f9160029190565b6004811015611d6c5780611cd3575050565b60018103611d03577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b60028103611d3757507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b600314611d415750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411611e1d579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156106f0575f5173ffffffffffffffffffffffffffffffffffffffff811615611e1357905f905f90565b505f906001905f90565b5050505f916003919056",
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
	parsed, err := TeemoduleMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
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

// UpdateChallengeWindowDuration is a paid mutator transaction binding the contract method 0x16275f87.
//
// Solidity: function updateChallengeWindowDuration(uint64 challengeWindowDuration_) returns()
func (_Teemodule *TeemoduleTransactor) UpdateChallengeWindowDuration(opts *bind.TransactOpts, challengeWindowDuration_ uint64) (*types.Transaction, error) {
	return _Teemodule.contract.Transact(opts, "updateChallengeWindowDuration", challengeWindowDuration_)
}

// UpdateChallengeWindowDuration is a paid mutator transaction binding the contract method 0x16275f87.
//
// Solidity: function updateChallengeWindowDuration(uint64 challengeWindowDuration_) returns()
func (_Teemodule *TeemoduleSession) UpdateChallengeWindowDuration(challengeWindowDuration_ uint64) (*types.Transaction, error) {
	return _Teemodule.Contract.UpdateChallengeWindowDuration(&_Teemodule.TransactOpts, challengeWindowDuration_)
}

// UpdateChallengeWindowDuration is a paid mutator transaction binding the contract method 0x16275f87.
//
// Solidity: function updateChallengeWindowDuration(uint64 challengeWindowDuration_) returns()
func (_Teemodule *TeemoduleTransactorSession) UpdateChallengeWindowDuration(challengeWindowDuration_ uint64) (*types.Transaction, error) {
	return _Teemodule.Contract.UpdateChallengeWindowDuration(&_Teemodule.TransactOpts, challengeWindowDuration_)
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

// TeemoduleAssertionPosterTransferredIterator is returned from FilterAssertionPosterTransferred and is used to iterate over the raw logs and unpacked data for AssertionPosterTransferred events raised by the Teemodule contract.
type TeemoduleAssertionPosterTransferredIterator struct {
	Event *TeemoduleAssertionPosterTransferred // Event containing the contract specifics and raw log

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
func (it *TeemoduleAssertionPosterTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleAssertionPosterTransferred)
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
		it.Event = new(TeemoduleAssertionPosterTransferred)
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
func (it *TeemoduleAssertionPosterTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleAssertionPosterTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleAssertionPosterTransferred represents a AssertionPosterTransferred event raised by the Teemodule contract.
type TeemoduleAssertionPosterTransferred struct {
	Dest common.Address
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterAssertionPosterTransferred is a free log retrieval operation binding the contract event 0x002ae90e22e60b8948054f7d1ac3af1d32155f74a4911928decf0c3a6f6351b1.
//
// Solidity: event AssertionPosterTransferred(address dest)
func (_Teemodule *TeemoduleFilterer) FilterAssertionPosterTransferred(opts *bind.FilterOpts) (*TeemoduleAssertionPosterTransferredIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "AssertionPosterTransferred")
	if err != nil {
		return nil, err
	}
	return &TeemoduleAssertionPosterTransferredIterator{contract: _Teemodule.contract, event: "AssertionPosterTransferred", logs: logs, sub: sub}, nil
}

// WatchAssertionPosterTransferred is a free log subscription operation binding the contract event 0x002ae90e22e60b8948054f7d1ac3af1d32155f74a4911928decf0c3a6f6351b1.
//
// Solidity: event AssertionPosterTransferred(address dest)
func (_Teemodule *TeemoduleFilterer) WatchAssertionPosterTransferred(opts *bind.WatchOpts, sink chan<- *TeemoduleAssertionPosterTransferred) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "AssertionPosterTransferred")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleAssertionPosterTransferred)
				if err := _Teemodule.contract.UnpackLog(event, "AssertionPosterTransferred", log); err != nil {
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

// ParseAssertionPosterTransferred is a log parse operation binding the contract event 0x002ae90e22e60b8948054f7d1ac3af1d32155f74a4911928decf0c3a6f6351b1.
//
// Solidity: event AssertionPosterTransferred(address dest)
func (_Teemodule *TeemoduleFilterer) ParseAssertionPosterTransferred(log types.Log) (*TeemoduleAssertionPosterTransferred, error) {
	event := new(TeemoduleAssertionPosterTransferred)
	if err := _Teemodule.contract.UnpackLog(event, "AssertionPosterTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
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

// TeemoduleChallengeWindowDurationUpdateIterator is returned from FilterChallengeWindowDurationUpdate and is used to iterate over the raw logs and unpacked data for ChallengeWindowDurationUpdate events raised by the Teemodule contract.
type TeemoduleChallengeWindowDurationUpdateIterator struct {
	Event *TeemoduleChallengeWindowDurationUpdate // Event containing the contract specifics and raw log

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
func (it *TeemoduleChallengeWindowDurationUpdateIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleChallengeWindowDurationUpdate)
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
		it.Event = new(TeemoduleChallengeWindowDurationUpdate)
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
func (it *TeemoduleChallengeWindowDurationUpdateIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleChallengeWindowDurationUpdateIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleChallengeWindowDurationUpdate represents a ChallengeWindowDurationUpdate event raised by the Teemodule contract.
type TeemoduleChallengeWindowDurationUpdate struct {
	NewChallengeWindowDuration uint64
	OldChallengeWindowDuration uint64
	Raw                        types.Log // Blockchain specific contextual infos
}

// FilterChallengeWindowDurationUpdate is a free log retrieval operation binding the contract event 0x75689a8adaf52fab3f618b2698a3868150b33d8ba13b2f1a3ee2bcc310733641.
//
// Solidity: event ChallengeWindowDurationUpdate(uint64 newChallengeWindowDuration, uint64 oldChallengeWindowDuration)
func (_Teemodule *TeemoduleFilterer) FilterChallengeWindowDurationUpdate(opts *bind.FilterOpts) (*TeemoduleChallengeWindowDurationUpdateIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "ChallengeWindowDurationUpdate")
	if err != nil {
		return nil, err
	}
	return &TeemoduleChallengeWindowDurationUpdateIterator{contract: _Teemodule.contract, event: "ChallengeWindowDurationUpdate", logs: logs, sub: sub}, nil
}

// WatchChallengeWindowDurationUpdate is a free log subscription operation binding the contract event 0x75689a8adaf52fab3f618b2698a3868150b33d8ba13b2f1a3ee2bcc310733641.
//
// Solidity: event ChallengeWindowDurationUpdate(uint64 newChallengeWindowDuration, uint64 oldChallengeWindowDuration)
func (_Teemodule *TeemoduleFilterer) WatchChallengeWindowDurationUpdate(opts *bind.WatchOpts, sink chan<- *TeemoduleChallengeWindowDurationUpdate) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "ChallengeWindowDurationUpdate")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleChallengeWindowDurationUpdate)
				if err := _Teemodule.contract.UnpackLog(event, "ChallengeWindowDurationUpdate", log); err != nil {
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

// ParseChallengeWindowDurationUpdate is a log parse operation binding the contract event 0x75689a8adaf52fab3f618b2698a3868150b33d8ba13b2f1a3ee2bcc310733641.
//
// Solidity: event ChallengeWindowDurationUpdate(uint64 newChallengeWindowDuration, uint64 oldChallengeWindowDuration)
func (_Teemodule *TeemoduleFilterer) ParseChallengeWindowDurationUpdate(log types.Log) (*TeemoduleChallengeWindowDurationUpdate, error) {
	event := new(TeemoduleChallengeWindowDurationUpdate)
	if err := _Teemodule.contract.UnpackLog(event, "ChallengeWindowDurationUpdate", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// TeemoduleFundsTransferredIterator is returned from FilterFundsTransferred and is used to iterate over the raw logs and unpacked data for FundsTransferred events raised by the Teemodule contract.
type TeemoduleFundsTransferredIterator struct {
	Event *TeemoduleFundsTransferred // Event containing the contract specifics and raw log

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
func (it *TeemoduleFundsTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TeemoduleFundsTransferred)
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
		it.Event = new(TeemoduleFundsTransferred)
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
func (it *TeemoduleFundsTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TeemoduleFundsTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TeemoduleFundsTransferred represents a FundsTransferred event raised by the Teemodule contract.
type TeemoduleFundsTransferred struct {
	Dest common.Address
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterFundsTransferred is a free log retrieval operation binding the contract event 0x17f29f58ff29e58f40fe3fa963a7469e393593784592e72c3b2355f9199776e0.
//
// Solidity: event FundsTransferred(address dest)
func (_Teemodule *TeemoduleFilterer) FilterFundsTransferred(opts *bind.FilterOpts) (*TeemoduleFundsTransferredIterator, error) {

	logs, sub, err := _Teemodule.contract.FilterLogs(opts, "FundsTransferred")
	if err != nil {
		return nil, err
	}
	return &TeemoduleFundsTransferredIterator{contract: _Teemodule.contract, event: "FundsTransferred", logs: logs, sub: sub}, nil
}

// WatchFundsTransferred is a free log subscription operation binding the contract event 0x17f29f58ff29e58f40fe3fa963a7469e393593784592e72c3b2355f9199776e0.
//
// Solidity: event FundsTransferred(address dest)
func (_Teemodule *TeemoduleFilterer) WatchFundsTransferred(opts *bind.WatchOpts, sink chan<- *TeemoduleFundsTransferred) (event.Subscription, error) {

	logs, sub, err := _Teemodule.contract.WatchLogs(opts, "FundsTransferred")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TeemoduleFundsTransferred)
				if err := _Teemodule.contract.UnpackLog(event, "FundsTransferred", log); err != nil {
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

// ParseFundsTransferred is a log parse operation binding the contract event 0x17f29f58ff29e58f40fe3fa963a7469e393593784592e72c3b2355f9199776e0.
//
// Solidity: event FundsTransferred(address dest)
func (_Teemodule *TeemoduleFilterer) ParseFundsTransferred(log types.Log) (*TeemoduleFundsTransferred, error) {
	event := new(TeemoduleFundsTransferred)
	if err := _Teemodule.contract.UnpackLog(event, "FundsTransferred", log); err != nil {
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
