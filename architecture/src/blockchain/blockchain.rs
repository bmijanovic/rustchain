use crate::blockchain::block::Block;

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
}


