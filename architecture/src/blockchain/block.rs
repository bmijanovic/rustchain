use std::fmt;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub last_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
    pub difficulty: u64,
}

impl Block {
    pub fn new(timestamp: u64, last_hash: String, hash: String, data: String, nonce: u64, difficulty: u64) -> Block {
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
            timestamp: 0,
            last_hash: "genesis_last_hash".to_string(),
            hash: "genesis_hash".to_string(),
            data: "genesis_data".to_string(),
            nonce: 0,
            difficulty: 0,
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block - Timestamp: {}, Last Hash: {}, Hash: {}, Data: {}, Nonce: {}, Difficulty: {}", self.timestamp, self.last_hash, self.hash, self.data, self.nonce, self.difficulty)
    }
}

