---
description: >-
  The Bonsol CLI is a command-line interface for creating, building, deploying,
  and interacting with verifiable programs on Solana.
icon: square-terminal
---

# CLI Commands

## General Usage

Most Bonsol commands accept the following global arguments:

* `-c`or `--config`: Path to the config file
* `-k`or `--keypair`: Path to the keypair file
* `-u`or `--rpc-url`: URL for the Solana RPC

If these arguments aren't provided, Bonsol will use the default Solana config located in `~/.config/solana/`. For example:

```bash
bonsol -k ./keypair.json -u http://localhost:8899 [COMMAND]
```

## Commands

### init: Creating a New Bonsol Program

Initialize a new Bonsol project with the following command:

```bash
bonsol init --project-name <PROJECT_NAME> [--dir <DIR>]
```

Options:

* `-n`, `--project-name <PROJECT_NAME>`: Name of your new project (required)
* `-d`, `--dir <DIR>`: Directory where the project will be created

This command creates a new Bonsol program structure in the specified directory.

### build: Building a Bonsol ZK Program

Build your zero-knowledge program using:

```bash
build --zk-program-path <ZK_PROGRAM_PATH>
```

Options:

* `-z`, `--zk-program-path <ZK_PROGRAM_PATH>`: Path to a ZK program folder containing a Cargo.toml (required)

This command builds your ZK program and creates a `manifest.json`file in the program directory, containing all necessary information for deployment. Example `manifest.json`:

```json
{
  "name": "simple",
  "binaryPath": "images/simple/target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/simple/simple",
  "imageId": "20b9db715f989e3f57842787badafae101ce0b16202491bac1a3aebf573da0ba",
  "inputOrder": [
    "Public",
    "Private"
  ],
  "signature": "3mdQ6RUV5Bw9f1oUJhfif4GqVQpE8Udcu7ZR5NjDeyEx5ls2aRxD74DC5v1d251q6c9Q4m523a5a1h0nOO5f+s",
  "size": 266608
}
```

### deploy: Deploying a Bonsol ZK Program

After building your ZK program, you can deploy it using various storage options:

```bash
bonsol deploy <COMMAND>
```

Commands:

* `s3`: Deploy using an AWS S3 bucket
* `url`: Deploy manually with a URL

#### S3 Deployment

First, create an S3 bucket (skip this if you already have one):

```bash
aws s3api create-bucket \
    --bucket <BUCKET_NAME> \
    --region <REGION> \
    --create-bucket-configuration LocationConstraint=<REGION>
```

Then deploy your ZK program to your S3 bucket:

```bash
bonsol deploy s3 \
    --bucket <BUCKET_NAME> \
    --access-key <ACCESS_KEY> \
    --secret-key <SECRET_KEY> \
    --region <REGION> \
    --manifest-path <PATH_TO_MANIFEST> \
    --storage-account s3://<BUCKET_NAME>
```

### execute: Requesting Execution

Request execution of your ZK program:

```bash
bonsol execute [OPTIONS]
```

Options:

The execution request file should be a JSON file with the following structure:

* `-f`, `--execution-request-file <EXECUTION_REQUEST_FILE>`: Path to execution request JSON file
* `-p`,`--program-id <PROGRAM_ID>`: Program ID
* `-e`, `--execution-id <EXECUTION_ID>`: Execution ID
* `-x`, `--expiry <EXPIRY>`: Expiry for the execution
* `-m`, `--tip <TIP>`: Tip amount for execution
* `-i`, `--input-file <INPUT_FILE>`: Override inputs in execution request file
* `-w`, `--wait`: Wait for execution to be proven
* `-t`, `--timeout <TIMEOUT>`: Timeout in seconds

The execution request file should be a JSON file with the following structure:

```json
{
  "imageId": "20b9db715f989e3f57842787badafae101ce0b16202491bac1a3aebf573da0ba",
  "executionId": "9878798-987987-987987-987987",
  "tip": 100,
  "maxBlockHeight": 100,
  "inputs": [
    {
      "inputType": "Public",
      "data": "<base64 encoded data>"
    }
  ],
  "callbackConfig": {
    "programId": "your program id",
    "instructionPrefix": [0, 1, 2, 3],
    "extraAccounts": [
      {
        "address": "",
        "role": "writable"
      }
    ]
  },
  "executionConfig": {
    "verifyInputHash": true,
    "forwardOutput": true,
    "inputHash": "<hex encoded sha256 hash of the input data>"
  }
}
```

If you pass the `--wait` flag, the CLI will wait for execution completion and display the result:

```
Execution 9878798-987987-987987-987987 completed successfully
```

### prove: Local Proving with the CLI

Perform local proving against a deployed program:

```bash
bonsol prove --execution-id <EXECUTION_ID> [OPTIONS]
```

Options:

* `-m`, `--manifest-path <MANIFEST_PATH>`: Path to the manifest file
* `-p`, `--program-id <PROGRAM_ID>`: Program ID
* `-i <INPUT_FILE>`: Input file
* `-e`, `--execution-id <EXECUTION_ID>`: Execution ID (required)
* `-o <OUTPUT_LOCATION>`: Output location for the proof

You can provide inputs in a JSON file:

```json
{
  "imageId": "20b9db715f989e3f57842787badafae101ce0b16202491bac1a3aebf573da0ba",
  "inputs": [
    {
      "inputType": "PrivateLocal",
      "data": "<base64 encoded data>"
    }
  ]
}
```

Or pipe inputs directly:

```bash
echo '"{"attestation":"test"}" "nottest"' | bonsol prove -e <execution_id> -m images/simple/manifest.json
```

If proving succeeds, the CLI will save a serialized RISC-0 receipt file named `<execution_id>.bin` in the current directory or the specified output location.

:bulb: Note: Only private local inputs are supported for the prove command.\
