use methods::{MULTIPLY_ID};
use risc0_zkvm::{Receipt};

fn main() {
    let rec_str = "./rec.bin".to_string();

    let receipt_file = std::fs::read(&rec_str).unwrap();
    let receipt_two: Receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();
    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    let _verification = match receipt_two.verify(&MULTIPLY_ID) {
        Ok(()) => println!("Proof is valid"),
        Err(_) => println!("Something went wrong"),
    };
}
