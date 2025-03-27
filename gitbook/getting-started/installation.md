---
description: >-
  Start by installing the Bonsol CLI which provides you with all the necessary
  tools for starting a new project. The Bonsol CLI is compatible with both Linux
  and macOS operating systems.
icon: bullseye-arrow
---

# Installation

{% hint style="info" %}
Interested in contributing? Head over to the [broken-reference](broken-reference/ "mention") section to learn more.
{% endhint %}

## Requirements

* [Rust](https://solana.com/docs/intro/installation#install-rust)
* [Solana CLI](https://solana.com/docs/intro/installation#install-the-solana-cli)
* [Docker](https://docs.docker.com/engine/install/) ([WSL notes](installation.md#docker-setup-for-wsl))
* [FlatBuffers v24.3.25](https://github.com/google/flatbuffers/tree/v24.3.25) ([see notes](installation.md#notes))
* [Anchor CLI](https://solana.com/docs/intro/installation#install-anchor-cli) (optional, if you want to write your Solana programs in Anchor)

## Installation

Build with Bonsol by installing the following components:

- RISC Zero zkVM – Write secure off-chain logic.
- Bonsol CLI – Initialize, build, and deploy your off-chain programs.

You can install these tools using the provided install script or opt for manual installation.

### Install Script

```bash
# Install Bonsol CLI and Risc0 toolchain
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/bonsol-collective/bonsol/refs/heads/main/bin/install.sh | sh
```

Make sure the script completed without errors. Otherwise use the manual install method below.

### Manual Install

Install the RISC Zero version management library and CLI using `rzup`. Bonsol currently supports version 2.0.0.

```bash
curl -L https://risczero.com/install | bash
rzup install cargo-risczero 2.0.0
```

Then install the Bonsol CLI depending on your architecture:

<details>

<summary>Linux</summary>

Install the Bonsol CLI on Linux **without** CUDA support:

```bash
echo "Installing without cuda support, proving will be slower"
cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --locked
```

</details>

<details>

<summary>Linux + CUDA</summary>

Install the Bonsol CLI on Linux **with** CUDA support:

```bash
echo "Installing with cuda support"
cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features linux --locked
```

</details>

<details>

<summary>macOS</summary>

Install the Bonsol CLI on macOS:

```bash
echo "Installing on mac"
cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features mac --locked
```

</details>

### Verify Installation

Verify the installation by running:

```bash
bonsol --help
```

You will see the following:

```bash
Usage: bonsol [OPTIONS] <COMMAND>

Commands:
  deploy    Deploy a program with various storage options, such as S3, or manually with a URL
  build     Build a ZK program
  estimate  Estimate the execution cost of a ZK RISC0 program
  execute
  prove
  init      Initialize a new project
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>    The path to a Solana CLI config [Default: '~/.config/solana/cli/config.yml']
  -k, --keypair <KEYPAIR>  The path to a Solana keypair file [Default: '~/.config/solana/id.json']
  -u, --rpc-url <RPC_URL>  The Solana cluster the Solana CLI will make requests to
  -h, --help               Print help
  -V, --version            Print version
```

See [here for documentation](../cli-commands.md) on these Bonsol CLI commands.

## Uninstall

If you want to uninstall Bonsol, simply run:

```bash
cargo uninstall bonsol-cli
```

Verify the uninstall using:

```bash
bonsol --version
```

You will see the following:

```bash
zsh: command not found: bonsol
```

## Notes

### Docker Setup for WSL

For Bonsol development in WSL, we strongly recommend installing Docker directly in your WSL environment rather than using Docker Desktop for Windows.

<details>

<summary>Docker in WSL</summary>

#### 1. Install Prerequisites

```
sudo apt-get update
sudo apt-get install ca-certificates curl gnupg
```

#### 2. Add Docker's Official GPG Key

```
# Create directory for keyrings
sudo install -m 0755 -d /etc/apt/keyrings

# Download and add GPG key
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

# Set permissions
sudo chmod a+r /etc/apt/keyrings/docker.gpg
```

#### 3. Add Docker Repository

```
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

#### 4. Install Docker

```
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

#### 5. Configure User Permissions

```
# Add your user to docker group
sudo usermod -aG docker $USER

# Apply changes to current session
newgrp docker
```

#### 6. Verify Installation

```
docker --version
docker compose version
```

### Notes

* Docker daemon starts automatically with WSL
* No Docker Desktop required
* GUI features available through Docker Desktop later if needed
* Compatible with all Bonsol development requirements

### Troubleshooting

If you encounter permission issues after installation:

1. Ensure you've logged out and back in after adding your user to the docker group
2. Or run `newgrp docker` to apply changes in current session

If Docker daemon isn't starting:

```
sudo service docker start
```

</details>

### FlatBuffers v24.3.25

FlatBuffers is a cross-platform serialization library. Build it from source on Linux or macOS.

<details>

<summary>Linux</summary>

Ensure you have the build requirements.

```bash
# Update package lists
sudo apt update

# Install CMake
sudo apt install -y cmake

# Verify CMake installation
cmake --version   # Should show version 3.28.3 or later

# Install make
sudo apt install -y g++ make

# Verify make installation
make --version   # Should show version 3.81 or later
```

Build and install FlatBuffers v24.3.25:

```bash
# Create a temporary directory for building
cd /tmp

# Clone the FlatBuffers repository
git clone https://github.com/google/flatbuffers.git

# Enter the repository directory
cd flatbuffers

# Checkout the specific version
git checkout v24.3.25

# Build FlatBuffers
cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)

# Install flatc compiler
sudo mv flatc /usr/local/bin/

# Clean up
cd ..
rm -rf flatbuffers

# Verify the installation
flatc --version   # Should show version 24.3.25
```

</details>

<details>

<summary>macOS</summary>

Ensure you have the build requirements.

```bash
# Update package lists (macOS uses Homebrew instead of apt)
brew update

# Install CMake
brew install cmake

# Verify CMake installation
cmake --version   # Should show version 3.28.3 or later

# Install make (and g++ if needed)
brew install make gcc

# Verify make installation
make --version   # Should show version 3.81 or later
```

Build and install FlatBuffers v24.3.25:

```bash
# Create a temporary directory for building
cd /tmp

# Clone the FlatBuffers repository
git clone https://github.com/google/flatbuffers.git

# Enter the repository directory
cd flatbuffers

# Checkout the specific version
git checkout v24.3.25

# Build FlatBuffers
cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
make

# Install flatc compiler
sudo mv flatc /usr/local/bin/

# Clean up
cd ..
rm -rf flatbuffers

# Verify the installation
flatc --version   # Should show version 24.3.25
```

</details>
