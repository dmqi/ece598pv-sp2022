use rand::Rng;
use ring::{
    digest,
    signature::{self, Ed25519KeyPair, Signature},
};
use serde::{Deserialize, Serialize};

use crate::types::address::{generate_random_address, Address};
use crate::types::hash::{Hashable, H256};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transaction {
    sender: Address,
    receiver: Address,
    value: u32,
}

impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        let s = bincode::serialize(&self).unwrap();
        digest::digest(&digest::SHA256, s.as_ref()).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> H256 {
        let s = bincode::serialize(&self).unwrap();
        digest::digest(&digest::SHA256, s.as_ref()).into()
    }
}

/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
    let t = bincode::serialize(&t).unwrap();
    let sig = key.sign(&t);
    sig
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &[u8], signature: &[u8]) -> bool {
    // transaction + secret_key => signature
    let msg = bincode::serialize(&t).unwrap();
    let hash = digest::digest(&digest::SHA256, &msg).as_ref().to_vec();
    let pub_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);
    pub_key.verify(&hash, signature).is_ok()
}

#[cfg(any(test, test_utilities))]
pub fn generate_random_transaction() -> Transaction {
    Transaction { 
        sender: generate_random_address(), 
        receiver: generate_random_address(), 
        value: 0,
    }
}

// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. BEFORE TEST

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::key_pair;
    use ring::signature::KeyPair;

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, key.public_key().as_ref(), signature.as_ref()));
    }
    #[test]
    fn sign_verify_two() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        let key_2 = key_pair::random();
        let t_2 = generate_random_transaction();
        assert!(!verify(&t_2, key.public_key().as_ref(), signature.as_ref()));
        assert!(!verify(&t, key_2.public_key().as_ref(), signature.as_ref()));
    }
}

// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. AFTER TEST
