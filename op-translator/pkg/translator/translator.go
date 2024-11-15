package translator

import (
	"context"

	"github.com/SyndicateProtocol/metabased-rollup/op-translator/internal/config"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/backfill"
	rpc "github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/rpc-clients"
	"github.com/SyndicateProtocol/metabased-rollup/op-translator/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	ethtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/rs/zerolog/log"
)

type IRPCClient interface {
	CloseConnection()
	GetBlockByNumber(ctx context.Context, number string, withTransactions bool) (types.Block, error)
	GetBlockByHash(ctx context.Context, hash common.Hash, withTransactions bool) (types.Block, error)
	GetReceiptsByBlocks(ctx context.Context, blocks []*types.Block) ([]*ethtypes.Receipt, error)
	SimulateTransactions(ctx context.Context, simulationRequest rpc.SimulationRequest, blockParameter string) error
	AsEthClient() rpc.IETHClient
}

type IBatchProvider interface {
	GetBatch(ctx context.Context, block types.Block) (*types.Batch, error)
	FilterTransactionsStateful(rawTxs []hexutil.Bytes, parsedTxs []rpc.Transaction) (rawFilteredTxStateful []hexutil.Bytes, removedCountStateful int)
	Close()
}

// guarantees that the IRPCClient interface is implemented by RPCClient
var _ IRPCClient = (*rpc.RPCClient)(nil)

type OPTranslator struct {
	SettlementChain     IRPCClient
	BatchProvider       IBatchProvider
	BackfillProvider    *backfill.BackfillProvider
	Signer              Signer
	BatcherInboxAddress common.Address
	BatcherAddress      common.Address
}

func Init(cfg *config.Config) *OPTranslator {
	settlementChain, err := rpc.Connect(cfg.SettlementChainRPCURL)
	if err != nil {
		log.Panic().Err(err).Msg("Failed to initialize settlement chain")
	}

	metaBasedBatchProvider := InitMetaBasedBatchProvider(cfg)
	signer := NewSigner(cfg)
	backfillProvider := backfill.NewBackfillerProvider(cfg)

	return &OPTranslator{
		SettlementChain:     settlementChain,
		BatcherInboxAddress: common.HexToAddress(cfg.BatchInboxAddress),
		BatcherAddress:      common.HexToAddress(cfg.BatcherAddress),
		BatchProvider:       metaBasedBatchProvider,
		BackfillProvider:    backfillProvider,
		Signer:              *signer,
	}

}

func (t *OPTranslator) GetBlockByNumber(ctx context.Context, blockNumber string, transactionDetailFlag bool) (types.Block, error) {
	// transactions := []rpc.Transaction{
	// 	{
	// 		From:                 "0xBA401CdaC1A3b6AEeDe21c9C4a483be6C29F88C5",
	// 		To:                   "0x52A4380F691E71ff0015352AB1a450a1dfb689b9",
	// 		Value:                "0xDE0B6B3A7640000",
	// 		Data:                 "0x",
	// 		Nonce:                "0x59",
	// 		Gas:                  "0xC350",
	// 		MaxFeePerGas:         "0x83F9398D9",
	// 		MaxPriorityFeePerGas: "0x2540BE400",
	// 	},
	// 	{
	// 		From:                 "0xBA401CdaC1A3b6AEeDe21c9C4a483be6C29F88C5",
	// 		To:                   "0x52A4380F691E71ff0015352AB1a450a1dfb689b9",
	// 		Value:                "0xDE0B6B3A7640000",
	// 		Data:                 "0x",
	// 		Nonce:                "0x59", //5A is valid nonce but
	// 		Gas:                  "0xC350",
	// 		MaxFeePerGas:         "0x83F9398D9", //"0x83F9398D9",
	// 		MaxPriorityFeePerGas: "0x0",         //"0x2540BE400",
	// 	},
	// {
	// 	From:                 "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
	// 	To:                   "0x52A4380F691E71ff0015352AB1a450a1dfb689b9",
	// 	Value:                "0xDE0B6B3A7640000",
	// 	Data:                 "0x",
	// 	Nonce:                "0x1",
	// 	Gas:                  "0xC350",
	// 	MaxFeePerGas:         "0x2540BE400",
	// 	MaxPriorityFeePerGas: "0x2540BE400",
	// },
	// }

	// rawTxns := []hexutil.Bytes{hexutil.Bytes("rawtx1"), hexutil.Bytes("rawtx2")}
	// , hexutil.Bytes("rawtx2"), hexutil.Bytes("rawtx3")}
	// raw, removed := t.BatchProvider.FilterTransactionsStateful(rawTxns, transactions)
	// log.Debug().Msgf("raw: %v, removed: %v", raw, removed)

	log.Debug().Msg("-- HIT eth_getBlockByNumber")
	block, err := t.SettlementChain.GetBlockByNumber(ctx, blockNumber, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by number")
		return nil, err
	}
	if !transactionDetailFlag {
		return block, nil
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) GetBlockByHash(ctx context.Context, blockHash common.Hash, transactionDetailFlag bool) (types.Block, error) {
	log.Debug().Msg("-- HIT eth_getBlockByHash")
	block, err := t.SettlementChain.GetBlockByHash(ctx, blockHash, transactionDetailFlag)
	if err != nil {
		log.Error().Err(err).Msg("Failed to get block by hash")
		return nil, err
	}
	if !transactionDetailFlag {
		return block, nil
	}
	return t.translateBlock(ctx, block)
}

func (t *OPTranslator) Close() {
	t.SettlementChain.CloseConnection()
	t.BatchProvider.Close()
}

func (t *OPTranslator) getFrames(ctx context.Context, block types.Block) ([]*types.Frame, error) {
	if t.BackfillProvider.IsBlockInBackfillingWindow(block) {
		return t.BackfillProvider.GetBackfillFrames(ctx, block)
	} else {
		batch, err := t.BatchProvider.GetBatch(ctx, block)
		if err != nil {
			return nil, err
		}
		return batch.GetFrames(config.MaxFrameSize)
	}
}

func (t *OPTranslator) translateBlock(ctx context.Context, block types.Block) (types.Block, error) {
	if block.IsEmpty() {
		return nil, nil
	}

	frames, err := t.getFrames(ctx, block)
	if err != nil {
		return nil, err
	}

	blockNumHex, err := block.GetBlockNumberHex()
	if err != nil {
		return nil, err
	}

	if len(frames) == 0 {
		log.Debug().Msgf("No frames to translate, block number (hex): %s", blockNumHex)
		return block, nil
	}

	data, err := types.ToData(frames)
	if err != nil {
		return nil, err
	}

	blockHash, err := block.GetBlockHash()
	if err != nil {
		return nil, err
	}

	tx := types.NewBatcherTx(blockHash, blockNumHex, t.BatcherAddress.String(), t.BatcherInboxAddress.String(), data, t.Signer.ChainID())

	signedTxn, err := t.Signer.Sign(&tx)
	if err != nil {
		return nil, err
	}

	err = block.AppendTransaction(signedTxn)
	if err != nil {
		return nil, err
	}

	return block, nil
}

func ShouldTranslate(method string) bool {
	switch method {
	case "eth_getBlockByNumber":
		return true
	case "eth_getBlockByHash":
		return true
	}

	return false
}
