use super::block::Block;
use super::block::BlockHeader;
use super::transaction::Transaction;
use serde_json;
use sha2::{Digest, Sha256};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn genesis() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };
        blockchain.chain.push(Block {
            header: BlockHeader {
                index: 0,
                prev_blockhash: Vec::new(),
                timestamp: SystemTime::now(),
            },
            transactions: Vec::new(),
        });
        return blockchain;
    }

    // TODO
    //  :param proof: <int> The proof given by the Proof of Work algorithm
    //  :param previous_hash: (Optional) <str> Hash of previous Block
    pub fn create_block(&mut self) -> Block {
        let json_block = serde_json::to_string(&self.chain[self.chain.len() - 1]).unwrap();
        let block = Block {
            header: BlockHeader {
                index: self.chain.len() as u32,
                prev_blockhash: Sha256::digest(json_block.as_bytes()).as_slice().to_vec(), // TODO store hash as a string, the hasher.result() returns a Generic byte array
                timestamp: SystemTime::now(),
            },
            transactions: self.current_transactions.clone(),
            //proof: proof, // TODO PoW
        };
        // Reset the current list of transactions
        self.current_transactions = Vec::new();
        self.chain.push(block);
        return self.chain.last().unwrap().clone();
    }

    // return: <int> The index of the Block that will hold this transaction
    pub fn add_transaction(&mut self, _tx: Transaction) -> u32 {
        self.current_transactions.push(_tx);
        return self.chain.len() as u32;
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        return self.chain.clone();
    }
}
