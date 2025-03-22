---
description: >-
  This tutorial gives an example of requesting a ZK program execution from within a Solana program and using the result with a callback.
  By the end, you'll understand how to create complex workflows directly on-chain.
icon: phone-arrow-up-right
---

# Using Callbacks From Programs

## Setting up your environment

Similar to the [Simple Program tutorial](./simple-program.md), you'll work from the Bonsol repository and get a local validator and Bonsol prover node running.

### Start the Local Validator

The validator script builds and deploys necessary Solana programs, including the Bonsol core program and an example callback program.

```bash
./bin/validator.sh
```

> :bulb: Note: Keep this terminal window open as the validator needs to run throughout the tutorial.

### Run the Bonsol Prover Node

The prover node processes the off-chain computation. Open a new terminal and run:

```bash
./bin/run-node.sh
```

> :bulb: Note: Keep this terminal window open as the prover node needs to run throughout the tutorial.

### Run the Local ZK Program Server

This stores compiled ZK programs and serves them to the prover node. Open a new terminal and run:

```bash
cargo run -p local-zk-program-server
```

> :bulb: Note: Keep this terminal window open as the prover node needs to run throughout the tutorial.

## Deploying the ZK program

For this tutorial, we'll reuse the simple ZK program provided in the repo at `bonsol/images/simple/src/main.rs`. For more information on the program, see the [Simple Program tutorial](./simple-program.md).

To build it and generate the `manifest.json` file, run:

```bash
bonsol build --zk-program-path ./images/simple
```

Next, deploy the program to the local ZK program server and register it on-chain:

```bash
bonsol deploy url \
    --url http://localhost:8080 \
    --post \
    --manifest-path ./images/simple/manifest.json
```

## The Callback Program

For this tutorial, we'll use the example callback program provided in the repo at `bonsol/onchain/example-program-on-bonsol`. It is a basic Solana native program that has three instructions, but we will only look at the [first two instructions](https://github.com/bonsol-collective/bonsol/blob/39ece45389e8b5fb6faa6fd5001610a8f2e32d8b/onchain/example-program-on-bonsol/src/lib.rs#L37) for now. The first instruction will setup the execution of the ZK program, while the second will be the callback used by the prover once the proof is ready.

Let's break down what happens in the first instruction:

1. It checks and deserializes instruction data. It must contain an execution ID, the has of the input values, the expiration time of the execution request, the bump of the Program Derived Address (PDA) used to create the request, and the private input URL.
2. It creates the PDA of the callback program used to create the execution request.
3. It calls the Bonsol program to create the execution request. This specifies parameters like the callback parameters, the tip for the prover, etc.
4. It stores the public key of the execution request inside the requester PDA.

The second instruction is the callback and does the following:

1. Gets the address of the execution request from the requester PDA.
2. Verifies callback data and deserializes the output.
3. Checks that the extra accounts match the expected addresses.
4. Checks that the output of the ZK program is a single-byte integer equal to 1.

## Triggering the workflow

Because we are create the execution request directly from the callback program, we need to create a transaction that calls the first instruction. For this purpose, you can use the simple program in [`bin/callback-example-runner`](../../../bin/callback-example-runner) to create the transaction:

```bash
cargo run -p callback-example-runner
```

> :bulb: Note: Do not forget to update the `SIMPLE_IMAGE_ID` in the `lib.rs` file of the callback program to your actual image id. If you change it, restart the validator using `./bin/validator.sh -r`.

## Viewing the result

Once the transaction is sent, the prover node will automatically detect it and claim it. It then generates the proof and sends the proof with a transaction to the callback program. You can see this happening in the prover logs:

```bash
{"timestamp":"2025-03-20T20:35:31.577353832Z","level":"INFO","fields":{"message":"Sending transaction... (https://explorer.solana.com/tx/4bTfj66z39SKJP618We65y4142L6FzR23jp3XeQPDfvE98Sx7gfQpR5AXLr7Gxp4yLVXEuAuRpKHhQpBNRxE8HqR?cluster=custom&customUrl=http://localhost:8899)"},"target":"bonsol_node::transaction_sender"}
⠚ [0/1] Finalizing transaction 4bTfj66z39SKJP618We65y4142L6FzR23jp3XeQPDfvE98Sx7gfQpR5AXLr7Gxp4yLVXEuAuRpKHhQpBNSending to runner
```

You can again check the prover's logs for the link to the transaction on the Solana explorer, and see that the attestation is correct.
