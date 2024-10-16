package interfaces

type IConfig interface {
	SettlementChainAddr() string
	SequencingChainAddr() string
	MetaBasedChainAddr() string
	LogLevel() string
	Port() int
	FrameSize() int
	Pretty() bool

	SequencingContractAddress() string
	BatcherAddress() string
	BatchInboxAddress() string
	BatcherPrivateKey() string
	SettlementChainID() int64
	SettlementStartBlock() int
	SequencingStartBlock() int
	SequencePerSettlementBlock() int
}
