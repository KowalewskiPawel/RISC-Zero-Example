use risc0_zkvm::Receipt;

fn main() {
    let rec_str = "./receipt.bin".to_string();
    let id_str = "./id.bin".to_string();

    let receipt_file = std::fs::read(&rec_str).unwrap();
    let receipt: Receipt = bincode::deserialize::<Receipt>(&receipt_file).unwrap();
    let id_file = std::fs::read(&id_str).unwrap();
    let id: [u32; 8] = bincode::deserialize::<[u32; 8]>(&id_file).unwrap();

    let _verification = match receipt.verify(&id) {
        Ok(()) => println!("Proof is valid"),
        Err(_) => println!("Something went wrong"),
    };
}
