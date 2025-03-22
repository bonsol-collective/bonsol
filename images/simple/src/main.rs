use std::io::Read;

use gjson::Kind;
use risc0_zkvm::{
    guest::{env, sha::Impl},
    sha::Sha256,
};

fn main() {
    // Read all the input data at once
    // Inputs are sent as frames: 4-byte length followed by the data
    let mut buf: Vec<u8> = Vec::new();
    env::stdin().read_to_end(&mut buf).unwrap();

    let mut inputs = Vec::new();
    let mut offset = 0;
    while offset + 4 <= buf.len() {
        let length = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4; // Move past the length field

        // Ensure we have enough data for the frame
        if offset + length > buf.len() {
            // Stop if the frame is incomplete
            break;
        }

        // Extract the frame data
        inputs.push(&buf[offset..offset + length]);
        // Move to the next frame
        offset += length;
    }

    // Using from_utf8_lossy to avoid allocating new strings
    let publici1 = String::from_utf8_lossy(&inputs[0]);
    let privatei2 = String::from_utf8_lossy(&inputs[1]);

    println!("publici1: {}", publici1);
    println!("privatei2: {}", privatei2);

    let valid = gjson::valid(&publici1);
    let mut res: u8 = 0;
    if valid {
        let val = gjson::get(&publici1, "attestation");
        if val.kind() == Kind::String && val.str() == privatei2 {
            res = 1;
        }
    }

    let digest = Impl::hash_bytes(&[publici1.as_bytes(), privatei2.as_bytes()].concat());
    env::commit_slice(digest.as_bytes());
    env::commit_slice(&[res]);
}
