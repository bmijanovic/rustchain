use crate::wallet::transaction::Transaction;
use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::wallet::Wallet;

#[test]
fn test_add_transaction_to_the_pool() {
    let mut transaction_pool = TransactionPool::new();
    let transaction = Transaction::new(&Wallet::new(), "recipient".to_string(), 50);
    transaction_pool.update_or_add_transaction(transaction.clone());
    assert_eq!(transaction_pool.transactions.len(), 1);
    assert_eq!(transaction_pool.transactions[0], transaction);
}


#[test]
fn test_update_transaction_in_the_pool() {
    let mut transaction_pool = TransactionPool::new();
    let wallet = Wallet::new();
    let mut transaction = Transaction::new(&wallet, "recipient".to_string(), 50);
    transaction_pool.update_or_add_transaction(transaction.clone());
    transaction.update(&wallet, "new_recipient".to_string(), 25).expect("Failed to update transaction");
    transaction_pool.update_or_add_transaction(transaction.clone());
    assert_eq!(transaction_pool.transactions.len(), 1);
    assert_eq!(transaction_pool.transactions[0], transaction);
}

