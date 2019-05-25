use super::block::Block;
use super::block::BlockHeader;
use super::transaction::Transaction;
use data_encoding::HEXLOWER;
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
                prev_blockhash: String::new(),
                timestamp: SystemTime::now(),
            },
            transactions: Vec::new(),
            proof: 100,
        });
        return blockchain;
    }

    pub fn validate_proof(last_proof: i32, proof: i32) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = HEXLOWER.encode(Sha256::digest(guess.as_bytes()).as_slice());
        return guess_hash.starts_with("0000");
    }

    // TODO
    //  :param proof: <int> The proof given by the Proof of Work algorithm
    //  :param previous_hash: (Optional) <str> Hash of previous Block
    pub fn create_block(&mut self, proof: i32) -> Block {
        let json_block = serde_json::to_vec(&self.chain[self.chain.len() - 1]).unwrap();
        let block = Block {
            header: BlockHeader {
                index: self.chain.len() as u32,
                prev_blockhash: HEXLOWER.encode(Sha256::digest(json_block.as_slice()).as_slice()),
                timestamp: SystemTime::now(),
            },
            transactions: self.current_transactions.clone(),
            proof: proof,
        };
        // Reset the current list of transactions
        self.current_transactions = Vec::new();
        println!("CREATED BLOCK: {:#?}", block);
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

    pub fn get_last_block(&self) -> Block {
        return self.chain.last().unwrap().clone();
    }

    pub fn proof_of_work(&self, last_proof: i32) -> i32 {
        let mut proof = 0;
        while !Blockchain::validate_proof(last_proof, proof) {
            proof += 1;
        }
        return proof;
    }
}
