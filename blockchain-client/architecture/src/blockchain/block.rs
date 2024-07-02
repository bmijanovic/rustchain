use std::fmt;

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;
use serde_json::json;

use crate::utils::utils::crypto_hash;

#[derive(Debug, Deserialize, Clone)]
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

    pub fn mine_block(last_block: &Block, data: String) -> Block {
        let timestamp = Local::now().with_timezone(&Utc);
        let last_hash = last_block.hash.clone();
        // make hash from all the block properties and nonce
        let hash = crypto_hash(&[
            json!(&timestamp),
            json!(&last_hash),
            json!(&data),
            json!(&last_block.nonce),
            json!(&last_block.difficulty)
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

    pub fn block_hash(block: &Block) -> String {
        crypto_hash(&[
            json!(&block.timestamp),
            json!(&block.last_hash),
            json!(&block.data),
            json!(&block.nonce),
            json!(&block.difficulty)
        ])
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_hash = &self.last_hash[..10];
        let hash = &self.hash[..10];
        write!(f, "Block - \n    Timestamp: {}, \n    Last Hash: {}, \n    Hash: {}, \n    Data: {}, \n    Nonce: {}, \n    Difficulty: {}"
               , self.timestamp.to_rfc2822(), last_hash, hash, self.data, self.nonce, self.difficulty)
    }
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Block", 6)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("last_hash", &self.last_hash)?;
        state.serialize_field("hash", &self.hash)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("difficulty", &self.difficulty)?;
        state.end()
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.last_hash == other.last_hash &&
            self.hash == other.hash &&
            self.data == other.data &&
            self.nonce == other.nonce &&
            self.difficulty == other.difficulty
    }
}