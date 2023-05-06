use risc0_zkvm::serde::from_slice;
use risc0_zkvm::Receipt;

fn main() {
    let rec_str = "./receipt.bin".to_string();
    let id_str = "./image_id.bin".to_string();

    let receipt_file = std::fs::read(&rec_str).unwrap();
    let receipt: Receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();
    let id_file = std::fs::read(&id_str).unwrap();
    let id: [u32; 8] = bincode::deserialize::<[u32; 8]>(&id_file).unwrap();

    let result: u64 = from_slice(&receipt.journal).unwrap();

    let _verification = match receipt.verify(&id) {
        Ok(()) => println!("Proof for the result {} is valid", result),
        Err(_) => println!("Something went wrong"),
    };
}
