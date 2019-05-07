use sha2::Sha256;
use super::transaction::Transaction;

#[derive(/*Copy, PartialEq, Eq,*/ Clone, Debug)]    // TODO implement PartialEq, Eq for Sha256
pub struct BlockHeader {
    pub index: u32,
    pub prev_blockhash: Sha256,
    pub timestamp: u32,
}

#[derive(/*PartialEq, Eq,*/ Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>
}
