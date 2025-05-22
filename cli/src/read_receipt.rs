use anyhow::Result;
use risc0_zkvm::Receipt; 
use std::{fs::read, path::Path};

pub fn read_receipt_file(receipt_path: &Path) -> Result<()> {
    println!("Attempting to read receipt from: {}", receipt_path.to_string_lossy());
    let receipt_bytes = read(receipt_path)?;
    let receipt: Receipt = bincode::deserialize(&receipt_bytes)?;

    println!("Successfully deserialized receipt.");

    // Print the journal
    if receipt.journal.bytes.is_empty() {
        println!("Committed output (journal) is empty.");
    } else {
        match std::str::from_utf8(&receipt.journal.bytes) {
            Ok(s) => println!("Committed output (journal) as string: \"{}\"", s),
            Err(_) => println!("Committed output (journal) as bytes: {:?}", receipt.journal.bytes),
        }
    }

    // We can also print other information from the receipt if needed,
    // for example, the Image ID (hex-encoded):
    // if let Ok(image_id) = receipt.get_image_id() { // Assuming get_image_id() returns a Result<[u32; 8]>
    //     let image_id_hex = image_id.iter().map(|word| format!("{:08x}", word)).collect::<String>();
    //     println!("Image ID: 0x{}", image_id_hex);
    // }

    Ok(())
} 
