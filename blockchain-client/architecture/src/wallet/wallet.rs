use std::fmt;
use crate::utils::config::INITIAL_BALANCE;
use ecdsa::{SignatureBytes, SigningKey, VerifyingKey};
use ecdsa::signature::{Signer, Verifier};
use k256::{Secp256k1};
use k256::{ecdsa::Signature as K256Signature, elliptic_curve::sec1::ToEncodedPoint};
use k256::elliptic_curve::weierstrass::add;

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
}

