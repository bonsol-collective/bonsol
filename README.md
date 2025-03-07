# 乃ㄖ几丂ㄖㄥ
Bonsol is the Offchain compute framework to make everything possible on solana.

[![commitlint](https://github.com/bonsolcollective/bonsol/actions/workflows/commit-lint.yaml/badge.svg)](https://github.com/bonsolcollective/bonsol/actions/workflows/commit-lint.yaml)
[![Docker Build from Image CI](https://github.com/bonsolcollective/bonsol/actions/workflows/build-ci-image.yaml/badge.svg)](https://github.com/bonsolcollective/bonsol/actions/workflows/build-ci-image.yaml)

Interact with the docs at [Bonsol.sh](https://bonsol.sh)

# NOTE !!!!!
Do not use `node_keypair.json` in production, it is for local development only. 

## Requirements:
* Flat buffers 24.3.25
* For running the prover x86_64-linux due to (stark to snark tooling)[https://github.com/risc0/risc0/commit/7c6101f925e8fd1b3de09654941f0608d4459a2b]

## Scripts and Configuration

### Running a Node (`bin/run-node.sh`)
The node runner script provides several options for running a Bonsol node with different configurations:

```bash
./bin/run-node.sh [-F cuda] [-L] [-d]
```

#### Options
- `-F cuda`: Enable CUDA support for GPU acceleration
- `-L`: Use local build instead of installed bonsol
- `-d`: Enable debug logging for all relevant modules

#### Debug Mode Features
When running with `-d`:
- Detailed logging for all components
- System configuration display
- Core dump configuration
- Build information

#### System Configuration
The script automatically configures:
- Unlimited stack size
- Unlimited virtual memory
- Unlimited max memory size
- Core dumps enabled (stored in `/tmp/cores`)
- Detailed system limits display

#### Log Levels
- `error`: Show errors only
- `warn`: Show warnings and errors
- `info`: Show general information (default)
- `debug`: Show detailed debugging information
- `trace`: Show all possible logging information

#### Debug Components
- `risc0_runner`: Image downloads, proofs, and claims
- `transaction_sender`: Transaction processing and status
- `input_resolver`: Input processing and validation
- `reqwest`: HTTP client logs
- `hyper`: Low-level HTTP details

## Roadmap
Stage 1: Dawn (current stage)
* Developer feedback
    * New features 
        * Interfaces
            * More Ingesters, Senders
            * More Input Types
        * Adding Integrations
            * Zktls,web proofs, client proving
    * Node Ops
        * Claim based prover network (SOL)
        * Prover Supply Integrations
* Community Building

## Contributing and Local Development 
Please see our [Contributing Guide](https://bonsol.sh/docs/contributing) for details on how to get started building 乃ㄖ几丂ㄖㄥ.
