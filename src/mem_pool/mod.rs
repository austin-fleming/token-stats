use ethers::types::{Transaction};

pub trait MempoolTransaction {
    fn pretty_print(&self) -> String;
}

impl MempoolTransaction for Transaction {
    fn pretty_print(&self) -> String {
        format!(
            "\
            \nChain ID | {}\
            \nHash     | {}\
            \nPriority | {}\
            \nMax Fee  | {}\
            \nBlock    | {}\
            ",
            self.chain_id.unwrap_or_default(),
            self.hash,
            self.max_priority_fee_per_gas.unwrap_or_default(),
            self.max_fee_per_gas.unwrap_or_default(),
            self.block_number.unwrap_or_default()
        )
    }
}
