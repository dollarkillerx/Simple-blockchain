use super::block;
use crate::block::Block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>
}

impl BlockChain {
    pub fn add(&mut self,data: String) {
        let per_block = &self.blocks[self.blocks.len() -1];
        let block = Block::new(data,per_block.hash.clone());
        self.blocks.push(block);
    }

    // 创世块
    fn new_genesis_block(&mut self) {
        self.blocks.push(Block::new("genesis block".to_string(),"".to_string()))
    }

    pub fn new() -> BlockChain {
        let mut block = BlockChain{
            blocks: vec![]
        };
        block.new_genesis_block();
        block
    }
}
