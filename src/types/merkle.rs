use super::hash::{Hashable, H256};

use ring::digest::{self, Context};
use std::cell::RefCell;
use std::collections::btree_set::Difference;
use std::collections::{BTreeMap, VecDeque};
use std::convert::TryFrom;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct MerkleNode {
    left: Option<Rc<RefCell<MerkleNode>>>,
    right: Option<Rc<RefCell<MerkleNode>>>,
    value: H256,
}

impl MerkleNode {
    pub fn from_hash(value: H256) -> MerkleNode {
        MerkleNode { 
            left: None,
            right: None,
            value,
        }
    }

    pub fn from_nodes(l: &MerkleNode, r: &MerkleNode) -> MerkleNode {
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(l.value.as_ref());
        ctx.update(r.value.as_ref());
        let v = ctx.finish();
        MerkleNode { 
            left: Option::from(Rc::new(RefCell::new(l.clone()))),
            right: Option::from(Rc::new(RefCell::new(r.clone()))),
            value: H256::from(<[u8; 32]>::try_from(v.as_ref()).unwrap()),
        }
    }
}

/// A Merkle tree.
#[derive(Debug, Default)]
pub struct MerkleTree {
    root: MerkleNode,
    nodes: BTreeMap<usize, VecDeque<MerkleNode>>,
    height: usize, 
    leaf_size: usize,
}

impl MerkleTree {
    pub fn new<T>(data: &[T]) -> Self
    where
        T: Hashable,
    {
        unimplemented!()
    }

    pub fn root(&self) -> H256 {
        unimplemented!()
    }

    /// Returns the Merkle Proof of data at index i
    pub fn proof(&self, index: usize) -> Vec<H256> {
        unimplemented!()
    }
}

/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn verify(root: &H256, datum: &H256, proof: &[H256], index: usize, leaf_size: usize) -> bool {
    unimplemented!()
}
// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. BEFORE TEST

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::hash::H256;

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    #[test]
    fn merkle_root() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }

    #[test]
    fn merkle_proof() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert_eq!(
            proof,
            vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }

    #[test]
    fn merkle_verifying() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(
            &merkle_tree.root(),
            &input_data[0].hash(),
            &proof,
            0,
            input_data.len()
        ));
    }
}

// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. AFTER TEST
