use std::fmt;
use crate::utils::config::INITIAL_BALANCE;
use ecdsa::{SigningKey, VerifyingKey};
use ecdsa::signature::{Signer, Verifier};
use k256::{Secp256k1};
use k256::{ecdsa::Signature as K256Signature};
use crate::blockchain::blockchain::Blockchain;
use crate::wallet::transaction::Transaction;
use crate::wallet::transaction_pool::TransactionPool;

#[derive(Clone)]
pub struct Wallet {
    pub balance: u64,
    pub signing_key: SigningKey<Secp256k1>,
    pub verifying_key: VerifyingKey<Secp256k1>,
    pub public_key: String,
}


// to string
impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wallet -\n  balance: {}\n  public_key: {}\n", self.balance, self.public_key)
    }
}


impl Wallet {
    pub fn new() -> Wallet {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let binding = signing_key.clone();
        let verifying_key = binding.verifying_key();
        let public_key = hex::encode(&verifying_key.to_sec1_bytes());
        Wallet {
            balance: INITIAL_BALANCE,
            public_key,
            signing_key,
            verifying_key: *verifying_key,
        }
    }

    pub fn sign(&self, data: &str) -> String {
        let signature: K256Signature = self.signing_key.sign(data.as_bytes());
        hex::encode(signature.to_der().as_bytes())
    }

    pub fn verify(address: VerifyingKey<Secp256k1>, data: &str, signature: &str) -> bool {
        let signature_bytes = hex::decode(signature).unwrap();
        let signature = K256Signature::from_der(&signature_bytes).unwrap();
        address.verify(data.as_bytes(), &signature).is_ok()
    }

    pub fn create_transaction(&mut self, recipient: String, amount: u64, transaction_pool: &mut TransactionPool, blockchain: &Blockchain) -> Result<Transaction, &'static str> {
        self.balance = self.calculate_balance(blockchain);
        if amount > self.balance {
            return Err("Amount exceeds balance");
        }
        let transaction = transaction_pool.existing_transaction(&self.public_key);

        let updated_transaction : Transaction;
        if let Some(mut existing_transaction) = transaction {
            updated_transaction = existing_transaction.update(self, recipient, amount).unwrap();
            transaction_pool.update_or_add_transaction(updated_transaction.clone());
        } else {
            updated_transaction = Transaction::new(self, recipient, amount);
            transaction_pool.update_or_add_transaction(updated_transaction.clone());
        }

        Ok(updated_transaction)
    }

    pub fn blockchain_wallet() -> Wallet {
        let mut blockchain_wallet = Wallet::new();
        blockchain_wallet.public_key = "blockchain_wallet".to_string();
        blockchain_wallet
    }

    pub fn calculate_balance(&self, blockchain: &Blockchain) -> u64 {
        let mut balance = self.balance;
        let mut transactions: Vec<Transaction> = Vec::new();

        blockchain.chain.iter().for_each(|block| {
            block.data.iter().for_each(|transaction| {
                transactions.push(transaction.clone());
            });
        });

        let wallet_input_transactions = transactions.iter().filter(|transaction|
            transaction.input.is_some() && transaction.input.as_ref().unwrap().address == self.public_key
        ).collect::<Vec<&Transaction>>();

        println!("transactions: {:?}", transactions);

        let mut start_time = 0;
        if wallet_input_transactions.len() > 0 {
            let recent_input_transaction = wallet_input_transactions.iter()
                .max_by_key(|transaction|
                    transaction.input.as_ref().unwrap().timestamp.timestamp()
                );
            start_time = recent_input_transaction.unwrap().input.as_ref().unwrap().timestamp.timestamp();
            balance = recent_input_transaction.unwrap().outputs.iter().find(|output|
                output.address == self.public_key
            ).unwrap().amount;
        }

        transactions.iter().for_each(|transaction| {
            if transaction.input.as_ref().unwrap().timestamp.timestamp() > start_time {
                transaction.outputs.iter().for_each(|output| {
                    if output.address == self.public_key {
                        balance += output.amount;
                    }
                });
            }
        });

        balance


    }

}

