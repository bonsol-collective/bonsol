use gjson::Kind;
use risc0_zkvm::{guest::{env, sha::Impl}, sha::{Digest, Sha256}};

fn main() {
    // 1. Read public input (JSON)
    let mut public1 = Vec::new();
    env::read_slice(&mut public1);
    let publici1 = String::from_utf8(public1).unwrap();

    // 2. Read private input
    let mut private2 = Vec::new();
    env::read_slice(&mut private2);
    let privatei2 = String::from_utf8(private2).unwrap();

    // 3. Validate and process inputs
    let valid = gjson::valid(&publici1);
    let mut res = 0;
    if valid {
        let val = gjson::get(&publici1, "attestation");
        if val.kind() == Kind::String && val.str() == privatei2 {
            res = 1;
        }
    }

    // 4. Create and commit hash of all inputs
    let digest = Impl::hash_bytes(
        &[
            publici1.as_bytes(),
            privatei2.as_bytes(),
        ].concat(),
    );
    env::commit_slice(digest.as_bytes());

    // 5. Commit the result
    env::commit_slice(&[res]);
}
