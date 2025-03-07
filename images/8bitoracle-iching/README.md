# I Ching ZK Program

A zero-knowledge proof program for generating I Ching hexagrams using the traditional Yarrow Stalk method.

## Overview

This zkprogram implements the traditional Yarrow Stalk divination method for generating I Ching hexagrams. It provides cryptographic proof that the divination process was performed correctly while keeping the intermediate calculations private.

## Features

- Generates complete 6-line hexagrams
- Uses traditional 50-stalk division method
- Provides zero-knowledge proofs of correct calculation
- Maintains privacy of intermediate steps
- Deterministic generation from random seed

## Input

- 32-byte random seed (private input)

## Output

1. Hash of the random seed
2. Final hexagram values (6 numbers between 6-9)
3. Proof of valid calculation

## Building

```bash
bonsol build --zk-program-path ./images/iching
```

## Usage

The program takes a random seed as input and produces a verifiable hexagram. Each line value corresponds to:
- 6: Old Yin (changing)
- 7: Young Yang (stable)
- 8: Young Yin (stable)
- 9: Old Yang (changing)

## Implementation Details

The program follows the traditional Yarrow Stalk method:
1. Start with 49 stalks (50 minus 1 for initial division)
2. Perform 3 rounds of division for each line
3. Calculate remainders and map to line values
4. Generate proof of correct calculation

## Testing

Run the tests with:
```bash
cargo test
``` 