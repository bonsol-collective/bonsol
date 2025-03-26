---
description: >-
  This tutorial guides you through creating, building, and deploying a
  zero-knowledge program using Bonsol on Solana. By the end, you'll understand
  how to create ZK proofs that can be verified on-chain.
icon: space-awesome
---

# Tutorial: Simple Program

## Setting up your environment

### Setup a local environment

Refer to the [Setup a local environment](./setup-a-local-environment.md) page for instructions on setting up a local environment.

### Start the Local Validator

The validator script builds and deploys necessary Solana programs, including the Bonsol core program and an example callback program (not used in this tutorial).

```bash
$ ./bin/validator.sh

./bin/validator.sh -r
++ which cargo
+ '[' '!' -x /home/ubuntu/.cargo/bin/cargo ']'
+ cargo build-sbf
   Compiling ...
   Compiling bonsol-interface v0.4.5 (/home/ubuntu/bonsol/onchain/interface)
   Compiling callback-example v0.4.5 (/home/ubuntu/bonsol/onchain/example-program-on-bonsol)
    Finished `release` profile [optimized] target(s) in 18.51s
+ solana-test-validator --limit-ledger-size 0 --bind-address 0.0.0.0 --rpc-pubsub-enable-block-subscription --bpf-program BoNsHRcyLLNdtnoDf8hiCNZpyehMC4FDMxs6NTxFi3ew target/deploy/bonsol.so --bpf-program exay1T7QqsJPNcwzMiWubR6vZnqrgM16jZRraHgqBGG target/deploy/callback_example.so -r
Ledger location: test-ledger
Log: test-ledger/validator.log
⠠ Initializing...                                                                                                        Waiting for fees to stabilize 1...
Identity: Bdudyg3GB4Gw3we7g9RCLBnL3E9TJ1N2bfsZTFAaocv6
Genesis Hash: HWEv5jLYLrzdxsEXcc56dkSV96b7h8cYwSPgKyR77a6Q
Version: 2.1.14
Shred Version: 64458
⠉ 00:18:59 | Processed Slot: 2789 | Confirmed Slot: 2789 | Finalized Slot: 2758 | Full Snapshot Slot: 2700 | Incremental Snapshot Slot: - | Transactions: 2788 | ◎499.986060000
```

> :bulb: Note: Keep this terminal window open as the validator needs to run throughout the tutorial.

### Run the Bonsol Prover Node

The prover node processes the off-chain computation. Open a new terminal and run:

