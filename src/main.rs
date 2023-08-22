// use configuration::Settings;
use dotenv::dotenv;
use ethers_providers::StreamExt;
use token_stats::mem_pool::MempoolTransaction;
use token_stats::providers::EthMainnetProvider;
use token_stats::reporting::DiscordReporter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_reporter = DiscordReporter::new().expect("Could not create discord reporter");
    let mainnet_provider = EthMainnetProvider::new()
        .await
        .expect("Could not create ethereum mainnet provider");

    let mut stream = mainnet_provider
        .subscribe_pending_txs()
        .await
        .expect("Could not subscribe pending txs");

    while let Some(tx) = stream.next().await {
        let message = tx.pretty_print();

        println!("{}", message);
        let _ = discord_reporter.send_report(message.as_str()).await;
    }
}
