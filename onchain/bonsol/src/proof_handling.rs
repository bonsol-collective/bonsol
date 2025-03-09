use std::ops::Neg;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use groth16_solana::groth16::{Groth16Verifier, Groth16Verifyingkey};
use solana_program::{hash::hashv, msg};

use crate::{
    error::ChannelError,
    prover::{PROVER_CONSTANTS_V1_0_1, PROVER_CONSTANTS_V1_2_1},
    verifying_key::VERIFYINGKEY,
};

type G1 = ark_bn254::g1::G1Affine;

pub fn verify_risc0_v1_0_1(proof: &[u8], inputs: &[u8]) -> Result<bool, ChannelError> {
    let ins: [[u8; 32]; 5] = [
        sized_range::<32>(&inputs[0..32])?,
        sized_range::<32>(&inputs[32..64])?,
        sized_range::<32>(&inputs[64..96])?,
        sized_range::<32>(&inputs[96..128])?,
        sized_range::<32>(&inputs[128..160])?,
    ];
    verify_proof::<5>(proof, ins, &VERIFYINGKEY)
}

pub fn verify_risc0_v1_2_1(proof: &[u8], inputs: &[u8]) -> Result<bool, ChannelError> {
    msg!(
        "Starting RISC0 v1.2.1 verification:\n\
         - Proof size: {} bytes\n\
         - Input size: {} bytes\n\
         - Proof prefix: {}\n\
         - Input prefix: {}",
        proof.len(),
        inputs.len(),
        hex::encode(&proof[..32.min(proof.len())]),
        hex::encode(&inputs[..32.min(inputs.len())])
    );

    let ins: [[u8; 32]; 5] = [
        sized_range::<32>(&inputs[0..32])?,
        sized_range::<32>(&inputs[32..64])?,
        sized_range::<32>(&inputs[64..96])?,
        sized_range::<32>(&inputs[96..128])?,
        sized_range::<32>(&inputs[128..160])?,
    ];

    msg!(
        "Input arrays prepared:\n\
         - Array 0 (control root 0): {}\n\
         - Array 1 (control root 1): {}\n\
         - Array 2 (digest half 1): {}\n\
         - Array 3 (digest half 2): {}\n\
         - Array 4 (control ID): {}",
        hex::encode(&ins[0]),
        hex::encode(&ins[1]),
        hex::encode(&ins[2]),
        hex::encode(&ins[3]),
        hex::encode(&ins[4])
    );
    
    msg!("Starting proof verification with verifying key");
    let result = verify_proof::<5>(proof, ins, &VERIFYINGKEY);
    
    match &result {
        Ok(true) => msg!(
            "Verification succeeded:\n\
             - Total inputs processed: 5"
        ),
        Ok(false) => msg!(
            "Verification failed (returned false):\n\
             - Total inputs processed: 5"
        ),
        Err(e) => msg!(
            "Verification error:\n\
             - Error: {:?}\n\
             - Total inputs processed: 5",
            e
        ),
    }
    
    result
}

