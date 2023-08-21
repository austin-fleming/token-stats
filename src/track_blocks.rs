use ethers::types::U64;
use tokio::time::sleep;

/* pub async fn track_blocks(
    http_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) {
    let mut previous_block = U64::zero();

    loop {
        let current_block = http_provider.get_block_number().await;

        if current_block > previous_block {
            previous_block = current_block;

            println!("{:?}", block);

            sleep(Duration::from_millis(100)).await;
        }
    }
}
 */