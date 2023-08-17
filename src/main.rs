use dotenv::dotenv;
use token_stats::{
    configuration,
    repo::{BinanceRepo, EthereumRepo},
};
use configuration::Settings;

async fn test_run(settings: Settings) {
    let ethereum_repo = EthereumRepo::init(settings.infura);
    let binance_repo = BinanceRepo::init(settings.binance);

    match binance_repo.get_ticker_prices().await {
        Ok(response) => println!("Ticker count | {}", response.len()),
        Err(e) => println!("Failed to get ticker prices | {}", e),
    }

    match binance_repo.get_trading_pairs().await {
        Ok(response) => println!("Trading pair count | {}", response.len()),
        Err(e) => println!("Failed to get trading pairs | {}", e),
    }

    match binance_repo
        .get_trading_pairs_from_coin("ETH".to_string())
        .await
    {
        Ok(response) => println!("ETH pairs | {}", response.len()),
        Err(e) => println!("Failed to get ETH pairs | {}", e),
    }

    match ethereum_repo.get_current_block().await {
        Ok(response) => println!("Current Block | {}", response.result),
        Err(e) => println!("Failed to get block | {}", e),
    }

    match ethereum_repo.get_current_hashrate().await {
        Ok(response) => println!("Hashrate | {}", response.result),
        Err(e) => println!("Failed to get hashrate | {}", e),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let settings = Settings::load().expect("Failed to load settings");

    test_run(settings).await;
}
