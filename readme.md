# ShaiForge - Virtual Block Explorer

## Introduction

ShaiForge is a virtual block explorer script written in Rust. It allows users to explore various block heights, view block rewards, total supply, and estimated time based on a simulated blockchain with a decay rate.


Shai (SHA) is a cryptocurrency, a fork of Bitcoin, designed to implement a unique mining algorithm. Shai aims for 5-minute block times and limits block size to 2 MB. The total amount of coins will be approximately 4,925,405 SHA. Tail emission will commence at block 420,480. The initial block reward will be 50 SHA, and with each subsequent block, it will decrease with the lowest possible value being 0.1 SHA/block. The smallest denomination is called a Kismet, where 1 SHA equals 100,000,000 Kismet (KIS).

## Features

- Explore block heights and view corresponding information.
- Calculate block rewards and total supply.
- Estimate time based on block height.

## Installation

1. Ensure you have Rust installed on your system.
2. Clone the ShaiForge repository.
3. Navigate to the project directory.

## Usage
1. Run the script.`cargo run --release`
2. Enter a block height to explore.
3. View the block reward, total supply, and estimated time.
4. To exit, enter 'q'.

# Coin Reward Function

The coin reward function f(x) represents the coin reward at block height x, defined by the equation:

f(x) = 50 * e^{ -0.00001 * x }

Where:
- f(x): Coin reward at block height x
- 50: Initial coin reward at block height 0
- e: Base of the natural logarithm
- -0.00001: Decay rate (b)
- x: Block height

## Block Height 420480

At block height 420480:
- Approximately 4,925,405 SHA will be emitted.
- Tail emission kicks in at 0.1 SHA/Block.


## Example

```bash
Enter a block height to explore (or 'q' to quit):
420480
══════════════════════════════════════════════
Block Height: 420480
Reward: 0.1 SHA
Total Supply: 4925405.87222002 SHA
Estimated Time: 4 years, 0 days, 0 hours, 0 minutes
══════════════════════════════════════════════
```

