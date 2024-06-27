use crate::blockchain::blockchain::Blockchain;

// make me tests:
// before each test, create a new blockchain
// 1. test that a blockchain is created with a genesis block
// 2. test that a block is added to the blockchain

#[test]
fn test_blockchain() {
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

