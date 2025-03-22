---
description: >-
  This tutorial gives a first example of using prover callbacks once the proof is ready. By the end, you'll understand
  how the prover node be automatically call Solana programs when it generates a proof.
icon: phone-arrow-up-right
---

# Using Callbacks

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

## Preparing the callback program

For this tutorial, we'll use the example callback program provided in the repo at `bonsol/onchain/example-program-on-bonsol`. It is a basic Solana native program that has three instructions, but we will only look at the [last one](https://github.com/bonsol-collective/bonsol/blob/39ece45389e8b5fb6faa6fd5001610a8f2e32d8b/onchain/example-program-on-bonsol/src/lib.rs#L129) for now.

Let's break down what happens in the instruction:

1. It uses the `bonsol-interface` to verify and deserialize the callback data.
2. It logs the callback output.
3. It checks that provided accounts match some hard-coded addresses.
4. It checks that the callback output is a single byte with value 1. This is the case if the public input is a JSON string with a property `attestation` that has the value `test` and the private input resolves to the value `test`, as seen in the simple program example.

## Creating and submitting the execution request

### Edit the execution request

Locate the sample execution request template at `bonsol/charts/input_files/simple_execution_request_callback.json`. Update the template with your specific `imageId` from the `manifest.json`file:

```json
# bonsol/charts/input_files/simple_execution_request_callback.json
{
  "imageId": "e13f590c2859117db29e210cc08263b1f0da3d3f74928791b129438083edfa31",
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
  "expiry": 1000,
  "callbackConfig": {
    "programId": "exay1T7QqsJPNcwzMiWubR6vZnqrgM16jZRraHgqBGG",
    "instructionPrefix": [2],
    "extraAccounts": [
      {
        "pubkey": "3b6DR2gbTJwrrX27VLEZ2FJcHrDvTSLKEcTLVhdxCoaf",
        "isSigner": false,
        "isWritable": true
      },
      {
        "pubkey": "g7dD1FHSemkUQrX1Eak37wzvDjscgBW2pFCENwjLdMX",
        "isSigner": false,
        "isWritable": false
      },
      {
        "pubkey": "FHab8zDcP1DooZqXHWQowikqtXJb1eNHc46FEh1KejmX",
        "isSigner": false,
        "isWritable": false
      }
    ]
  }
}
```

### Understanding the request

What changes from the [Simple Program tutorial](./simple-program.md#understanding-the-request) is the addition of the `callbackConfig` field. This tells the prover node to call the callback program after the proof is ready, using the `instructionPrefix` before sending the proof and passes the provided `extraAccounts`. Let's take a closer look at the `callbackConfig` field:

- `programId`: The address of the callback program. In this case, the address of the example callback program defined in the [`lib.rs` file](https://github.com/bonsol-collective/bonsol/blob/5157b677d676ddcd042e8b4d5b757191c7773a58/onchain/example-program-on-bonsol/src/lib.rs#L20).
- `instructionPrefix`: The instruction prefix to use when calling the callback program. In this case, `[2]` means that the prover node will call the third instruction of the program that we mentioned above.
- `extraAccounts`: The accounts to pass to the callback program, on top of the account of the prover node which acts as the signer of the transaction. In this case, we pass the hard-coded addresses of the example callback program (they are empty accounts only used to verify that they match what the program expects).

### Submit the execution request

```bash
bonsol execute -f charts/input_files/simple_execution_request_callback.json --wait
```

> :bulb: Note: Do not forget to update the `SIMPLE_IMAGE_ID` in the `lib.rs` file of the callback program to your actual image id. If you change it, restart the validator using `./bin/validator.sh -r`.

## Proof submission

Once the proof generates, you'll see the notification at your CLI:

```bash
bonsol execute -f charts/input_files/simple_execution_request_callback.json --wait
Execution expiry 34380
current block 24380
  Waiting for execution
  Execution completed with exit code Success
```

You can check your prover logs for the corresponding on-chain transaction:

```
{"timestamp":"2025-03-20T20:35:31.577353832Z","level":"INFO","fields":{"message":"Sending transaction... (https://explorer.solana.com/tx/4bTfj66z39SKJP618We65y4142L6FzR23jp3XeQPDfvE98Sx7gfQpR5AXLr7Gxp4yLVXEuAuRpKHhQpBNRxE8HqR?cluster=custom&customUrl=http://localhost:8899)"},"target":"bonsol_node::transaction_sender"}
⠚ [0/1] Finalizing transaction 4bTfj66z39SKJP618We65y4142L6FzR23jp3XeQPDfvE98Sx7gfQpR5AXLr7Gxp4yLVXEuAuRpKHhQpBNSending to runner
```

You can again check the prover's logs for the link to the transaction on the Solana explorer, and see that the attestation is correct.
