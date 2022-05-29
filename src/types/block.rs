use crate::types::hash::{Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        unimplemented!()
    }
}

impl Block {
    pub fn get_parent(&self) -> H256 {
        unimplemented!()
    }

    pub fn get_difficulty(&self) -> H256 {
        unimplemented!()
    }
}

#[cfg(any(test, test_utilities))]
pub fn generate_random_block(parent: &H256) -> Block {
    unimplemented!()
}
