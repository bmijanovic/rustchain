use crate::blockchain::blockchain::{Blockchain};
use crate::utils::config::DIFFICULTY;
use crate::wallet::transaction::Transaction;
use crate::wallet::wallet::Wallet;

#[test]
fn test_start_with_genesis() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.chain.len(), 1);
    assert_eq!(blockchain.chain[0].last_hash, "genesis_last_hash");
    assert_eq!(blockchain.chain[0].hash, "genesis_hash");
    assert_eq!(blockchain.chain[0].nonce, 0);
    assert_eq!(blockchain.chain[0].difficulty, DIFFICULTY);
}

#[test]
fn test_add_block() {
    let mut blockchain = Blockchain::new();
    let data = vec![];
    blockchain.add_block(data);
    assert_eq!(blockchain.chain.len(), 2);
    assert_eq!(blockchain.chain[1].last_hash, "genesis_hash");
}

#[test]
fn test_validates_a_valid_chain() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(vec![]);
    assert_eq!(Blockchain::is_valid_chain(&blockchain.chain), true);
}

#[test]
fn test_invalidates_a_chain_with_a_corrupt_genesis_block() {
    let mut blockchain = Blockchain::new();
    blockchain.chain[0].data = vec![Transaction::reward_transaction(&Wallet::new(), &Wallet::blockchain_wallet())];
    assert_eq!(Blockchain::is_valid_chain(&blockchain.chain), false);
}

#[test]
fn test_invalidates_a_corrupt_chain() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(vec![]);
    blockchain.chain[1].data = vec![Transaction::reward_transaction(&Wallet::new(), &Wallet::blockchain_wallet())];
    assert_eq!(Blockchain::is_valid_chain(&blockchain.chain), false);
}

#[test]
fn test_replaces_chain_with_a_valid_chain(){
    let mut blockchain = Blockchain::new();
    let mut blockchain2 = Blockchain::new();
    blockchain2.add_block(vec![Transaction::reward_transaction(&Wallet::new(), &Wallet::blockchain_wallet())]);
    blockchain.replace_chain(blockchain2.chain.clone());
    assert_eq!(blockchain.chain, blockchain2.chain);
}

#[test]
fn test_does_not_replace_chain_with_one_of_equal_or_less_length(){
    let mut blockchain = Blockchain::new();
    let blockchain2 = Blockchain::new();
    blockchain.add_block(vec![Transaction::reward_transaction(&Wallet::new(), &Wallet::blockchain_wallet())]);
    blockchain.replace_chain(blockchain2.chain.clone());
    assert_ne!(blockchain.chain, blockchain2.chain);
}
