use reqwest::Client;
use serde::Deserialize;

use crate::{
    configuration::BinanceSettings,
    helpers::{create_hmac264_signature, get_millisecond_timestamp},
};

#[derive(Clone, Debug)]
pub struct BinanceRepo {
    pub client: Client,
    pub settings: BinanceSettings,
}

impl BinanceRepo {
    pub fn init(settings: BinanceSettings) -> BinanceRepo {
        Self {
            client: Client::new(),
            settings,
        }
    }

    pub async fn get_ticker_prices(&self) -> Result<Vec<TickerPriceDTO>, String> {
        let endpoint = "https://api.binance.us/api/v3/ticker/price";

        let response = self
            .client
            .get(endpoint)
            .send()
            .await
            .expect("Failed to fetch ticker prices");

        let prices: Vec<TickerPriceDTO> = response.json().await.expect("Could not deserialize");

        Ok(prices)
    }

    pub async fn get_trading_pairs(&self) -> Result<Vec<TradingPairDTO>, String> {
        let endpoint = format!("{}/sapi/v1/otc/coinPairs", self.settings.url);

        let timestamp_ms = get_millisecond_timestamp().expect("Failed to create timestamp");
        let signature_data = format!("timestamp={}", timestamp_ms);
        let signature =
            create_hmac264_signature(self.settings.private_key.clone(), signature_data)?;

        let response = self
            .client
            .get(endpoint)
            // .form(&params)
            .query(&[
                ("timestamp", timestamp_ms.to_string()),
                ("signature", signature),
            ])
            .header("X-MBX-APIKEY", self.settings.public_key.clone())
            .send()
            .await
            .expect("Failed to fetch");

        if response.status().as_str() != "200" {
            return Err(format!("Error code {}", response.status()));
        }

        let prices: Vec<TradingPairDTO> = response
            .json()
            .await
            .expect("Could not deserialize trading pair list");

        Ok(prices)
    }

    pub async fn get_trading_pairs_from_coin(
        &self,
        coin: String,
    ) -> Result<Vec<TradingPairDTO>, String> {
        let timestamp_ms = get_millisecond_timestamp().expect("Failed to create timestamp");
        let signature_data = format!("timestamp={}&fromCoin={}", timestamp_ms, coin);

        let signature =
            create_hmac264_signature(self.settings.private_key.clone(), signature_data)?;

        // let mut params = HashMap::new();
        // params.insert("timestamp", timestamp_ms.to_string());
        // params.insert("signature", signature);

        let response = self
            .client
            .get("https://api.binance.us/sapi/v1/otc/coinPairs")
            // .form(&params)
            .query(&[
                ("timestamp", timestamp_ms.to_string()),
                ("signature", signature),
                ("fromCoin", coin),
            ])
            .header("X-MBX-APIKEY", self.settings.public_key.clone())
            .send()
            .await
            .expect("Failed to fetch");

        if response.status().as_str() != "200" {
            return Err(format!("Error code {}", response.status()));
        }

        let prices: Vec<TradingPairDTO> = response.json().await.expect("Could not deserialize");

        Ok(prices)
    }
}

#[derive(Deserialize, Debug)]
pub struct TickerPriceDTO {
    pub symbol: String,
    pub price: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradingPairDTO {
    pub from_coin: String,
    pub to_coin: String,
    pub from_coin_min_amount: String,
    pub from_coin_max_amount: String,
    pub to_coin_min_amount: String,
    pub to_coin_max_amount: String,
}
