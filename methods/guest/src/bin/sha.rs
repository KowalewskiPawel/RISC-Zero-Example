#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256, Digest};
use std::time::SystemTime;
use signature_core::DateAndHash;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let data: String = env::read();
    let now: SystemTime = env::read();
    let sha = Impl::hash_bytes(&data.as_bytes());
    let new_receipt = DateAndHash {
        sha_info: *sha,
        date: now
    };
    env::commit(&new_receipt);
}
