// use configuration::Settings;
use ethers::providers::{Provider, Ws};
use ethers::types::Transaction;
use ethers_providers::{Middleware, ProviderError, SubscriptionStream};

pub struct EthMainnetProvider {
    ws_provider: Provider<Ws>,
}

impl EthMainnetProvider {
    pub async fn new() -> Result<Self, String> {
        let node_ws_url =
            std::env::var("NODE_ETHMAINNET_WS_URL").expect("'NODE_ETHMAINNET_WS_URL' is not set.");

        let ws_provider: Provider<Ws> = Provider::<Ws>::connect(node_ws_url)
            .await
            .expect("Could not connect to web socket");

        Ok(Self { ws_provider })
    }

    pub async fn subscribe_pending_txs(
        &self,
    ) -> Result<SubscriptionStream<'_, Ws, Transaction>, ProviderError> {
        // client.subscribe(["pendingTransactions"]).await
        self.ws_provider.subscribe_full_pending_txs().await
    }
}
