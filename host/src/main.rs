use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::Prover;

fn main() {
    let a: u64 = 17;
    let b: u64 = 23;
    // Make the prover.
    let mut prover =
        Prover::new(MULTIPLY_ELF).expect("Prover should be constructed from valid ELF binary");

    prover.add_input_u32_slice(&to_vec(&a).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&b).expect("should be serializable"));

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    // Code for transmitting or serializing the receipt for
    // other parties to verify here

    let c: u64 = from_slice(&receipt.journal).unwrap();

    // Print an assertion
    println!(
        "Hello, world! I know the factors of {}, and I can prove it!",
        c
    );

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(&MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
