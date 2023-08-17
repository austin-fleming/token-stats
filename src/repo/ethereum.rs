use serde::Deserialize;
use std::collections::HashMap;

use reqwest::Client;

use crate::configuration::InfuraSettings;

#[derive(Clone, Debug)]
pub struct EthereumRepo {
    pub client: Client,
    pub settings: InfuraSettings,
}

impl EthereumRepo {
    pub fn init(settings: InfuraSettings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }

    pub async fn query_eth(&self, method: String) -> Result<RpcResponse<String>, reqwest::Error> {
        let endpoint = format!("{}{}", self.settings.url_mainnet, self.settings.public_key);

        let mut body = HashMap::new();
        body.insert("jsonrpc", "2.0");
        body.insert("id", "1");
        body.insert("method", &method);
        // body.insert("params",: vec![]);

        let response = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let outcome: RpcResponse<String> = response.json().await?;

        Ok(outcome)
    }

    pub async fn get_current_block(&self) -> Result<RpcResponse<String>, reqwest::Error> {
        self.query_eth("eth_blockNumber".to_string()).await
    }

    pub async fn get_current_hashrate(&self) -> Result<RpcResponse<String>, reqwest::Error> {
        self.query_eth("eth_hashrate".to_string()).await
    }
}

//{\"jsonrpc\":\"2.0\",\"id\":\"1\",\"result\":\"0x1119ec1\"}"
#[derive(Deserialize, Debug, Clone)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T,
}
