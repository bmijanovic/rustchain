use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::wallet::Wallet;

#[test]
fn test_print_wallet() {
    let wallet = Wallet::new();
    println!("{}", wallet);
}

#[test]
fn test_doubles_the_send_amount_subtracted_from_the_wallet_balance() {
    let sender_wallet = Wallet::new();
    let mut tp = TransactionPool::new();
    let sender_amount = 50;
    let recipient = "recipient".to_string();
    let transaction = sender_wallet.create_transaction(recipient.clone(), sender_amount, &mut tp).unwrap();
    let transaction2 = sender_wallet.create_transaction(recipient.clone(), sender_amount, &mut tp).unwrap();
    assert_eq!(transaction2.outputs[0].amount, sender_wallet.balance - sender_amount * 2);
}


#[test]
fn clones_the_send_amount_output_for_the_recipient() {
    let sender_wallet = Wallet::new();
    let mut tp = TransactionPool::new();
    let sender_amount = 50;
    let recipient = "recipient".to_string();
    sender_wallet.create_transaction(recipient.clone(), sender_amount, &mut tp).unwrap();
    let transaction2 = sender_wallet.create_transaction(recipient.clone(), sender_amount, &mut tp).unwrap();
    assert_eq!(transaction2.outputs.iter().filter(|output| output.address == recipient).map(|output| output.amount).collect::<Vec<u64>>(), vec![sender_amount, sender_amount]);
}