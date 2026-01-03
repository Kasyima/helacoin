use crate::U256;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hash(U256);
impl Hash {
    // hash anything that can be serde Serialized via ciborium
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];
        // serialize the data into the CBOR binary format via the ciborium crate
        // the ciborium crate requires sth that can be written into.
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!("Failed to serialize data: {:?}. This should not happen", e);
        }
        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array = hash_bytes.as_slice();

        Hash(U256::from_little_endian(hash_array))
    }
    // check if a hash matches a target
    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }
    // zero hash
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
}

use std::fmt;

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
