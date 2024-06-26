use std::fmt;
use chrono::{Local, DateTime, Utc};
use serde_json::{json, Value};
use crate::utils::utils::crypto_hash;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: DateTime<Utc>,
    pub last_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
    pub difficulty: u64,
}

impl Block {
    pub fn new(timestamp: DateTime<Utc>, last_hash: String, hash: String, data: String, nonce: u64, difficulty: u64) -> Block {
        Block {
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty,
        }
    }

    pub fn genesis() -> Block {
        Block {
            timestamp: Local::now().with_timezone(&Utc),
            last_hash: "genesis_last_hash".to_string(),
            hash: "genesis_hash".to_string(),
            data: "genesis_data".to_string(),
            nonce: 0,
            difficulty: 0,
        }
    }

    pub fn mine_block(last_block: &Block, data: &String) -> Block {
        let timestamp = Local::now().with_timezone(&Utc);
        let last_hash = last_block.hash.clone();
        // make hash from all the block properties and nonce
        let hash = crypto_hash(&[
            json!(&timestamp),
            json!(&last_hash),
            json!(&data)
        ]);
        Block {
            timestamp,
            last_hash,
            hash,
            data: data.clone(),
            nonce: 0,
            difficulty: 0,
        }

    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_hash = &self.last_hash[..10];
        let hash = &self.hash[..10];
        write!(f, "Block - \n    Timestamp: {}, \n    Last Hash: {}, \n    Hash: {}, \n    Data: {}, \n    Nonce: {}, \n    Difficulty: {}"
               , self.timestamp, last_hash, hash, self.data, self.nonce, self.difficulty)
    }
}

