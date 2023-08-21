use ethers::types::{Address, U256};

#[derive(Debug, Clone, PartialEq)]
pub enum ContractInterface {
    UniswapV2,
    UniswapV3,
    SpookySwap,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pool {
    pub pool_address: Address,
    pub token_a_address: Address,
    pub token_b_address: Address,
    pub swap_fee: U256,
    pub contract_interface: ContractInterface,
}

impl Pool {
    fn new(
        pool_address: Address,
        token_a_address: Address,
        token_b_address: Address,
        swap_fee: U256,
        contract_interface: ContractInterface,
    ) -> Result<Pool, String> {
        Ok(Self {
            pool_address,
            token_a_address,
            token_b_address,
            swap_fee,
            contract_interface,
        })
    }
}
