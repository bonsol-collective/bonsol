---
description: Building a Calculator DApp with Bonsol CLI
icon: calculator
---

# Bonsol Calculator Example

A zero-knowledge calculator web application using the Bonsol ZK network for verifiable computations. This guide walks you through building a simple calculator dApp using the [Bonsol CLI and local development environment.](https://bonsol.gitbook.io/docs/getting-started/installation)

Here’s a link to the repo:

> [https://github.com/en-tropyc/bonsol-calculator](https://github.com/en-tropyc/bonsol-calculator)

### **General flow**

The general flow of the process that we’ll be covering can be visualized below:

```markdown

React Frontend ──→ Express API ──→ Bonsol Network ──→ ZK Computation
      ↓               ↓               ↓               ↓
  localhost:3000  localhost:3001   ZK Execution    Verified Result
```

The proving system we use is the Groth16 SNARK that enables fast on-chain verification of proofs.

### **Prerequisites**

Make sure you have the following installed:

* Bonsol CLI - follow the steps in the [installation guide](https://bonsol.gitbook.io/docs/getting-started/installation) to get started
* [**Solana CLI**](https://solana.com/docs/intro/installation#install-rust)
* **Node.js** (v18+ recommended)
* [pnpm](https://pnpm.io/installation)

### Step 1: Set Up a Project

Firstly, we need to verify your installation

```solidity
bonsol --version
```

The current version is Version **0.4.5**

1. **Create a New Project Directory**:

```solidity
mkdir bonsol-calculator
cd bonsol-calculator
```

1. Clone the repository

```rust
git clone <https://github.com/en-tropyc/bonsol-calculator.git>
```

We should see the following project structure:

```rust
bonsol_calculator/
├── bonsol
├── calculator-api
├── client
├── frontend
├── local-server
├── solana-program
├── zk-program
├── Cargo.toml
├── README.md
```

### Step 2: Setting up the Local environment

The documentation [here](https://bonsol.gitbook.io/docs/developers/setup-a-local-environment) provides instructions for setting up a local Bonsol development environment for the calculator example.

\<aside> \<img src="/icons/activity\_pink.svg" alt="/icons/activity\_pink.svg" width="40px" />

At present, provers can only run on **x86\_64-linux** systems due to dependencies in the STARK-to-SNARK tooling. We’re actively exploring macOS support, but in the meantime, we recommend using a **remote Linux environment** for development.

\</aside>

In this step, you will need to

* **Start the Local Validator**

The validator script builds and deploys the necessary Solana programs, including the Bonsol core program and an example callback program

```rust
./bin/validator.sh -r
```

If the validator fails to start, ensure that Rust and Solana CLI tools are properly installed

* **Run the Bonsol Prover Node**

The prover node processes the off-chain computation. Open a new terminal and run:

```rust
$ ./bin/run-node.sh
```

* **Run the Local ZK Program Server**

Provers on the network need to fetch the ZK programs and the input data used to generate the proof.

Open a new terminal and run:

```rust
$ cargo run -p local-zk-program-server
```

### Step 3: The ZK Program

In `zk-program/src/main.rs`Let's go through the code snippet:

```rust

// Calculator ZK program (from zk-program/manifest.json)
const CALCULATOR_IMAGE_ID: &str = "5881e972d41fe651c2989c65699528da8b1ed68ab7057350a686b8a64a00fc91";
const CALLBACK_PROGRAM_ID: &str = "2zBRw2sEXvjskx7w1w9hqdFEMZWy7KipQ6jKPfwjpnL6";

// Calculator operation codes (from zk-program/src/main.rs)
const OP_ADD: i64 = 0;
const OP_SUBTRACT: i64 = 1;
const OP_MULTIPLY: i64 = 2;
const OP_DIVIDE: i64 = 3;
```

**Constants -** The program defines four operation codes as u8 values:

* OP\_ADD (0): Addition.
* OP\_SUBTRACT (1): Subtraction.
* OP\_MULTIPLY (2): Multiplication.
* OP\_DIVIDE (3): Division.

These constants serve as identifiers for the different arithmetic operations the program is designed to handle. By mapping each operation to a distinct `u8` value, the program can efficiently reference and execute these operations based on their numeric codes.

### Input Format

The calculator ZK program expects three inputs as i64 little-endian bytes:

1. **Operation Code** (8 bytes): 0=add, 1=subtract, 2=multiply, 3=divide
2. **Operand A** (8 bytes): First number
3. **Operand B** (8 bytes): Second number

For example, to calculate `5 + 3`:

* Operation: `0` (add) → `[0, 0, 0, 0, 0, 0, 0, 0]`
* Operand A: `5` → `[5, 0, 0, 0, 0, 0, 0, 0]`
* Operand B: `3` → `[3, 0, 0, 0, 0, 0, 0, 0]`

Now, let's build the zk Program

```rust
bonsol build --zk-program-path ./calculator-example/zk-program
```

### Step 4: The Solana Program

Let's examine the simple ZK program provided in the repo at `solana-program/src/lib.rs` We have a program that uses Bonsol to verify an off-chain addition:

#### Program Structure and Initialization

* This creates a calculator that uses the Bonsol prover network for computations. It imports key `solana_program` modules such as `AccountInfo`, `Pubkey`, and `ProgramError` for handling accounts and errors, and `borsh` for serializing data structures such as `CalculatorState` (tracking program state) and `CalculationRecord` (storing calculation details).
* The `CalculatorInstruction` enum defines four instructions: `Initialize`, `SubmitCalculation`, `GetHistory`, and `Callback`.

#### ZK Calculation and Handling

* The `submit_calculation` function prepares a calculation for the Bonsol ZK network by validating the payer, operation (add, subtract, multiply, divide), and owner. It serializes the operation and operands into a 24-byte input for the ZK program and creates a Bonsol instruction with `execute_v1,` including a callback configuration and 100-slot expiration.
* A pending `CalculationRecord` is stored in `CalculatorState`, which is updated and serialized. The `get_history` function logs the calculation count and last calculation details, while the callback function updates the `CalculationRecord` with the ZK result.

**Build the Code**

Run anchor `build` to make sure everything builds correctly:

```bash
cd solana-program
anchor build
anchor deploy
```

### Step 5: Set Up the Bonsol Calculator Client

```bash
cargo run --bin prover
```

Let's examine the Bonsol calculator client program provided in the repo at `zk-program/src/main.rs`

The Rust program is a client for submitting calculator execution requests to the Bonsol platform on the Solana blockchain, using the bonsol\_interface crate to create execution instructions.

**Key Components**

* Combines operation code and operands into a 24-byte input for the ZK calculator program.
* Configures `ExecutionConfig` (disables input hash, enables output forwarding) and CallbackConfig (specifies callback program and extra accounts).
* Uses `execute_v1` to create a Bonsol execution instruction with the image ID, execution ID, inputs, tip, and expiration.
* Sends the instruction as a signed transaction using `solana_client.`

```
cd client
cargo build
```

### Step 6: Start the Backend API Server

Firstly, we need to start up the Node.js Server

```tsx
cd calculator-api
npm install
npm start
```

Node.js Express server acts as a REST API wrapper for a Rust-based Bonsol calculator client, enabling users to submit arithmetic operations (add, subtract, multiply, divide) to the Bonsol platform on the Solana blockchain.

#### **Endpoints**

The core endpoint, `POST /calculate`, validates input (operation and operands), generates or uses a provided execution ID, and runs the Rust client with cargo run to submit the calculation to Bonsol via the direct-bonsol method, storing the request status and transaction signature in a Map. Additional endpoints (`GET /execution/:id`, `/executions`, `/health`, /) provide execution status, list all requests, check server health, and display API info.

If the command was successful, you should see the code below

<figure><img src="../.gitbook/assets/Screenshot 2025-07-28 at 5.46.24 PM.png" alt=""><figcaption></figcaption></figure>

### Step 7: Build the Frontend

In this step, we will interact with the calculator with a frontend using the `CalculationRequest` and `CalculationResponse` that reveals `ExecutionStatus` as 'submitted', 'completed', or 'failed' in the file in `frontend/src/bonsol-api-client.ts`.

It defines a `BonsolApiClient` class that serves as a client for interacting with the Bonsol Calculator REST API, facilitating arithmetic operation submissions to the Bonsol platform on the Solana blockchain.

It provides methods to:

* Submit calculations via POST /calculate,
* Retrieve execution status with GET /execution/:id,
* List all executions using GET /executions
* Check API health with GET /health

Lastly, run the frontend client

```tsx
cd frontend  
npm install
npm start
```

* Open `http://localhost:3000`

![](<../.gitbook/assets/Screenshot 2025-07-28 at 5.47.32 PM.png>)

### Step 8: Test Application

Run the computation:

* Enter numbers and select operation (e.g 25 \* 15)
* Click "Calculate with ZK"
* Wait \~15-30 seconds for ZK proof computation\


What it does - submit calculations through a web interface that:

1. Sends requests to local API server
2. Submits to Bonsol ZK network for computation
3. Returns cryptographically verified results
4. Displays execution IDs and transaction signatures\


Example successful execution:

* Calculation: 15 × 25 = 375
* Execution ID: `calc_1748059174997_35200c8e`
* Transaction: `5yTzwjn88HTWTPnciBoqpXnj7ouuYZJsRFN8n2GPM9YmjuctidvJkhywepj11dxuXjKvTRC48PXBetL6ERtDb5mF`

### Note

Here are some ideas if you are interested in extending the capabilities of this client example

1. Add support for more complex mathematical operations
2. Implement batch calculations
3. Add input validation and error handling
4. Support for floating-point operations (requires ZK program changes)
5. Add result verification and display
