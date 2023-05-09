use methods::{SHA_ELF, SHA_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    sha::Digest,
    Prover, Receipt,
};
fn main() {
    let a = "abc".to_string();
    // Make the prover.
    let mut prover =
        Prover::new(SHA_ELF).expect("Prover should be constructed from valid ELF binary");

    prover.add_input_u32_slice(&to_vec(&a).expect("should be serializable"));

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    let digest: Digest = from_slice(&receipt.journal).expect("Journal should contain SHA Digest");

    println!("I provably know data whose SHA-256 hash is {}", digest);

    // // Code for transmitting or serializing the receipt for
    // // other parties to verify here

    // let c: u64 = from_slice(&receipt.journal).unwrap();

    // // Print an assertion
    // println!(
    //     "Hello, world! I know the factors of {}, and I can prove it!",
    //     c
    // );

    // let serialized_receipt = bincode::serialize(&receipt).unwrap();
    // let serialized_image_id = bincode::serialize(&MULTIPLY_ID).unwrap();

    // let _saved_receipt_file = match std::fs::write("./receipt.bin", serialized_receipt) {
    //     Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
    //     Err(_) => println!("Something went wrong"),
    // };

    // let _saved_image_id_file = match std::fs::write("./image_id.bin", serialized_image_id) {
    //     Ok(()) => println!("ImageID saved and serialized as id.bin"),
    //     Err(_) => println!("Something went wrong"),
    // };
}

// #[cfg(test)]
// mod tests {

//     use methods::{MULTIPLY_ELF, MULTIPLY_ID};
//     use risc0_zkvm::{
//         serde::{from_slice, to_vec},
//         Prover, Receipt,
//     };

//     const TEST_FACTOR_ONE: u64 = 17;
//     const TEST_FACTOR_TWO: u64 = 23;

//     #[test]
//     fn run_factors() {
//         let mut prover = Prover::new(MULTIPLY_ELF).expect(
//             "Prover should be constructed from valid method source code and corresponding method ID",
//         );

//         prover.add_input_u32_slice(&to_vec(&TEST_FACTOR_ONE).expect("should be serializable"));
//         prover.add_input_u32_slice(&to_vec(&TEST_FACTOR_TWO).expect("should be serializable"));

//         let receipt = prover.run().expect("Should be able to prove valid code");
//         receipt
//             .verify(&MULTIPLY_ID)
//             .expect("Proven code should verify");

//         let result: u64 = from_slice(&receipt.journal).expect(
//             "Journal output should deserialize into the same types (& order) that it was written",
//         );
//         assert_eq!(
//             result,
//             TEST_FACTOR_ONE * TEST_FACTOR_TWO,
//             "We expect the zkVM output to be the product of the inputs"
//         )
//     }

//     #[test]
//     fn verify_receipt() {
//         let rec_str = "../receipt.bin".to_string();

//         let receipt_file = std::fs::read(&rec_str).unwrap();
//         let receipt: Receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();

//         let id_str = "../image_id.bin".to_string();

//         let id_file = std::fs::read(&id_str).unwrap();
//         let id: [u32; 8] = bincode::deserialize::<[u32; 8]>(&id_file).unwrap();
//         receipt.verify(&id).expect("Proven code should verify");

//         let result: u64 = from_slice(&receipt.journal).expect(
//             "Journal output should deserialize into the same types (& order) that it was written",
//         );
//         assert_eq!(
//             result,
//             TEST_FACTOR_ONE * TEST_FACTOR_TWO,
//             "We expect the zkVM output to be the product of the inputs"
//         )
//     }
// }
