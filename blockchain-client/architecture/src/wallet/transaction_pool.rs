use crate::wallet::transaction::Transaction;

pub struct TransactionPool {
    pub transactions: Vec<Transaction>,
}


impl TransactionPool {
    pub fn new() -> TransactionPool {
        TransactionPool {
            transactions: vec![],
        }
    }

    pub fn update_or_add_transaction(&mut self, transaction: Transaction) {
        let transaction_id = transaction.id;
        let existing_transaction = self.transactions.iter_mut().find(|t| t.id == transaction_id);
        match existing_transaction {
            Some(existing_transaction) => *existing_transaction = transaction,
            None => self.transactions.push(transaction),
        }
    }

    pub fn existing_transaction(&self, address: &str) -> Option<Transaction> {
        self.transactions.iter().find(|t| t.input.as_ref().unwrap().address == address).cloned()
    }
}