```bash
$ ./bin/run-node.sh

Bonsol node keypair exists
Requesting airdrop of 1 SOL

Signature: 5xBZKBZhk9Zn9HdXSuo9w6pqzdiQTq6hriWy5n4xNKu6oD5DoZdih1gfAnjL4gzKr8wcv3DQ553uYGrdpKUWK7ta

1 SOL
Requesting airdrop of 1 SOL

Signature: 53GJFP1HMopNuyXfnQSxP7m49MZ2HfTTfRvVkMLdCk3dkzgKr8whfzQw7amWmVz2BP3zU6GhwJ913qD9V6VdPjS2

500000001 SOL
   Compiling bonsol-schema v0.4.5 (/home/ubuntu/bonsol/schemas)
   Compiling bonsol-prover v0.4.5 (/home/ubuntu/bonsol/prover)
   Compiling bonsol-interface v0.4.5 (/home/ubuntu/bonsol/onchain/interface)
   Compiling bonsol-node v0.4.5 (/home/ubuntu/bonsol/node)
    Finished `release` profile [optimized] target(s) in 26.01s
     Running `target/release/bonsol-node -f ./Node.toml`
{"timestamp":"2025-03-11T06:57:50.284199771Z","level":"INFO","fields":{"message":"Event: BonsolStartup","event":"BonsolStartup","up":true},"target":"bonsol_node"}
{"timestamp":"2025-03-11T06:57:50.284232745Z","level":"INFO","fields":{"message":"Using Keypair File"},"target":"bonsol_node"}
{"timestamp":"2025-03-11T06:57:50.284454687Z","level":"INFO","fields":{"message":"Using RPC Block Subscription"},"target":"bonsol_node"}
{"timestamp":"2025-03-11T06:57:50.384950857Z","level":"INFO","fields":{"message":"Loaded image: 7cb4887749266c099ad1793e8a7d486a27ff1426d614ec0cc9ff50e686d17699"},"target":"bonsol_node::risc0_runner"}
{"timestamp":"2025-03-11T06:57:50.453175244Z","level":"INFO","fields":{"message":"Loaded image: f899f7bf9823d6e1dab99f8a33a4e203f0341d1a0be98a6b8e07c25e834571a0"},"target":"bonsol_node::risc0_runner"}
{"timestamp":"2025-03-11T06:57:50.520950383Z","level":"INFO","fields":{"message":"Loaded image: 4fe2a1e650dc0ba12e58bccb07c66b23fe5a3ff90e2bd06dfddf87576f3f3b22"},"target":"bonsol_node::risc0_runner"}
{"timestamp":"2025-03-11T06:57:50.593944246Z","level":"INFO","fields":{"message":"Loaded image: 7cb4887749266c099ad1793e8a7d486a27ff1426d614ec0cc9ff50e686d17699"},"target":"bonsol_node::risc0_runner"}
{"timestamp":"2025-03-11T06:57:50.667674793Z","level":"INFO","fields":{"message":"Loaded image: 20b9db715f989e3f57842787badafae101ce0b16202491bac1a3aebf573da0ba"},"target":"bonsol_node::risc0_runner"}
{"timestamp":"2025-03-11T06:57:50.74100063Z","level":"INFO","fields":{"message":"Loaded image: 68f4b0c5f9ce034aa60ceb264a18d6c410a3af68fafd931bcfd9ebe7c1e42960"},"target":"bonsol_node::risc0_runner"}
{"timestamp":"2025-03-11T06:57:50.808491816Z","level":"INFO","fields":{"message":"Loaded image: 6700902caf52fb56277157db725faa5c1aeac0c08221d2e13e27430da2f77136"},"target":"bonsol_node::risc0_runner"}
{"timestamp":"2025-03-11T06:57:50.810318595Z","level":"INFO","fields":{"message":"Risc0 Prover with digest c101b42bcacd62e35222b1207223250814d05dd41d41f8cadc1f16f86707ae15"},"target":"bonsol_node::risc0_runner::verify_prover_version"}

```

> :bulb: Note: Keep this terminal window open as the prover node needs to run throughout the tutorial.

### Run the Local ZK Program Server

As explained in the [architecture](../core-concepts/architecture.md.md) page, provers on the network need to fetch the ZK programs and the input data used to generate the proof. The methods used to fetch these resources are stored on-chain by the ZK program developer.

For the purpose of local development, we will use a local HTTP server to host the ZK program data. This server stores everything in memory, so it will reset when the server is restarted. Open a new terminal and run:

```bash
$ cargo run -p local-zk-program-server
   Compiling local-zk-program-server v0.4.5 (/home/ubuntu/bonsol/local-zk-program-server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.56s
     Running `target/debug/local-zk-program-server`
Server is running on 0.0.0.0:8080
```

> :bulb: Note: Keep this terminal window open as the prover node needs to run throughout the tutorial.

## Writing the ZK program

Let's examine the simple ZK program provided in the repo at `bonsol/images/simple/src/main.rs`.

```rust
# bonsol/images/simple/src/main.rs

use gjson::Kind;
use risc0_zkvm::{guest::{env, sha::Impl},sha::{Digest, Sha256}};

fn main() {
    let mut public1 = Vec::new();
    env::read_slice(&mut public1);
    let publici1 = String::from_utf8(public1).unwrap();
    let mut private2 = Vec::new();
    env::read_slice(&mut private2);
    let privatei2 = String::from_utf8(private2).unwrap();
    let valid = gjson::valid(&publici1);
    let mut res = 0;
    if valid {
        let val = gjson::get(&publici1, "attestation");
        if val.kind() == Kind::String && val.str() == privatei2 {
            res = 1;
        }
    }
    let digest = Impl::hash_bytes(
        &[
            publici1.as_bytes(),
            privatei2.as_bytes(),
        ].concat(),
    );
    env::commit_slice(digest.as_bytes());
    env::commit_slice(&[res]);
}

```

This simple program demonstrates private input validation, where only the prover knows the private input, but anyone can verify the result. Here's how the program works:

1. Reads two inputs
   - public1: A JSON string with an "attestation" field
   - private2: A private string to compare against the attestation
2. Validates if
   - The public input is valid JSON
   - The "attestation" field in the JSON matches the private input
3. Outputs
   - A cryptographic digest of both inputs
   - A result (1 for match, 0 for no match)

## Building the ZK program

Now that we understand the program, let's build it:

