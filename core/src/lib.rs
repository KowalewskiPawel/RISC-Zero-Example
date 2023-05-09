
use core::fmt::Debug;

use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DateAndHash {
    pub hash: Digest,
    pub date: u64
}