package config

type Config struct {
	Port int

	// Translator
	SettlementChainAddr string
	SequencingChainAddr string
}

func Init() *Config {
	// TODO: Load from env and validate
	return &Config{
		Port: 8546,

		// Translator
		SettlementChainAddr: "https://sepolia.base.org",
		SequencingChainAddr: "https://sepolia.base.org",
	}
}
