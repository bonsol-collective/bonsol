---
description: >-
  This guide provides instructions for setting up a local Bonsol development
  environment. Whether you're contributing to the project or building with
  Bonsol, this documentation will help.
icon: pen-to-square
---

# Setup a local environment

## Requirements

:bulb: In this section, you'll be running a local Bonsol proving node. Currently provers are limited to running on x86\_64-linux systems due to dependencies on [STARK-to-SNARK](https://bonsol.gitbook.io/docs/core-concepts/introduction#stark-to-snark-conversion) tooling. We're looking for workarounds for MacOS, but in the meantime we suggest developing on a remote Linux machine.

Before you begin, ensure you have the following system requirements:

* [Rust](https://solana.com/docs/intro/installation#install-rust)
* [Solana CLI](https://solana.com/docs/intro/installation#install-the-solana-cli)
* [pnpm/pnpx](https://pnpm.io/installation)

Verify you have these requirements by running:

```bash
cargo --version
rustc --version
solana --version
pnpm --version
```

## Local environment setup&#x20;

{% stepper %}
{% step %}
### Clone the repository

```bash
git clone https://github.com/bonsol-collective/bonsol
cd bonsol
```
{% endstep %}

{% step %}
### Install the RISC Zero prover

```bash
# Install the prover to the default location (current directory)
./bin/install_prover.sh

# Or specify a custom installation location
./bin/install_prover.sh --prefix /path/to/install
```
{% endstep %}

{% step %}
### Run the setup script

* Checks that the STARK verification tools are installed
* Generates and parses the verification key for on-chain use

```bash
# Set up local environment
./bin/setup.sh

# Or specify a custom installation prefix if you used one for install_prover.sh
./bin/setup.sh --prefix /path/to/install
```
{% endstep %}

{% step %}
### Run the Solana validator script

* Builds the Solana BPF programs using `cargo build-sbf`
* Starts a local Solana validator with the Bonsol program at address `BoNsHRcyLLNdtnoDf8hiCNZpyehMC4FDMxs6NTxFi3ew`
* Includes a callback example program
* Allows adding additional BPF programs with their addresses

```bash
# Start a local validator
./bin/validator.sh

# Or run the local validator with the reset option
./bin/validator.sh -r 
```
{% endstep %}

{% step %}
### Run a local Bonsol node

* Creates a new node keypair if one doesn't exist
* Airdrop SOL to the node keypair for transaction fees
* Run the Bonsol node with the appropriate hardware acceleration:
  * Linux: CPU or CUDA (if `-F cuda` flag is used)

```bash
# Start a node with default CPU configuration
./bin/run-node.sh

# For Linux systems with CUDA support
./bin/run-node.sh -F cuda
```
{% endstep %}
{% endstepper %}

## Troubleshooting

* If the prover installation fails, check your internet connection and try increasing the `--job-timeout` value.
  * `--job-timeout`: Set timeout for download operations in seconds (default: 3600)

```bash
# Install the prover with an increased timeout
./bin/install_prover.sh --job-timeout 7200
```

* If the validator fails to start, ensure that Rust and Solana CLI tools are properly installed
* For node startup issues, verify that the validator is running and that SOL was successfully airdropped to your node keypair
