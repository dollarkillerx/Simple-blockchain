use core::blockchain;
fn main() {
    let mut block = blockchain::BlockChain::new();
    block.add("记录区块1个消费".to_string());
    block.add("记录区块2个消费".to_string());
    block.add("记录区块3个消费".to_string());

    for i in block.blocks {
        println!("========================");
        println!("{:#?}",i);
        println!();
    }
}
