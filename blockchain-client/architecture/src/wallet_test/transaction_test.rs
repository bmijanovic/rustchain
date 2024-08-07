use crate::utils::config::MINING_REWARD;
use crate::wallet::transaction::Transaction;
use crate::wallet::wallet::Wallet;

#[test]
fn test_outputs_the_amount_subtracted_from_the_sender_balance() {
    let sender_wallet = Wallet::new();
    let recipient = "recipient".to_string();
    let amount = 50;
    let transaction = Transaction::new(&sender_wallet, recipient.clone(), amount);
    assert_eq!(transaction.outputs[0].amount, sender_wallet.balance - amount);
}

#[test]
fn test_outputs_the_amount_added_to_the_recipient_balance() {
    let sender_wallet = Wallet::new();
    let recipient = "recipient".to_string();
    let amount = 50;
    let transaction = Transaction::new(&sender_wallet, recipient.clone(), amount);
    assert_eq!(transaction.outputs[1].amount, amount);
}

#[test]
fn test_transacting_with_amount_exceeding_balance() {
    let sender_wallet = Wallet::new();
    let recipient = "recipient".to_string();
    let amount = 100000;
    assert!(Transaction::new(&sender_wallet, recipient.clone(), amount).outputs.is_empty());
}

#[test]
fn test_inputs_the_balance_of_the_wallet() {
    let sender_wallet = Wallet::new();
    let recipient = "recipient".to_string();
    let amount = 50;
    let transaction = Transaction::new(&sender_wallet, recipient.clone(), amount);
    assert_eq!(transaction.input.as_ref().unwrap().amount, sender_wallet.balance);
}


#[test]
fn test_validates_a_valid_transaction() {
    let sender_wallet = Wallet::new();
    let recipient = "recipient".to_string();
    let amount = 50;
    let transaction = Transaction::new(&sender_wallet, recipient.clone(), amount);

    assert!(transaction.verify());
}


#[test]
fn test_invalidates_a_corrupt_transaction() {
    let sender_wallet = Wallet::new();
    let recipient = "recipient".to_string();
    let amount = 50;
    let mut transaction = Transaction::new(&sender_wallet, recipient.clone(), amount);
    transaction.outputs[0].amount = 100000;
    assert!(!transaction.verify());
}

// test update transaction
#[test]
fn test_updates_the_transaction() {
    let sender_wallet = Wallet::new();
    let recipient = "recipient".to_string();
    let amount = 50;
    let mut transaction = Transaction::new(&sender_wallet, recipient.clone(), amount);
    let new_amount = 25;
    let new_recipient = "new_recipient".to_string();
    transaction.update(&sender_wallet, new_recipient.clone(), new_amount).expect("Could not update transaction");
    assert_eq!(transaction.outputs[0].amount, sender_wallet.balance - amount - new_amount);
    assert_eq!(transaction.outputs[1].amount, amount);
    assert_eq!(transaction.outputs[1].address, recipient);
    assert_eq!(transaction.outputs[2].amount, new_amount);
    assert_eq!(transaction.outputs[2].address, new_recipient);
}

#[test]
fn test_reward_transaction() {
    let miner_wallet = Wallet::blockchain_wallet();
    let transaction = Transaction::reward_transaction(&miner_wallet, &Wallet::blockchain_wallet());
    assert_eq!(transaction.outputs.len(), 1);
    assert_eq!(transaction.outputs[0].amount, MINING_REWARD);
    assert_eq!(transaction.outputs[0].address, miner_wallet.public_key);
}