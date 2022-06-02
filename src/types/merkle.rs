use super::hash::{Hashable, H256};

use std::collections::VecDeque;

use ring::digest;

/// A Merkle tree.
#[derive(Debug, Default)]
pub struct MerkleTree {
    tree: Vec<H256>,
    leaf_size: usize,
}

impl MerkleTree {
    pub fn new<T>(data: &[T]) -> Self
    where
        T: Hashable,
    {
        let mut tree: Vec<H256> = Vec::new();
        let mut data_len = data.len();
        if data_len == 0 {
            let data_block: [u8; 32] = [0; 32];
            tree.push(data_block.into());
            return MerkleTree { tree, leaf_size: 0 };
        }
        let mut queue: VecDeque<H256> = VecDeque::new();
        for x in data.into_iter() {
            queue.push_back(x.hash());
        }
        if data_len % 2 != 0 {
            queue.push_back(data[data_len - 1].hash());
            data_len += 1;
        }
        let mut count = 0;
        let mut level_len = data_len;
        while !queue.is_empty() {
            let hash1 = queue.pop_front().unwrap();
            tree.push(hash1);
            count += 1;
            let temp = queue.pop_front();
            if temp.is_none() {
                break;
            }
            let hash2 = temp.unwrap();
            tree.push(hash2);
            count += 1;
            let mut ctx = digest::Context::new(&digest::SHA256);
            ctx.update(hash1.as_ref());
            ctx.update(hash2.as_ref());
            let hash: H256 = ctx.finish().into();
            queue.push_back(hash);
            if count == level_len && count != 2 {
                let i = level_len / 2;
                if i % 2 != 0 {
                    queue.push_back(hash);
                    level_len = i + 1;
                    count = 0;
                } else {
                    level_len = i;
                    count = 0;
                }
            }
        }

        MerkleTree {
            tree,
            leaf_size: data_len,
        }
    }

    pub fn root(&self) -> H256 {
        self.tree[self.tree.len() - 1]
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
