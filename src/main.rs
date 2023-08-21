use std::sync::Arc;

// use configuration::Settings;
use dotenv::dotenv;
use ethers::prelude::abigen;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Transaction;
use ethers::{
    core::{
        types::{Address, U256},
        utils::Anvil,
    },
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider, Ws}
};
use ethers_providers::{PubsubClient, StreamExt, SubscriptionStream, ProviderError};
// use ethers_providers::Ws;
// use token_stats::{
//     configuration,
//     repo::{BinanceRepo, EthereumRepo, OnchainRepo},
// };

// async fn test_run(settings: Settings) {
//     let ethereum_repo = EthereumRepo::init(settings.clone().infura);
//     let binance_repo = BinanceRepo::init(settings.clone().binance);
//     let onchain_repo = OnchainRepo::init(settings).expect("Could not init onchain repo");

//     match binance_repo.get_ticker_prices().await {
//         Ok(response) => println!("Ticker count | {}", response.len()),
//         Err(e) => println!("Failed to get ticker prices | {}", e),
//     }

//     match binance_repo.get_trading_pairs().await {
//         Ok(response) => println!("Trading pair count | {}", response.len()),
//         Err(e) => println!("Failed to get trading pairs | {}", e),
//     }

//     match binance_repo
//         .get_trading_pairs_from_coin("ETH".to_string())
//         .await
//     {
//         Ok(response) => println!("ETH pairs | {}", response.len()),
//         Err(e) => println!("Failed to get ETH pairs | {}", e),
//     }

//     match ethereum_repo.get_current_block().await {
//         Ok(response) => println!("Current Block | {}", response.result),
//         Err(e) => println!("Failed to get block | {}", e),
//     }

//     match ethereum_repo.get_current_hashrate().await {
//         Ok(response) => println!("Hashrate | {}", response.result),
//         Err(e) => println!("Failed to get hashrate | {}", e),
//     }

//     println!("Current url | {}", onchain_repo.get_current_url());

//     match onchain_repo.get_current_gas_price().await {
//         Ok(price) => println!("Current gas price | {}", price),
//         Err(e) => println!("Failed to get gas price | {}", e),
//     }

//     match onchain_repo.get_current_chain_id().await {
//         Ok(id) => println!("Current chain id | {}", id),
//         Err(e) => println!("Failed to get chain id | {}", e),
//     }

    
//     let futures = vec![
//         ethereum_repo.get_current_block(),
//         ethereum_repo.get_current_hashrate()
//     ];

//     let _results = future::join_all(futures).await;

    
// }

// abigen!(FactoryContract, "./src/contracts/uniswap/factory.json");

/* 
pub async fn subscribe_pending_txs_with_body(
    client: &Arc<Provider<Ws>>,
) -> Result<SubscriptionStream<'_, Ws, Transaction>, ProviderError>
{
    // this rpc is erigon specific
    client.subscribe(["newPendingTransactionsWithBody"]).await
}
*/

async fn subscribe_pending_txs(client: &Provider<Ws>) -> Result<SubscriptionStream<'_, Ws, Transaction>, ProviderError> {
    // client.subscribe(["pendingTransactions"]).await
    client.subscribe_full_pending_txs().await
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let settings = Settings::load().expect("Failed to load settings");

    // test_run(settings.clone()).await;

    // let url = format!(
    //     "{}{}",
    //     settings.infura.url_mainnet, settings.infura.public_key
    // );

    // let provider: Provider<Http> = Provider::try_from(url).expect("Could not initialize provider");
    // https://hannydevelop.hashnode.dev/building-defi-with-rust-and-ethereum-providers-and-signers-ckppk54ic08fwwhs1edi7h8h1
    // let wallet = LocalWallet::new(&mut rand::thread_rng());
    // let signature = wallet.sign_message("hello world").await;

    let node_ws_url = std::env::var("NODE_ETHMAINNET_WS_URL").expect("'NODE_ETHMAINNET_WS_URL' is not set.");

    let ws_provider: Provider<Ws> = Provider::<Ws>::connect(node_ws_url).await.expect("Could not connect to web socket");
    
    /* let mut stream = ws_provider.subscribe_pending_txs().await.expect("pendingTransactions failed to subscribe");
    while let Some(value) = stream.next().await {
        println!("Data | {}", value);
    } */

    let mut stream = subscribe_pending_txs(&ws_provider).await.expect("Could not subscribe pending txs");
    while let Some(tx) = stream.next().await {
        // println!("\nTX Group:\n{:?}", tx)
        println!(r#"
        ID       | {} 
        Hash     | {}
        Priority | {}
        Max Fee  | {}
        Block    | {}"#, tx.chain_id.unwrap_or_default(), tx.hash, tx.max_priority_fee_per_gas.unwrap_or_default(), tx.max_fee_per_gas.unwrap_or_default(), tx.block_number.unwrap_or_default())
    }
    
    // let client = SignerMiddleware::new(provider, wallet);
    // let client = Arc::new(client);

    // let address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f"
    //     .parse::<Address>()
    //     .expect("Failed to parse address");
    // let factory_contract = FactoryContract::new(address, client);

    // let pairs_length = factory_contract
    //     .all_pairs_length()
    //     .call()
    //     .await
    //     .expect("Could not get pairs length");
    // println!("Pairs length | {}", pairs_length);
    // let pairs = factory_contract
    //     .all_pairs(U256::zero())
    //     .call()
    //     .await
    //     .expect("Could not get pairs.");
    // println!("Pairs | {:?}", pairs);
}
