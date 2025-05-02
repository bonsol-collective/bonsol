---
description: Let's create a hello world project using our newly installed Bonsol CLI.
icon: globe-pointer
---

# Quickstart

{% stepper %}
{% step %}
#### Verify installation

```bash
$ bonsol --version
bonsol-cli 0.4.5
```
{% endstep %}

{% step %}
#### Initialize a new project

The `init` command creates a new verifiable program with the basic project structure and configuration needed to get started.

{% hint style="info" %}
Note: We suggest using an _underscore_ when initializing multi-word projects as this can prevent issues with the downstream `cargo risczero` docker build process.
{% endhint %}

```bash
$ bonsol init --project-name say_hello
Project 'tutorial' initialized successfully!
```

Project structure:

```
say_hello/
├── Cargo.toml
├── README.md
└── src
    └── main.rs
```

The generated project includes a `Cargo.toml` with special metadata for your verifiable program's inputs:

```
[package.metadata.zkprogram]
input_order = ["Public"]
```

Valid input options are: `["Public", "Private", "PublicProof"]`.
{% endstep %}

{% step %}
#### Write a verifiable program

Navigate to `src/` and inspect `main.rs`:

```rust
// src/main.rs

use risc0_zkvm::{guest::{env, sha::Impl},sha::{Sha256}};

fn main() {
    let mut input_1 = Vec::new();
    env::read_slice(&mut input_1);
    let digest = Impl::hash_bytes(&input_1.as_slice());
    env::commit_slice(digest.as_bytes());
}
```

Run `cargo build` to make sure everything builds correctly:

```bash
cargo build
   Compiling say_hello v0.1.0 (/Users/chris/say_hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s
```
{% endstep %}

{% step %}
#### Build the verifiable program

The `build` command compiles your verifiable program and generates a manifest file containing the deployment information. Ensure your Docker daemon is running and build your verifiable program:

```bash
$ bonsol build --zk-program-path .
Build complete
```

This generates a `manifest.json` file containing:

* Program name
* Binary path
* Image ID
* Input order configuration
* Cryptographic signature
* Program size

Example `manifest.json`:

```json
{
  "name": "say_hello",
  "binaryPath": "./target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/say_hello/say_hello",
  "imageId": "6700902caf52fb56277157db725faa5c1aeac0c08221d2e13e27430da2f77136",
  "inputOrder": [
    "Public"
  ],
  "signature": "k7XUcgk94oxsLpLZwzCQ3SdrZ5tq4TsCPW8paBC4JnDtKXMknwJ7MMENXs5ijFL2wDKAzFLrvFKGZCpFMPmRfo9",
  "size": 116744
}
```
{% endstep %}

{% step %}
#### Deploy the verifiable program

Verify you're on devnet:

```bash
$ solana config get
Config File: /Users/<user>/.config/solana/cli/config.yml
RPC URL: https://api.devnet.solana.com
WebSocket URL: wss://api.devnet.solana.com/ (computed)
Keypair Path: /Users/<user>/.config/solana/id.json
Commitment: confirmed
```

The `deploy` command uploads your newly built verifiable program to make it accessible to the prover network. We currently support S3-compatible storage hosts.

```bash
$ bonsol deploy s3 \
    --bucket <bucket-name> \
    --access-key <access-key> \
    --secret-key <secret-key> \
    --manifest-path <path-to-manifest.json>
Uploaded to S3 url https://bonsol.s3.us-east-1.amazonaws.com/say_hello-6700902caf52fb56277157db725faa5c1aeac0c08221d2e13e27430da2f77136
```

You'll be prompted to continue, press `y`:

```
Deploying to Solana, which will cost real money. Are you sure you want to continue? (y/n)
y
6700902caf52fb56277157db725faa5c1aeac0c08221d2e13e27430da2f77136 deployed
```
{% endstep %}

{% step %}
#### Create an execution request

An execution request is specified in a JSON file with the following structure:

```json
{
  "imageId": "6700902caf52fb56277157db725faa5c1aeac0c08221d2e13e27430da2f77136",
  "executionConfig": {
    "verifyInputHash": false,
    "forwardOutput": true
  },
  "inputs": [
    {
      "inputType": "PublicData",
      "data": "Hello, world!"
    }
  ],
  "tip": 12000,
  "expiry": 1000
}
```

**Required Fields**

* **imageId:** The unique identifier of the verifiable program image to execute. This is generated when building your program and is found in your `manifest.json`.
* **inputs**: An array of input objects that will be passed to the verifiable program. This program just uses one input.
* **inputType**: Type of input data.
* **data**: The actual input data, properly formatted as a string.
{% endstep %}

{% step %}
#### Execute the verifiable program

Use the `execute` command to submit your execution request to the prover network:

```bash
$ bonsol execute -f execution-request.json --wait
Execution expiry 35436
current block 34436
  Claimed by 5RChCvEt8z5Uq9DF2yv2sJeazgm1SFmJChm1mrHH35oU at slot 34469, committed 17718
  Execution completed with exit code Success
```

* If the execution is not completed within the timeout period, you'll receive a timeout message.
* If the execution request expires without being claimed, you'll receive an expiry message.
{% endstep %}
{% endstepper %}

Congratulations! You've deployed your first verifiable program on Solana. See the [setup-a-local-environment.md](../developers/setup-a-local-environment.md "mention") section to build a more fleshed out development environment to build fully unstoppable applications.
