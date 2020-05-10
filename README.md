# Simple-blockchain
Rust实现简单的区块链

### 区块结构
- 区块头
```rust
pub struct BlockHeader {
    pub time: i64,        // time stamp
    pub tx_hash: String,  // transaction merkle hash
    pub pre_hash: String, // hash of the pre block 
}
```
- 区块体
```rust
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,         // hash of the block header
    pub data: String,         // transactions data 
}
```
- 区块链 结构
```rust
pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}
```
### 参考文献
- 《Bitcoin: A Peer-to-Peer Electronic Cash System》
- https://github.com/rust-bitcoin/rust-bitcoin
