use std::time::SystemTime;

use methods::{SHA_ELF, SHA_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};
use signature_core::DateAndHash;
fn main() {
    let string_to_hash = "abc".to_string();
    let time_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let unix_secs = time_now.as_secs();
    // Make the prover.
    let mut prover =
        Prover::new(SHA_ELF).expect("Prover should be constructed from valid ELF binary");

    prover.add_input_u32_slice(&to_vec(&string_to_hash).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&unix_secs).expect("should be serializable"));

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    let result: DateAndHash =
        from_slice(&receipt.journal).expect("Journal should contain SHA Digest");

    println!(
        "I provably know data whose SHA-256 hash is {}, time (UNIX secs): {} ",
        result.hash, result.date
    );

    let serialized_receipt = bincode::serialize(&receipt).unwrap();
    let serialized_image_id = bincode::serialize(&SHA_ID).unwrap();

    let _saved_receipt_file = match std::fs::write("./receipt.bin", serialized_receipt) {
        Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
        Err(_) => println!("Something went wrong"),
    };

    let _saved_image_id_file = match std::fs::write("./image_id.bin", serialized_image_id) {
        Ok(()) => println!("ImageID saved and serialized as id.bin"),
        Err(_) => println!("Something went wrong"),
    };
}

#[cfg(test)]
mod tests {

    use std::time::SystemTime;

    use methods::{SHA_ELF, SHA_ID};
    use risc0_zkvm::{
        serde::{from_slice, to_vec},
        sha::{Impl, Sha256},
        Prover
    };
    use signature_core::DateAndHash;


    const TEST_STR: &str = "fiz_buzz";

    #[test]
    fn prove_sha() {
        let string_to_hash = TEST_STR.to_string();
        let time_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let unix_secs = time_now.as_secs();
        let hashed_data = Impl::hash_bytes(&TEST_STR.to_string().as_bytes());
        let mut prover = Prover::new(SHA_ELF).expect(
            "Prover should be constructed from valid method source code and corresponding method ID",
        );

        prover.add_input_u32_slice(&to_vec(&string_to_hash).expect("should be serializable"));
        prover.add_input_u32_slice(&to_vec(&unix_secs).expect("should be serializable"));

        let receipt = prover.run().expect("Should be able to prove valid code");
        receipt.verify(&SHA_ID).expect("Proven code should verify");

        let result: DateAndHash = from_slice(&receipt.journal).expect(
            "Journal output should deserialize into the same types (& order) that it was written",
        );
        assert_eq!(
            result.hash, *hashed_data,
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}
