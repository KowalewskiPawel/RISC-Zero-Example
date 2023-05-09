
use core::fmt::Debug;
use std::time::SystemTime;

use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DateAndHash {
    pub sha_info: Digest,
    pub date: SystemTime
}