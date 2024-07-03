use std::fmt;
use crate::utils::config::INITIAL_BALANCE;
use ecdsa::{SigningKey, VerifyingKey};
use ecdsa::signature::{Signer, Verifier};
use k256::{Secp256k1};
use k256::{ecdsa::Signature as K256Signature};
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

    pub fn create_transaction(&self, recipient: String, amount: u64, mut transaction_pool: &mut TransactionPool) -> Result<Transaction, &'static str> {
        if amount > self.balance {
            return Err("Amount exceeds balance");
        }
        let mut transaction = transaction_pool.existing_transaction(&self.public_key);

        let mut updated_transaction : Transaction;
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

}

