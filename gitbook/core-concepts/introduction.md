---
description: >-
  An introduction to Bonsol, its underlying technologies, and how it integrates
  with Solana.
icon: subtitles
---

# Introduction

## Understanding Bonsol

Bonsol acts as a bridge between Solana's on-chain capabilities and off-chain computational power. It allows developers to execute computationally intensive tasks off-chain and then verify the results on-chain, leveraging the power of verifiable computation. Using Bonsol, developers can:

* Lower their regulatory burden
* Build trust with their community
* Simplify protocol design

Bonsol is deeply integrated with Solana and can be used to build a variety of use cases. You can compose other programs on top of Bonsol to add verifiable computation to your protocol, or add a verifiable layer on top of existing primitives. Bonsol is built on top of the excellent RISC Zero zkVM, which allows developers to write arbitrary programs and generate verifiable proofs of their execution, in some cases those proofs can be zero-knowledge with regard to the inputs.

## How Bonsol Works

1. Developers create verifiable programs using RISC Zero Tooling
2. These verifiable programs are registered with Bonsol
3. Users can request execution of these verifiable programs through Bonsol
4. Provers run the verifiable programs and generate STARK proofs
5. Bonsol wraps the STARK proof into a SNARK (Succinct Non-interactive ARgument of Knowledge)
6. The SNARK proof is verified natively on Solana

### RISC0 STARK Proofs

RISC Zero generates STARK proofs, which have several important properties:

1. Scalability: STARK proofs can handle arbitrarily large computations, with proof size and verification time growing logarithmically with the computation size.
2. Transparency: STARKs don't require a trusted setup, enhancing their security and reducing reliance on external parties.
3. Variable Length: The size of a STARK proof is directly related to the complexity and length of the computation being proved. This means that for simple computations, the proof can be quite small, while for more complex ones, it can grow larger.
4. Post-Quantum Security: STARKs are believed to be secure against attacks from quantum computers.

However, these proofs can become quite large for complex computations, which can be problematic for on-chain verification on Solana.

### STARK to SNARK Conversion

To address the potential size issues of STARK proofs, Bonsol converts them into Groth16 SNARKs. This process involves several steps:

1. Proof Aggregation: In the case of using Proofs as Inputs, Bonsol may first aggregate multiple proof segments into a single, more compact proof.
2. Circuit Generation: The STARK verification circuit is transformed into an arithmetic circuit suitable for SNARK proving.
3. Trusted Setup: A one-time trusted setup is performed for the Groth16 scheme. This setup is universal for all STARK to SNARK conversions in Bonsol.
4. Proof Generation: Using the Groth16 scheme, a new SNARK proof is generated that attests to the validity of the original STARK proof.

### Benefits of Groth16 SNARKs

The conversion to Groth16 SNARKs offers several advantages:

1. Constant-size proofs: Regardless of the complexity of the original computation, the Groth16 SNARK proof has a fixed, small size.
2. Fast verification: Groth16 proofs can be verified extremely quickly, which is crucial for on-chain verification.
3. Efficient implementation: The algebraic structure of Groth16 proofs allows for efficient implementation on Solana.

### Native Verification on Solana

Bonsol implements a native Groth16 verifier on Solana, allowing for:

* Efficient proof verification, with the verification call happening in less than 200k compute units
* This means we can compose over other programs in the same transaction

### Input Digest Verification

To ensure the integrity of inputs, Bonsol:

1. Ensures that verifiable programs compute a digest (hash) of all inputs (public and private)
2. Commits this digest as part of the verifiable programs execution
3. Verifies the digest on-chain during proof verification

This additional step prevents potential attacks where a malicious prover might try to use different inputs than those specified in the execution request.\
