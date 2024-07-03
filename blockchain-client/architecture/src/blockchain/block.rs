use std::fmt;

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;
use serde_json::json;

use crate::utils::utils::crypto_hash;

use crate::utils::config::{DIFFICULTY, MINE_RATE};
use crate::wallet::transaction::Transaction;

#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    pub timestamp: DateTime<Utc>,
    pub last_hash: String,
    pub hash: String,
    pub data: Vec<Transaction>,
    pub nonce: u64,
    pub difficulty: u64,
}

impl Block {
    pub fn new(timestamp: DateTime<Utc>, last_hash: String, hash: String, data: Vec<Transaction>, nonce: u64, difficulty: u64) -> Block {
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
            data: Vec::new(),
            nonce: 0,
            difficulty: DIFFICULTY,
        }
    }

    pub fn mine_block(last_block: &Block, data: Vec<Transaction>) -> Block {
        let mut timestamp = Local::now().with_timezone(&Utc);
        let last_hash = last_block.hash.clone();
        let mut nonce = 0;
        let mut difficulty = last_block.difficulty;

        // do-while loop
        let mut hash = crypto_hash(&[
            json!(&timestamp),
            json!(&last_hash),
            json!(&data),
            json!(&nonce),
            json!(&difficulty)
        ]);

        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            nonce += 1;
            timestamp = Local::now().with_timezone(&Utc);
            difficulty = Block::adjust_difficulty(last_block, timestamp);
            hash = crypto_hash(&[
                json!(&timestamp),
                json!(&last_hash),
                json!(&data),
                json!(&nonce),
                json!(&difficulty)
            ]);
        }



        Block {
            timestamp,
            last_hash,
            hash,
            data: data.clone(),
            nonce,
            difficulty,
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

    pub(crate) fn adjust_difficulty(last_block: &Block, timestamp: DateTime<Utc>) -> u64 {
        let difficulty = last_block.difficulty;
        let time_diff = timestamp.timestamp() - last_block.timestamp.timestamp();
        if difficulty <= 1 {
            return 1;
        }
        return if time_diff < MINE_RATE as i64 {
            difficulty + 1
        } else {
            difficulty - 1
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_hash = &self.last_hash[..10];
        let hash = &self.hash[..10];
        write!(f, "Block - \n    Timestamp: {}, \n    Last Hash: {}, \n    Hash: {}, \n    Data: {:?}, \n    Nonce: {}, \n    Difficulty: {}"
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