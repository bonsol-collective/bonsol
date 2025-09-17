# Manually Provision a Bonsol Node

Bonsol has a fully featured Docker image and Helm chart that can be used to run a Bonsol node on Kubernetes. For more information on how to run a Bonsol node on kubernetes check out the [Run a Bonsol Node on Kubernetes](https://bonsol.sh/docs/how-to-guides/run-a-bonsol-node-on-k8s) guide.

### Prerequisites <a href="#prerequisites" id="prerequisites"></a>

* A keypair for the node, you need some SOL to pay for the transactions
* A Dragons mouth compatible rpc provider endpoint [Dragons Mouth Docs](https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions) click here to get one from [Triton One](https://triton.one/triton-rpc/)
* Docker on your local machine (not required on the node)
* The node will do better if it has a gpu with cuda installed, which will require nvidia drivers and tools.

> **Note**: Ansible role coming soon

### Hardware Requirements

To run a Bonsol prover node effectively, you'll need:

**CPU**:

* Minimum: 4 cores / 8 threads
* Recommended: 8 cores / 16 threads for better proof generation performance
* Architecture: x86\_64

**Memory**:

* Minimum: 16 GB RAM
* Recommended: 32 GB RAM

**Storage**:

* Minimum: 100 GB SSD available space
* Recommended: 250 GB+ SSD for image caching

**GPU** (Optional but recommended):

* Minimum: GTX 1060 6GB or equivalent
* Recommended: RTX 3060 or better
* Required: CUDA 11.0+

**Network**:

* Stable internet connection with at least 100 Mbps bandwidth
* Low latency connection to your RPC provider

> **Note**: While a GPU is optional, nodes with CUDA-capable GPUs will have significantly better proof generation performance and may be more competitive in the network.

### Installing Deps <a href="#installing-deps" id="installing-deps"></a>

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.81.0 -y
```

Ensure cargo is on the path

On your local machine, you will need to run a Docker image to get the needed Groth16 witness generator and snark binary.On your local machine, you will need to run a Docker image to get the needed Groth16 witness generator and snark binary. This script will download them from the internet and save them in the current directory use `--prefix` to change the output directory

```
./bin/setup.sh
```

You will have a director called snark with the binaries in it. You need to copy these binaries to the node and remember the path to the snark directory.

```
# on the node
sudo mkdir -p /opt/bonsol/stark
sudo chown -R ubuntu /opt/bonsol/stark
sudo mkdir -p /opt/bonsol/keys
sudo chown -R ubuntu /opt/bonsol/keys
```

```
# on your local computer

scp -i <your_ssh_key> -r stark/* <node_user>@<node ip>:/opt/bonsol/stark
```

You will put the path in the `stark_compression_tools_path` in the config file.

### Upload the keypair to the node <a href="#upload-the-keypair-to-the-node" id="upload-the-keypair-to-the-node"></a>

You will need to upload the keypair to the node.

```
scp -r <keypair path> <node ip>:/opt/bonsol/keys/
```

You will put the path in the section below of the config file.you will put the path in the below section of the config file.

```
[signer_config]
  KeypairFile = { path = "<your keypair path>" }
```

### Installing Bonsol <a href="#installing-bonsol" id="installing-bonsol"></a>

```
git clone --depth=1 https://github.com/anagrambuild/bonsol.git bonsol
cd bonsol/
cargo build -f cuda --release
```

### Configuring the Node <a href="#configuring-the-node" id="configuring-the-node"></a>

You will need to create a config file for the node. The config file is a `toml` file that contains the configuration for the node.

```
touch Node.toml
```

Here is an example of a config file.

```
risc0_image_folder = "/opt/bonsol/risc0_images"
max_input_size_mb = 10
image_download_timeout_secs = 60
input_download_timeout_secs = 60
maximum_concurrent_proofs = 1
max_image_size_mb = 4
image_compression_ttl_hours = 24
env = "dev"
stark_compression_tools_path = "<the path to the stark directory>"
missing_image_strategy = "DownloadAndClaim"
[metrics_config]
  Prometheus = {}
[ingester_config]
RpcBlockSubscription = { wss_rpc_url = "<your websockets endpoint>" }
[transaction_sender_config]
  Rpc = { rpc_url = "<your solana rpc endpoint>" }
[signer_config]
  KeypairFile = { path = "<your keypair path>" }
```

### Running the Node <a href="#running-the-node" id="running-the-node"></a>

After building the relay package, you can run the node with the following command.

```
ulimit -s unlimited //this is required for the c++ groth16 witness generator it will blow your stack without a huge stack size
#from within the bonsol root dir
./target/release/relay -f Node.toml
```

#### Running the Node with systemd <a href="#running-the-node-with-systemd" id="running-the-node-with-systemd"></a>

You can use the following systemd service file to run the node.

```
[Unit]
Description=Bonsol Node
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
User=ubuntu
Restart=always
RestartSec=1
LimitSTACK=infinity
LimitNOFILE=1000000
LogRateLimitIntervalSec=0
WorkingDirectory=/home/ubuntu/bonsol
ExecStart=/home/ubuntu/bonsol/target/release/bonsol-node -f Node.toml

# Create BACKTRACE only on panics
Environment="RUST_BACKTRACE=1"
Environment="RUST_LIB_BACKTRACE=0"

[Install]
WantedBy=multi-user.target
```

You will need to copy this file `/etc/systemd/system/bonsol.service` and then run the following command. After that, you can reload the systemd daemon and start the service with the following command.

```
systemctl daemon-reload
systemctl start bonsol
```

Installing Alloy is out of the scope of this guide, but you can follow the [Grafana Cloud docs](https://grafana.com/docs/alloy/latest/set-up/install/linux/) to install it.