fn verify_proof<const NI: usize>(
    proof: &[u8],
    inputs: [[u8; 32]; NI],
    vkey: &Groth16Verifyingkey,
) -> Result<bool, ChannelError> {
    msg!("Step 1: Starting proof verification process");
    
    // Log input details
    let input_concat = inputs.concat();
    let input_slice = &[&input_concat[..]];
    msg!(
        "Step 1a - Input validation:\n\
         - Number of inputs: {}\n\
         - Total input bytes: {}\n\
         - Input hash: {}\n\
         - First input preview: {}",
        NI,
        inputs.len() * 32,
        hex::encode(hashv(input_slice).to_bytes()),
        hex::encode(&inputs[0])
    );
    
    msg!("Step 2: Deserializing proof_a");
    let ace: Vec<u8> = toggle_endianness_256(&[&proof[0..64], &[0u8][..]].concat());
    let proof_a: G1 = match G1::deserialize_with_mode(&*ace, Compress::No, Validate::No) {
        Ok(pa) => {
            msg!("Step 2a: Successfully deserialized proof_a");
            pa
        }
        Err(e) => {
            msg!(
                "Step 2 FAILED - proof_a deserialization error:\n\
                 - Error: {:?}\n\
                 - Raw bytes (first 64): {}\n\
                 - Endian-toggled (first 64): {}",
                e,
                hex::encode(&proof[0..64]),
                hex::encode(&ace[0..64])
            );
            return Err(ChannelError::InvalidInstruction);
        }
    };

    msg!("Step 3: Negating and reserializing proof_a");
    let mut proof_a_neg = [0u8; 65];
    if let Err(e) = G1::serialize_with_mode(&proof_a.neg(), &mut proof_a_neg[..], Compress::No) {
        msg!(
            "Step 3 FAILED - proof_a negation error:\n\
             - Error: {:?}\n\
             - Original proof_a: {:?}",
            e,
            proof_a
        );
        return Err(ChannelError::InvalidInstruction);
    }
    msg!("Step 3a: Successfully negated proof_a");

    msg!("Step 4: Converting proof_a endianness");
    let proof_a: [u8; 64] = match toggle_endianness_256(&proof_a_neg[..64]).try_into() {
        Ok(pa) => {
            msg!("Step 4a: Successfully converted proof_a endianness");
            pa
        }
        Err(e) => {
            msg!(
                "Step 4 FAILED - proof_a endianness conversion error:\n\
                 - Error: {:?}\n\
                 - Negated bytes: {}",
                e,
                hex::encode(&proof_a_neg[..64])
            );
            return Err(ChannelError::InvalidInstruction);
        }
    };

    msg!("Step 5: Extracting proof_b");
    let proof_b: [u8; 128] = match proof[64..192].try_into() {
        Ok(pb) => {
            msg!("Step 5a: Successfully extracted proof_b");
            pb
        }
        Err(e) => {
            msg!(
                "Step 5 FAILED - proof_b extraction error:\n\
                 - Error: {:?}\n\
                 - Raw bytes: {}",
                e,
                hex::encode(&proof[64..192])
            );
            return Err(ChannelError::InvalidInstruction);
        }
    };

    msg!("Step 6: Extracting proof_c");
    let proof_c: [u8; 64] = match proof[192..256].try_into() {
        Ok(pc) => {
            msg!("Step 6a: Successfully extracted proof_c");
            pc
        }
        Err(e) => {
            msg!(
                "Step 6 FAILED - proof_c extraction error:\n\
                 - Error: {:?}\n\
                 - Raw bytes: {}",
                e,
                hex::encode(&proof[192..256])
            );
            return Err(ChannelError::InvalidInstruction);
        }
    };

    msg!(
        "Step 7: All proof components prepared:\n\
         - proof_a hash: {}\n\
         - proof_b hash: {}\n\
         - proof_c hash: {}\n\
         - Verifying key info: prepared",
        hex::encode(hashv(&[&proof_a.to_vec()[..]]).to_bytes()),
        hex::encode(hashv(&[&proof_b.to_vec()[..]]).to_bytes()),
        hex::encode(hashv(&[&proof_c.to_vec()[..]]).to_bytes())
    );

    msg!("Step 8: Creating Groth16 verifier");
    let mut verifier: Groth16Verifier<NI> = match Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &inputs, vkey) {
        Ok(v) => {
            msg!("Step 8a: Successfully created Groth16 verifier");
            v
        }
        Err(e) => {
            msg!(
                "Step 8 FAILED - Groth16 verifier creation error:\n\
                 - Error: {:?}\n\
                 - Proof components valid: {}\n\
                 - Input size matches: {}\n\
                 - Component sizes:\n\
                   * proof_a: {} bytes\n\
                   * proof_b: {} bytes\n\
                   * proof_c: {} bytes",
                e,
                proof_a.len() == 64 && proof_b.len() == 128 && proof_c.len() == 64,
                inputs.len() == NI,
                proof_a.len(),
                proof_b.len(),
                proof_c.len()
            );
            return Err(ChannelError::InvalidProof);
        }
    };

    msg!("Step 9: Starting final Groth16 verification");
    verifier
        .verify()
        .map_err(|e| {
            msg!(
                "Step 9 FAILED - Groth16 verification error:\n\
                 - Error: {:?}\n\
                 - Input count: {}\n\
                 - Total proof size: {} bytes\n\
                 - Individual hashes:\n\
                   * proof_a: {}\n\
                   * proof_b: {}\n\
                   * proof_c: {}\n\
                 - First input preview: {}",
                e,
                NI,
                proof.len(),
                hex::encode(hashv(&[&proof_a.to_vec()[..]]).to_bytes()),
                hex::encode(hashv(&[&proof_b.to_vec()[..]]).to_bytes()),
                hex::encode(hashv(&[&proof_c.to_vec()[..]]).to_bytes()),
                hex::encode(&inputs[0])
            );
            ChannelError::ProofVerificationFailed
        })
}

