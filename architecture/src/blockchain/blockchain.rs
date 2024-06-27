use serde_json::json;
use crate::blockchain::block::Block;
use crate::utils::utils::crypto_hash;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::genesis()],
        }
    }
    
    pub fn add_block(&mut self, data: String) -> Block {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::mine_block(&last_block, data);
        self.chain.push(new_block.clone());
        new_block
    }
    
    pub fn is_valid_chain(chain: &Vec<Block>) -> bool {
        if chain[0] != Block::genesis() {
            return false;
        }
        
        for i in 1..chain.len() {
            let block = &chain[i];
            let last_block = &chain[i - 1];
            
            if block.last_hash != last_block.hash || block.hash != Block::block_hash(block){
                return false;
            }
        }
        
        true
    }
}


