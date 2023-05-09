#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use signature_core::DateAndHash;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let data: String = env::read();
    let now: u64 = env::read();
    let sha = Impl::hash_bytes(&data.as_bytes());
    let sha_receipt = DateAndHash {
        hash: *sha,
        date: now
    };
    env::commit(&sha_receipt);
}
