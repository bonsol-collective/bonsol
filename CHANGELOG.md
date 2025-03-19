# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

### Fixed
* Fix S3 path duplication in deploy command causing provers to be unable to download images

## [0.4.5] - 2024-02-26

### Changed
* `bonsol` cli option requirements and error messages updated for added clarity
* **Breaking**: `bonsol deploy` cli subcommand requirements updated. Please refer to the docs, or use `bonsol deploy --help` for more info.
* Fixes corner cases in bonsol program where there is no callback program.
* Pins cli to a cargo risc0 version
* Upgrade to solana 2.0 and risc0 zkvm 1.2.1
* Publish crates

### Added
* `bonsol estimate` for estimating execution cost of bonsol programs.

### Fixed
* **Breaking**: `execute_v1` interface instruction now uses the new `InputRef` to improve CU usage.
* Adds a callback struct to use the input_hash and committed_outputs from the callback program ergonomically.
* Fixes requester/payer mismatch in the node account selection
* **Breaking**: Forwards input hash to the callback program in all cases.
* **Breaking**: Changes flatbuffer `Account` struct to have 8 byte alignment due a possible bug in the flatbufers compiler. [https://github.com/google/flatbuffers/pull/8398](Bug Here)
* **Breaking**: Flatbuffers was upgraded to `24.3.25`
* `risc0-groth16-prover` binaries (rapidsnark & stark-verify) are available to the nix store, partially unblocking NixOS support.
* `flatbuffers` code is now dynamically generated at build time
* Fixed alignment of `Account` struct in the schemas.

## [0.2.1] - 2024-10-13

### Changed
* **Breaking**: `relay` was renamed to `bonsol-node`.
* **Breaking**: `relaykp.json` was renamed to `node_keypair.json`, and is no longer recognized by that name.
* **Breaking**: `bonsol-channel` was renamed to `bonsol`.
* **Breaking**: `bonsol-channel-interface` and `bonsol-channel-utilities` were merged into a single crate, `bonsol-interface`.
* `run-relay.sh`, a script for building and running a bonsol node was renamed to `run-node.sh`.
* **Breaking**: Proving and input resolution functionality was removed from `bonsol-sdk`, and placed in a new crate, `bonsol-prover`.
* Naming conventions across the board were updated in documentation accordingly.

## [0.2.0] - 2024-10-11

### Added
* A nix flake was added which brings with it a development environment and CI checks.
* A contributing guide was added to the docs, and linked to the README.md at the root of the project.
* Docker Cuda options enabled.

### Fixed
* Fixed a bug that used block height instead of slot on the cli to determining expiry, leading to short claim expiry.
* Fixed `libsodium`, cli fixes and docker harness.
