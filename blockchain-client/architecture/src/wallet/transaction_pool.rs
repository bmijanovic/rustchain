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

    pub fn valid_transactions(&self) -> Vec<Transaction> {
        self.transactions.iter().filter(|t| {
            // reduce output amount to total amount
            let total_output_amount: u64 = t.outputs.iter().map(|o| o.amount).sum();
            if total_output_amount != t.input.as_ref().unwrap().amount {
                return false;
            }
            // verify signature
            if !t.verify() {
                return false;
            }
            true
        }).cloned().collect()
    }
}