package config

type Config struct {
	// Translator
	SettlementChainAddr string
	SequencingChainAddr string

	Port int32
}

func Init() *Config {
	// TODO [SEQ-104]: Load from env and validate
	return &Config{
		Port: 8546, //nolint:mnd // TODO

		// Translator
		SettlementChainAddr: "https://sepolia.base.org",
		SequencingChainAddr: "https://sepolia.base.org",
	}
}
