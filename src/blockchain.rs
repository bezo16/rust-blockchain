use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};



#[derive(Debug)]    
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn init() -> Self {
        Blockchain { blocks: vec![
            Block {
                id: 0,
                hash: String::from(""),
                previous_hash: String::from(""),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                data: String::from(""),
                transactions: vec![],
                nonce: 0,
        
            }
        ] }
    }
}

#[derive(Debug)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u128,
    pub data: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u128,
}

#[derive(Debug)]
pub struct Transaction {
}