```bash
bonsol build --zk-program-path ./images/simple
```

This compiles the Rust code into a format compatible with the RISC Zero VM and generates a `manifest.json` file containing:

```json
{
  "name": "simple2",
  "binaryPath": "./images/simple/target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/simple2/simple2",
  "imageId": "ec93e0a9592a2f00c177a7fce6ff191019740ff83f589e334153126c02f5772e",
  "inputOrder": ["Public", "Private"],
  "signature": "5PdbBK1A5Qtyg1P6GUbMLt2eG4VSPfYRMaGsPoxJRwoQzJbAgkFx9N5nafTHxpdG5d2CUqVUsBfUgWijyEBXtxqH",
  "size": 279880
}
```

> :bulb: Important: Take note of the `imageId` as you'll need it for the next steps. This uniquely identifies your ZK program in the network.

## Deploying the ZK program

Next, deploy the program to the local ZK program server and register it on-chain:

```bash
$ bonsol deploy url \
    --bucket bonsol \
    --url http://localhost:8080 \
    --post \
    --manifest-path ./images/simple/manifest.json

Program available at URL https://localhost:8080/simple2-ec93e0a9592a2f00c177a7fce6ff191019740ff83f589e334153126c02f5772e
Deploying to Solana, which will cost real money. Are you sure you want to continue? (y/n)
y
ec93e0a9592a2f00c177a7fce6ff191019740ff83f589e334153126c02f5772e deployed
```

> 💡 Note: When deploying to mainnet, this operation costs SOL to register your program on-chain.

## Creating and submitting an execution request

### Edit the execution request

Locate the sample execution request template at `bonsol/charts/input_files/simple_execution_request.json`. Update the template with your specific `imageId` from the `manifest.json`file:

```json
# bonsol/charts/input_files/simple_execution_request.json
{
  "imageId": "ec93e0a9592a2f00c177a7fce6ff191019740ff83f589e334153126c02f5772e",
  "executionConfig": {
    "verifyInputHash": false,
    "forwardOutput": true
  },
  "inputs": [
    {
      "inputType": "PublicData",
      "data": "{\"attestation\":\"test\"}"
    },
    {
      "inputType": "Private",
      "data": "https://echoserver.dev/server?response=N4IgFgpghgJhBOBnEAuA2mkBjA9gOwBcJCBaAgTwAcIQAaEIgDwIHpKAbKASzxAF0+9AEY4Y5VKArVUDCMzogYUAlBlFEBEAF96G5QFdkKAEwAGU1qA"
    }
  ],
  "tip": 12000,
  "expiry": 1000
}

```

### Understanding the request

- `imageId`: The identifier for your ZK program
- `executionConfig`: Configuration for execution behavior
- `inputs`: The inputs to your program (must match the order in manifest.json)
- First input: Public JSON data with `"attestation":"test"`
- Second input: Private data (URL-encoded and hosted remotely)
- `tip`: The amount to pay the prover (in lamports)
- `expiry`: Number of blocks until the request expires

### Submit the execution request

```bash
$ bonsol execute -f charts/input_files/simple_execution_request.json --wait

Execution expiry 13235
current block 13135
  Waiting for execution
  Claimed by 5RChCvEt8z5Uq9DF2yv2sJeazgm1SFmJChm1mrHH35oU at slot 13168, committed 6617
```

The `--wait` flag makes the command wait until the execution is complete. You should see:

1. The block at which your request will expire
2. When a prover claims your request
3. When the proof is committed on-chain

## Proof submission

Once the proof generates, you'll see the notification at your CLI:

```bash
bonsol execute -f charts/input_files/simple_execution_request.json --wait
Execution expiry 34380
current block 24380
  Waiting for execution
  Execution completed with exit code Success
```

You can check your prover logs for the corresponding on-chain transaction:

```
⠒ [0/1] Finalizing transaction 2j6iCbz8fKid1MKVKhB9QzvbAiziT4q4xeANAiEPP5DpQuwRTqAycD4JKJ3ca1pHkwNXxQ1fSJKmBdPFXYSrihtA  Sending to runner
{"timestamp":"2025-03-11T09:24:48.933897605Z","level":"INFO","fields":{"message":"Proof submitted: 2j6iCbz8fKid1MKVKhB9QzvbAiziT4q4xeANAiEPP5DpQuwRTqAycD4JKJ3ca1pHkwNXxQ1fSJKmBdPFXYSrihtA"},"target":"bonsol_node::risc0_runner"}
```
