#![allow(clippy::large_enum_variant)]

use ethereum::{Header, Transaction};
use ethereum_types::{Address, H256, U256};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForkData {
    pub genesis: H256,
    pub forks: BTreeSet<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockBody {
    pub transactions: Vec<Transaction>,
    pub ommers: Vec<Header>,
}

#[tarpc::service]
pub trait EthApi {
    async fn forks() -> Result<ForkData, String>;

    async fn best_block() -> Result<(H256, u64), String>;
    async fn canonical_hash(number: u64) -> Result<Option<H256>, String>;

    async fn header(hash: H256) -> Result<Option<Header>, String>;
    async fn body(hash: H256) -> Result<Option<BlockBody>, String>;
    async fn total_difficulty(hash: H256) -> Result<Option<U256>, String>;

    async fn balance(block: H256, address: Address) -> Result<Option<U256>, String>;
    async fn nonce(block: H256, address: Address) -> Result<Option<u64>, String>;
}
