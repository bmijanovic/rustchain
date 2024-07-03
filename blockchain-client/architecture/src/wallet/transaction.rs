use std::fmt;
use chrono::{DateTime, Local, Utc};
use ecdsa::VerifyingKey;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::blockchain::block::Block;
use crate::wallet::wallet::Wallet;
use crate::utils::utils::crypto_hash;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: TransactionId,
    pub input: Option<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Copy, Hash, Default)]
pub struct TransactionId(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TransactionInput {
    pub timestamp: DateTime<Utc>,
    pub amount: u64,
    pub address: String,
    pub signature: String,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TransactionOutput {
    pub amount: u64,
    pub address: String,
}


impl TransactionOutput {
    pub fn new(amount: u64, address: String) -> TransactionOutput {
        TransactionOutput {
            amount,
            address,
        }
    }
}

impl Transaction {
    pub fn new(sender_wallet: &Wallet, recipient: String, amount: u64) -> Transaction {
        if amount > sender_wallet.balance {
            return Transaction {
                id: TransactionId(Uuid::new_v4()),
                input: None,
                outputs: vec![],
            };
        }
        let sender_transaction_output = TransactionOutput::new(sender_wallet.balance - amount, sender_wallet.public_key.clone());
        let reciever_transaction_output = TransactionOutput::new(amount, recipient);
        let mut transaction = Transaction {
            id: TransactionId(Uuid::new_v4()),
            input: None,
            outputs: vec![sender_transaction_output, reciever_transaction_output],
        };

        Transaction::sign(&mut transaction, sender_wallet);

        transaction


    }

    pub fn sign(&mut self, sender_wallet: &Wallet) {
        let input = TransactionInput {
            timestamp: Local::now().with_timezone(&Utc),
            amount: sender_wallet.balance,
            address: sender_wallet.public_key.clone(),
            signature: sender_wallet.sign(crypto_hash(&[json!(&self.outputs)]).as_str()),
        };
        self.input = Some(input);
    }

    pub fn verify(&self) -> bool {
        let input = self.input.as_ref().unwrap();
        let hash = crypto_hash(&[json!(&self.outputs)]);
        let verifying_key = VerifyingKey::from_sec1_bytes(&hex::decode(&input.address).unwrap()).unwrap();
        Wallet::verify(verifying_key, hash.as_str(), &input.signature)
    }

    pub fn update(&mut self, sender_wallet: &Wallet, recipient: String, amount: u64) -> Result<Transaction, &'static str> {
        let sender_output = self.outputs.iter_mut().find(|output| output.address == sender_wallet.public_key);
        if (amount > sender_wallet.balance) || sender_output.is_none() {
            return Err("Amount exceeds balance");
        }
        let sender_output = sender_output.unwrap();
        sender_output.amount = sender_output.amount - amount;
        self.outputs.push(TransactionOutput::new(amount, recipient));
        Transaction::sign(self, sender_wallet);
        Ok(self.to_owned())
    }

}


impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: \n\tid: {}\n\tinput: {:?}\n\toutputs: {:?}\n", self.id.0, self.input, self.outputs)
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TransactionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransactionInput: \n\ttimestamp: {}\n\tamount: {}\n\taddress: {}\n\tsignature: {}\n", self.timestamp, self.amount, self.address, self.signature)
    }
}

impl fmt::Display for TransactionOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransactionOutput: \n\tamount: {}\n\taddress: {}\n", self.amount, self.address)
    }
}



