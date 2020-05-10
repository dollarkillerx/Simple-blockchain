use serde::{Serialize,Deserialize};
use chrono::prelude::*;
use utils::coder;

#[derive(Serialize,Deserialize,Debug)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new(data: String,pre_hash: String) -> Block {
        let transactions = coder::marshal(&data); // 记录一笔交易
        let tx_hash = coder::hash(&transactions[..]);
        let time = Utc::now().timestamp();

        let mut block = Block{
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash
            },
            hash: "".to_string(),
            data
        };

        block.set_hash();
        block
    }

    pub fn set_hash(&mut self) {
        self.hash = coder::hash(&coder::marshal(&self.header)[..])
    }
}