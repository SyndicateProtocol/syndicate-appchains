use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone)]
pub enum Profile {
    Mainnet,
    Testnet,
    Devnet,
    Localnet,
}

impl Profile {
    pub fn to_env_filename(&self) -> &'static str {
        match &self {
            Profile::Mainnet => "mainnet.env",
            Profile::Testnet => "testnet.env",
            Profile::Devnet => "devnet.env",
            Profile::Localnet => "localnet.env",
        }
    }
}
