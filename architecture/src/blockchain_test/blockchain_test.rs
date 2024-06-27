use crate::blockchain::blockchain::Blockchain;


#[test]
fn test_start_with_genesis() {
    let mut blockchain = Blockchain::new();
    assert_eq!(blockchain.chain.len(), 1);
    assert_eq!(blockchain.chain[0].data, "genesis_data");
    assert_eq!(blockchain.chain[0].last_hash, "genesis_last_hash");
    assert_eq!(blockchain.chain[0].hash, "genesis_hash");
    assert_eq!(blockchain.chain[0].nonce, 0);
    assert_eq!(blockchain.chain[0].difficulty, 0);
}

#[test]
fn test_add_block() {
    let mut blockchain = Blockchain::new();
    let data = String::from("new block data");
    blockchain.add_block(data);
    assert_eq!(blockchain.chain.len(), 2);
    assert_eq!(blockchain.chain[1].data, "new block data");
    assert_eq!(blockchain.chain[1].last_hash, "genesis_hash");
    assert_eq!(blockchain.chain[1].nonce, 0);
    assert_eq!(blockchain.chain[1].difficulty, 0);
}

#[test]
fn test_validates_a_valid_chain() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("new block data"));
    assert_eq!(Blockchain::is_valid_chain(&blockchain.chain), true);
}

#[test]
fn test_invalidates_a_chain_with_a_corrupt_genesis_block() {
    let mut blockchain = Blockchain::new();
    blockchain.chain[0].data = String::from("corrupt data");
    assert_eq!(Blockchain::is_valid_chain(&blockchain.chain), false);
}

#[test]
fn test_invalidates_a_corrupt_chain() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("new block data"));
    blockchain.chain[1].data = String::from("corrupt data");
    assert_eq!(Blockchain::is_valid_chain(&blockchain.chain), false);
}
