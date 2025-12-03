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
bonsol build --zk-program-path <ZK_PROGRAM_PATH>
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
* `url`: Deploy with a custom URL (e.g. localhost)

<details>

<summary>S3 Deployment</summary>

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

</details>

<details>

<summary>URL</summary>

The `bonsol deploy url` command allows you to deploy your program by either uploading your binary to a URL endpoint or using an existing binary at a URL.

#### Usage

```warp-runnable-command
bonsol deploy url --url <URL> --manifest-path <MANIFEST_PATH> [OPTIONS]
```

#### Required Arguments

* `--url <URL>`
* The base URL endpoint for your binary
* Example: `http://localhost:8080`
* The actual binary will be stored at `<URL>/<program-name>-<image-id>`
* `--manifest-path <MANIFEST_PATH>`
* Path to your program's manifest file (manifest.json)
* Example: `images/simple/manifest.json`

#### Optional Arguments

* `--no-post`
* By default, the command uploads your binary to the URL
* With this flag, it instead verifies that the correct binary already exists at the URL
* Useful when your binary is already hosted and you just want to deploy it to Solana
* `--auto-confirm` or `-y`
* Skip the confirmation prompt for Solana deployment
* Use with caution as deployments cost real money

#### Examples

1. Upload and deploy a new binary:

```warp-runnable-command
bonsol deploy url \
    --url http://localhost:8080 \
    --manifest-path images/simple/manifest.json
```

2. Deploy using an existing binary (verifies the binary first):

```warp-runnable-command
bonsol deploy url \
    --url http://localhost:8080 \
    --manifest-path images/simple/manifest.json \
    --no-post
```

#### How It Works

1. The command constructs the full URL by appending your program name and image ID:

```warp-runnable-command
   <base-url>/<program-name>-<image-id>
```

For example: `http://localhost:8080/simple2-ec93e0a9592a2f00c177a7fce6ff191019740ff83f589e334153126c02f5772e`

2. Without `--no-post` (default):

* POSTs your binary to this URL
* Proceeds with Solana deployment after successful upload

3. With `--no-post`:

* Attempts to GET the binary from this URL
* Verifies it matches your local binary
* Only proceeds with Solana deployment if verification succeeds

#### Common Errors

1. "Binary does not match":

```warp-runnable-command
   Error: The binary uploaded does not match the local binary at path '...'

```

* This occurs when using `--no-post` and either:
  * No binary exists at the URL
  * The binary at the URL is different from your local binary

2. "Failed to connect":

* Check that your URL endpoint is accessible
* Ensure you have the correct permissions

#### Notes

* The command always requires a local binary for verification, even when using `--no-post`
* Deployments to Solana are immutable and cost real money
* The URL endpoint must support both POST and GET operations

</details>

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

:bulb: Note: Only private local inputs are supported for the prove command.\\
