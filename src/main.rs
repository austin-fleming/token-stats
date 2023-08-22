// use configuration::Settings;
use dotenv::dotenv;
use ethers_providers::StreamExt;
use token_stats::mem_pool::MempoolTransaction;
use token_stats::providers::EthMainnetProvider;
use token_stats::reporting::DiscordReporter;

async fn run() -> Result<(), String> {
    let discord_reporter = DiscordReporter::new().expect("Could not create discord reporter");
    let mainnet_provider = EthMainnetProvider::new()
        .await
        .expect("Could not create ethereum mainnet provider");

    let mut stream = mainnet_provider
        .subscribe_pending_txs()
        .await
        .map_err(|e| format!("Could not create stream: {}", e))?;

    while let Some(tx) = stream.next().await {
        let message = tx.pretty_print();

        println!("{}", message);
        let _ = discord_reporter.send_report(message.as_str()).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    match run().await {
        Ok(_) => println!("Run complete"),
        Err(e) => println!("Run failed: {:?}", e),
    };
}
