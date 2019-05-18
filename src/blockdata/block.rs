use super::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub index: u32,
    pub prev_blockhash: Vec<u8>,
    pub timestamp: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
