use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};



#[derive(Debug)]    
pub struct Blockchain {
     blocks: Vec<Block>
}

impl Blockchain {
    pub fn init() -> Self {
        let current_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = digest(String::from(current_millis.to_string()));
        Blockchain { blocks: vec![
            Block {
                id: 0,
                hash: String::from(hash),
                previous_hash: String::from(""),
                timestamp: current_millis,
                data: String::from(""),
                transactions: vec![],
                nonce: 0,
        
            }
        ] }
    }

    pub fn add(&mut self) -> () {
        let blocks_length: u64 = self.blocks.len() as u64;

        self.blocks.push(Block 
            { 
                id: blocks_length, 
                hash: String::from("pes"), 
                previous_hash: String::from("pes"), 
                timestamp: 2, 
                data: String::from("pes"), 
                transactions: vec![], 
                nonce: 0, 
        });
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

