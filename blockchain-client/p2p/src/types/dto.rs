use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct BlockchainData {
    pub data: String,
}


#[derive(Serialize, Deserialize)]
pub struct TransactionData {
    pub recipient: String,
    pub amount: u64,
}