pub fn output_digest_v1_0_1(
    input_digest: &[u8],
    committed_outputs: &[u8],
    assumption_digest: &[u8],
) -> [u8; 32] {
    let jbytes = [input_digest, committed_outputs].concat(); // bad copy here
    let journal = hashv(&[jbytes.as_slice()]);
    hashv(&[
        PROVER_CONSTANTS_V1_0_1.output_hash.as_ref(),
        journal.as_ref(),
        assumption_digest,
        &2u16.to_le_bytes(),
    ])
    .to_bytes()
}

pub fn prepare_inputs_v1_0_1(
    image_id: &str,
    execution_digest: &[u8],
    output_digest: &[u8],
    system_exit_code: u32,
    user_exit_code: u32,
) -> Result<Vec<u8>, ChannelError> {
    let imgbytes = hex::decode(image_id).map_err(|_| ChannelError::InvalidFieldElement)?;
    let mut digest = hashv(&[
        PROVER_CONSTANTS_V1_0_1.receipt_claim_hash.as_ref(),
        &[0u8; 32],
        &imgbytes,
        execution_digest,
        output_digest,
        &system_exit_code.to_le_bytes(),
        &user_exit_code.to_le_bytes(),
        &4u16.to_le_bytes(),
    ])
    .to_bytes();
    let (c0, c1) = split_digest_reversed(&mut PROVER_CONSTANTS_V1_0_1.control_root.clone())
        .map_err(|_| ChannelError::InvalidFieldElement)?;
    let (half1_bytes, half2_bytes) =
        split_digest_reversed(&mut digest).map_err(|_| ChannelError::InvalidFieldElement)?;
    let inputs = [
        c0,
        c1,
        half1_bytes.try_into().unwrap(),
        half2_bytes.try_into().unwrap(),
        PROVER_CONSTANTS_V1_0_1.bn254_control_id_bytes,
    ]
    .concat();
    Ok(inputs)
}

pub fn output_digest_v1_2_1(
    input_digest: &[u8],
    committed_outputs: &[u8],
    assumption_digest: &[u8],
) -> [u8; 32] {
    let jbytes = [input_digest, committed_outputs].concat(); // bad copy here
    let journal = hashv(&[jbytes.as_slice()]);
    hashv(&[
        PROVER_CONSTANTS_V1_2_1.output_hash.as_ref(),
        journal.as_ref(),
        assumption_digest,
        &2u16.to_le_bytes(),
    ])
    .to_bytes()
}

pub fn prepare_inputs_v1_2_1(
    image_id: &str,
    execution_digest: &[u8],
    output_digest: &[u8],
    system_exit_code: u32,
    user_exit_code: u32,
) -> Result<Vec<u8>, ChannelError> {
    let imgbytes = hex::decode(image_id).map_err(|_| ChannelError::InvalidFieldElement)?;
    let mut digest = hashv(&[
        PROVER_CONSTANTS_V1_2_1.receipt_claim_hash.as_ref(),
        &[0u8; 32],
        &imgbytes,
        execution_digest,
        output_digest,
        &system_exit_code.to_le_bytes(),
        &user_exit_code.to_le_bytes(),
        &4u16.to_le_bytes(),
    ])
    .to_bytes();
    let (c0, c1) = split_digest_reversed(&mut PROVER_CONSTANTS_V1_2_1.control_root.clone())
        .map_err(|_| ChannelError::InvalidFieldElement)?;
    let (half1_bytes, half2_bytes) =
        split_digest_reversed(&mut digest).map_err(|_| ChannelError::InvalidFieldElement)?;
    let inputs = [
        c0,
        c1,
        half1_bytes.try_into().unwrap(),
        half2_bytes.try_into().unwrap(),
        PROVER_CONSTANTS_V1_2_1.bn254_control_id_bytes,
    ]
    .concat();
    Ok(inputs)
}

/**
 * Reverse and split a digest into two halves
 * The first half is the left half of the digest
 * The second half is the right half of the digest
 *
 * @param d: The digest to split
 * @return A tuple containing the left and right halves of the digest
 */
pub fn split_digest_reversed_256(d: &mut [u8]) -> Result<([u8; 32], [u8; 32]), ChannelError> {
    split_digest_reversed::<32>(d)
}

