---
description: >-
  A practical guide to formatting inputs for Bonsol ZK programs, covering
  common pitfalls and working solutions for different input scenarios.
icon: code
---

# Input Format Guide

## Overview

When creating execution requests for Bonsol ZK programs, proper input formatting is crucial for successful execution. This guide covers the practical aspects of formatting inputs that work with your ZK program's expectations.

## Understanding Input Format Mismatch

One of the most common issues developers face is input format mismatch between:
- The manifest file's `inputOrder` specification
- The execution request JSON format
- What the ZK program actually expects to read

### Common Error: InputError (0x3)

If you encounter `custom program error: 0x3` during execution, this indicates an `InputError`. Common causes include:

- **Wrong byte count**: ZK program expects 8-byte arrays but receives different sizes
- **Wrong input count**: Manifest specifies 1 input but you're sending multiple inputs
- **String vs. byte mismatch**: Sending string data when ZK program expects binary data

## Working Input Formats

### Single Combined Byte Input

**Best Practice**: When your ZK program reads multiple values using `env::read_slice()`, combine all values into a single input.

**Example**: Calculator program that reads 3 i64 values

```rust
// ZK program code
fn read_i64_input() -> i64 {
    let mut input_bytes = [0u8; 8];
    env::read_slice(&mut input_bytes);
    i64::from_le_bytes(input_bytes)
}

fn main() {
    let operation = read_i64_input();  // First 8 bytes
    let operand_a = read_i64_input();  // Next 8 bytes
    let operand_b = read_i64_input();  // Next 8 bytes
    // ... rest of program
}
```

**Corresponding Rust client code**:

```rust
// Create individual i64 values as little-endian bytes
let operation_bytes = 2i64.to_le_bytes();  // multiply operation
let operand_a_bytes = 7i64.to_le_bytes();  // first operand
let operand_b_bytes = 6i64.to_le_bytes();  // second operand

// Combine into single 24-byte input
let mut combined_input = Vec::with_capacity(24);
combined_input.extend_from_slice(&operation_bytes);
combined_input.extend_from_slice(&operand_a_bytes);
combined_input.extend_from_slice(&operand_b_bytes);

// Send as single input
let execution_instruction = execute_v1(
    &requester,
    &payer.pubkey(),
    image_id,
    execution_id,
    vec![
        InputRef::public(&combined_input),  // Single 24-byte input
    ],
    // ... other parameters
)?;
```

**Manifest alignment**:
```json
{
  "inputOrder": ["Public"]  // Single input, not multiple
}
```

### String Inputs (Limited Support)

**Caution**: String inputs may work for transaction submission but fail during ZK execution.

**What doesn't work**:
```rust
// This will fail during ZK execution
vec![
    InputRef::public("2".as_bytes()),    // [50] - only 1 byte
    InputRef::public("7".as_bytes()),    // [55] - only 1 byte  
    InputRef::public("6".as_bytes()),    // [54] - only 1 byte
]
```

**Why it fails**: The ZK program expects 8-byte arrays, but strings produce variable-length byte arrays.

## Debugging Input Issues

### Enable Debug Output

Add debug printing to your client to understand exactly what's being sent:

```rust
println!("📥 Input being sent:");
println!("   Data: {:?} (length: {})", &input_data, input_data.len());
println!("   Expected by ZK program: {} calls to env::read_slice() with {}-byte arrays", 
         num_reads, bytes_per_read);
```

### Check Transaction vs. Execution

- **Transaction success + execution failure**: Input format accepted by Bonsol but incompatible with ZK program
- **Transaction failure**: Input format rejected by Bonsol interface

### Monitor Bonsol Logs

Watch for these error patterns in the prover logs:
```
"0 inputs resolved"
"DeserializeUnexpectedEnd"
"Guest panicked"
```

## Best Practices

### 1. Align with ZK Program Expectations

**Do**: Format inputs to match exactly what your ZK program reads
```rust
// If ZK program does this:
let mut buffer = [0u8; 8];
env::read_slice(&mut buffer);

// Then send this:
let data = 42i64.to_le_bytes();  // Exactly 8 bytes
InputRef::public(&data)
```

### 2. Use Single Combined Inputs

**Do**: When reading multiple values sequentially, combine them into one input
```rust
// Multiple sequential reads = single combined input
let combined = [data1, data2, data3].concat();
vec![InputRef::public(&combined)]
```

**Don't**: Send separate inputs when ZK program expects sequential reads from one input
```rust
// This may not work as expected
vec![
    InputRef::public(&data1),
    InputRef::public(&data2), 
    InputRef::public(&data3),
]
```

### 3. Match Manifest Input Count

Ensure your `inputOrder` in the manifest matches your actual input usage:

```json
{
  "inputOrder": ["Public"]  // One input
}
```

```rust
vec![InputRef::public(&single_combined_input)]  // One input
```

### 4. Test with Known Working Formats

Start with the byte format that matches your ZK program's expectations, then experiment with convenience formats.

## Example: Working Calculator Client

This example shows a complete working implementation:

```rust
// Client code that works
let operation_bytes = 2i64.to_le_bytes();     // [2,0,0,0,0,0,0,0]
let operand_a_bytes = 7i64.to_le_bytes();     // [7,0,0,0,0,0,0,0]  
let operand_b_bytes = 6i64.to_le_bytes();     // [6,0,0,0,0,0,0,0]

let mut combined_input = Vec::with_capacity(24);
combined_input.extend_from_slice(&operation_bytes);
combined_input.extend_from_slice(&operand_a_bytes);
combined_input.extend_from_slice(&operand_b_bytes);

let execution_instruction = execute_v1(
    &requester,
    &payer.pubkey(),
    "5881e972d41fe651c2989c65699528da8b1ed68ab7057350a686b8a64a00fc91",
    "calc_exec_1",
    vec![InputRef::public(&combined_input)],
    1000,
    expiration,
    ExecutionConfig {
        verify_input_hash: false,
        input_hash: None,
        forward_output: true,
    },
    callback_config,
    None,
)?;
```

Result: ✅ Transaction succeeds + ZK execution succeeds

## Troubleshooting Checklist

- [ ] Input byte count matches ZK program's `env::read_slice()` expectations
- [ ] Number of inputs matches manifest's `inputOrder` length  
- [ ] Using little-endian byte order for numeric values
- [ ] Combined sequential reads into single input
- [ ] Tested with raw bytes before trying convenience formats
- [ ] Checked Bonsol logs for execution errors
- [ ] Verified ZK program logic handles input format correctly

## Further Reading

- [Bonsol Input Types](../explanation/bonsol-input-types.md) - Overview of available input types
- [Tutorial: Simple Program](tutorial-simple-program.md) - Complete example with JSON inputs
- [CLI Commands](../cli-commands.md) - Bonsol CLI reference 
