#[derive(Clone, Debug)]
pub struct InfuraSettings {
    pub url_mainnet: String,
    pub public_key: String,
}

impl InfuraSettings {
    pub fn load() -> Result<Self, String> {
        let public_key = std::env::var("INFURA_KEY_PUBLIC").expect("INFURA_KEY_PUBLIC must be set");

        Ok(Self {
            url_mainnet: "https://mainnet.infura.io/v3/".to_string(),
            public_key,
        })
    }
}

#[derive(Clone, Debug)]
pub struct BinanceSettings {
    pub url: String,
    pub public_key: String,
    pub private_key: String,
}

impl BinanceSettings {
    pub fn load() -> Result<Self, String> {
        let private_key = std::env::var("BINANCE_KEY_READ_PRIVATE")
            .expect("BINANCE_KEY_READ_PRIVATE must be set");
        let public_key =
            std::env::var("BINANCE_KEY_READ_PUBLIC").expect("BINANCE_KEY_READ_PUBLIC must be set");

        Ok(Self {
            url: "https://api.binance.us".to_string(),
            private_key,
            public_key,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub binance: BinanceSettings,
    pub infura: InfuraSettings,
}

impl Settings {
    pub fn load() -> Result<Self, String> {
        Ok(Self {
            binance: BinanceSettings::load()?,
            infura: InfuraSettings::load()?,
        })
    }
}
