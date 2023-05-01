use methods::{MULTIPLY_ID};
use risc0_zkvm::{Receipt};

fn main() {
    let rec_str = "./rec.bin".to_string();

    let receipt_file = std::fs::read(&rec_str).unwrap();
    let receipt_two: Receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();
    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt_two.verify(&MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
