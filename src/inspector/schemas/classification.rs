use std::str::FromStr;

use ethers::types::{Address, U256};

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionClassification {
    Unknown,
    Swap,
    Transfer,
    Liquidate,
    Seize,
    PunkBid,
    PunkAcceptBid,
    NftTrade,
}

impl FromStr for TransactionClassification {
    type Err = ();

    fn from_str(value: &str) -> Result<TransactionClassification, Self::Err> {
        match value {
            "swap" => Ok(TransactionClassification::Swap),
            "transfer" => Ok(TransactionClassification::Transfer),
            "liquidate" => Ok(TransactionClassification::Liquidate),
            "seize" => Ok(TransactionClassification::Seize),
            "punk_bid" => Ok(TransactionClassification::PunkBid),
            "punk_accept_bid" => Ok(TransactionClassification::PunkAcceptBid),
            "nft_trade" => Ok(TransactionClassification::NftTrade),
            _ => Ok(TransactionClassification::Unknown),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    UniswapV2,
    UniswapV3,
    Cryptopunks,
    Sushiswap,
    Opensea,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassifierSpec {
    abi_name: String,
    protocol: Protocol,
    valid_contract_addresses: Vec<Address>,
    classifications: Vec<TransactionClassification>,
}

impl ClassifierSpec {
    pub fn new(
        abi_name: String,
        protocol: Protocol,
        valid_contract_addresses: Vec<Address>,
    ) -> Result<Self, String> {
        Ok(Self {
            abi_name,
            protocol,
            valid_contract_addresses,
            classifications: Vec::with_capacity(1), // TODO: ensure this is what should be expected
        })
    }

    pub fn add_classification(&mut self, classification: TransactionClassification) {
        self.classifications.push(classification);
    }
}

// TODO: ERC-721 or ERC-1155?
pub struct PunkSnipe {
    block_number: U256,
    trace_address: Vec<Address>,
    from_address: Address,
    punk_index: U256,
    min_acceptance_price: U256,
    acceptance_price: U256,
}

pub enum TraceVariant {
    Call,
    Create,
    DelegateCall,
    Reward,
    Suicide,
}