fn split_digest_reversed<const N: usize>(d: &mut [u8]) -> Result<([u8; N], [u8; N]), ChannelError> {
    if d.len() != N {
        return Err(ChannelError::UnexpectedProofSystem);
    }
    d.reverse();
    let split_index = (N + 1) / 2;
    let (a, b) = d.split_at(split_index);
    let af = to_fixed_array(a);
    let bf = to_fixed_array(b);
    Ok((bf, af))
}

fn to_fixed_array<const N: usize>(input: &[u8]) -> [u8; N] {
    let mut fixed_array = [0u8; N];
    if input.len() >= N {
        // Copy the last N bytes of input into fixed_array
        fixed_array.copy_from_slice(&input[input.len() - N..]);
    } else {
        // Copy input into the end of fixed_array
        let start = N - input.len();
        fixed_array[start..].copy_from_slice(input);
    }
    fixed_array
}

fn sized_range<const N: usize>(slice: &[u8]) -> Result<[u8; N], ChannelError> {
    slice
        .try_into()
        .map_err(|_| ChannelError::InvalidInstruction)
}

// hello ethereum! Toggle endianness of a slice of bytes assuming 256 bit word size
fn toggle_endianness_256(bytes: &[u8]) -> Vec<u8> {
    toggle_endianness::<32>(bytes)
}

fn toggle_endianness<const N: usize>(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::with_capacity(bytes.len());
    let chunk_size = N;

    for chunk in bytes.chunks(chunk_size) {
        // Reverse the chunk and extend the vector
        vec.extend(chunk.iter().rev());
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_endianness() {
        let bytes = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let expected = [8u8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(toggle_endianness::<8>(&bytes), expected);
    }

    #[test]
    fn test_toggle_endianness_odd() {
        let bytes = [1u8, 2, 3, 4, 5, 6, 7];
        let expected = [7u8, 6, 5, 4, 3, 2, 1];
        assert_eq!(toggle_endianness::<7>(&bytes), expected);
    }

    #[test]
    fn test_toggle_endianness_quad_word() {
        let bytes = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let expected = [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(toggle_endianness_256(&bytes), expected);
    }

    #[test]
    fn test_split_digest() {
        let mut digest = [1u8; 32];
        digest[0] = 103;
        let (a, b) = split_digest_reversed(&mut digest).unwrap();
        let expect_digest_right = to_fixed_array::<32>(&[1u8; 16]);
        let mut expect_digest_left = expect_digest_right;
        expect_digest_left[31] = 103;
        assert_eq!(a, expect_digest_left);
        assert_eq!(b, expect_digest_right);
    }

    #[test]
    fn test_split_digest_odd() {
        let mut digest = [1u8; 31];
        digest[0] = 103;
        let (a, b) = split_digest_reversed(&mut digest).unwrap();
        let expect_digest_right = to_fixed_array::<31>(&[1u8; 16]);
        let mut expect_digest_left = to_fixed_array::<31>(&[1u8; 15]);
        expect_digest_left[30] = 103;
        assert_eq!(a, expect_digest_left);
        assert_eq!(b, expect_digest_right);
    }

    #[test]
    fn test_split_digest_16() {
        let digest = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let (a, b) = split_digest_reversed::<16>(&mut digest.to_vec()).unwrap();
        let expect_digest_left = to_fixed_array::<16>(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let expect_digest_right = to_fixed_array::<16>(&[15, 14, 13, 12, 11, 10, 9, 8]);
        assert_eq!(a, expect_digest_left);
        assert_eq!(b, expect_digest_right);
    }

    #[test]
    fn test_split_digest_8() {
        let digest = [0, 1, 2, 3, 4, 5, 6, 7];
        let (a, b) = split_digest_reversed::<8>(&mut digest.to_vec()).unwrap();
        let expect_digest_left = to_fixed_array::<8>(&[3, 2, 1, 0]);
        let expect_digest_right = to_fixed_array::<8>(&[7, 6, 5, 4]);
        assert_eq!(a, expect_digest_left);
        assert_eq!(b, expect_digest_right);
    }

    #[test]
    fn test_invalid_digest_wrong_size() {
        let mut d1 = [1u8; 31];
        assert!(split_digest_reversed_256(&mut d1).is_err());
        let mut d2 = [1u8; 33];
        assert!(split_digest_reversed_256(&mut d2).is_err());
    }

    #[test]
    fn test_sized_range() {
        let slice = [1u8; 32];
        let expected = [1u8; 32];
        assert_eq!(sized_range::<32>(&slice).unwrap(), expected);
    }
}
