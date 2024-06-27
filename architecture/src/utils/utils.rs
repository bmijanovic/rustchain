use sha2::{Sha256, Digest};
use hex;
use serde_json::{json, Value};

pub(crate) fn crypto_hash(inputs: &[Value]) -> String {
    let mut hasher = Sha256::new();
    let mut serialized_inputs: Vec<String> = inputs.iter()
        .map(|input| serde_json::to_string(input).unwrap())
        .collect();
    serialized_inputs.sort();
    let concatenated = serialized_inputs.join(" ");
    hasher.update(concatenated);
    let hash = hasher.finalize();
    hex::encode(hash)
}