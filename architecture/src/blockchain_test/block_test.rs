use crate::blockchain::block::Block;

#[test]
fn test_block() {
    let block = Block::new(1, "last_hash".to_string(), "hash".to_string(), "data".to_string(), 1, 1);
    assert_eq!(block.timestamp, 1);
    assert_eq!(block.last_hash, "last_hash");
    assert_eq!(block.hash, "hash");
    assert_eq!(block.data, "data");
    assert_eq!(block.nonce, 1);
    assert_eq!(block.difficulty, 1);
}


#[test]
fn test_genesis_block() {
    let timestamp = 0;
    let last_hash = String::from("genesis_last_hash");
    let hash = String::from("genesis_hash");
    let data = String::from("genesis_data");
    let nonce = 0;
    let difficulty = 0;

    let genesis_block = Block::genesis();

    // Validating genesis block properties
    assert_eq!(genesis_block.timestamp, timestamp);
    assert_eq!(genesis_block.last_hash, last_hash);
    assert_eq!(genesis_block.hash, hash);
    assert_eq!(genesis_block.data, data);
    assert_eq!(genesis_block.nonce, nonce);
    assert_eq!(genesis_block.difficulty, difficulty);
}