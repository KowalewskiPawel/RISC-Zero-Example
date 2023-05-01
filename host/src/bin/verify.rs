use methods::MULTIPLY_ID;
use risc0_zkvm::Receipt;

fn main() {
    let rec_str = "./receipt.bin".to_string();

    let receipt_file = std::fs::read(&rec_str).unwrap();
    let receipt: Receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();

    let _verification = match receipt.verify(&MULTIPLY_ID) {
        Ok(()) => println!("Proof is valid"),
        Err(_) => println!("Something went wrong"),
    };
}
