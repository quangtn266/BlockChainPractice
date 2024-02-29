use crate::{
    model::{
        Address, Block, BlockHash, Blockchain, Transaction, TransactionPool, TransactionVec,
        BLOCK_SUBSIDY,
    }
    util::{
        execution::{sleep_millis, Runnable},
        Context,
    },
};

use anyhow::Result;
use ethereum_types::Address;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MineError {
    #[error("No valid block was mined at index `{0}`")]
    BlockNotMined(u64),
}

pub struct Miner {
    miner_address: Address,
    max_blocks: u64,
    max_nonce: u64,
    tx_waiting_ms: u64,
    blockchain: Blockchain,
    pool: TransactionPool,
    target: BlockHash,
}

impl Runnable for Miner {
    fn run(&self) -> Result<()> {
        self.start()
    }
}

impl Miner {
    pub fn new(context: &Context) -> Miner {

    }
}