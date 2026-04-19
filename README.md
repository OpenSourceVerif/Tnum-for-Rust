# Tnum-for-Rust

A `no_std`-compatible **Tnum** (truncated number) abstract domain implementation in pure Rust, designed for static analysis of eBPF programs (Solana SBF).

## Overview

Tnum is a compact abstract domain that represents a set of possible 64-bit unsigned integer values using two bitmasks:

- `**value`**: bits known to be constant
- `**mask**`: unknown bits (`1` = unknown, `0` = known)

This representation enables efficient interval tracking with sub-interval precision, making it ideal for value-range analysis in eBPF interpreters and compilers.

## Features

- `no_std` compatible (no standard library dependency)
- `#[cfg(feature = "std")]` for opt-in standard library features (e.g., debug printing)
- Full arithmetic operations: add, sub, mul, div, rem
- Bitwise operations: and, or, xor, not, shl, lshr, ashr
- Byte swap: bswap16, bswap32, bswap64
- Type-safe abstract domain with join, intersect, and subset operations
- LTO-optimized release profile

```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tnum4rust = { git = "https://github.com/ctc-team/tnum4rust" }
```

Or from [crates.io](https://crates.io) once published:

```toml
[dependencies]
tnum4rust = "0.1"
```

## Usage

### Basic Operations

```rust
use tnum::Tnum;

// Creation
let t = Tnum::const_val(100);
let t = Tnum::top();        // completely unknown
let t = Tnum::bottom();     // impossible value
let t = Tnum::new(v, m);    // custom value/mask

// Arithmetic
let sum    = a.add(b);
let diff   = a.sub(b);
let prod   = a.mul(b);
let quot   = a.udiv(b);
let rem    = a.urem(b);

// Bitwise
let ored   = a.or(&b);
let anded  = a.and(&b);
let xored  = a.xor(b);
let shifted = a.shl_const(3);

// Domain operations
let joined = a.join(b);        // least upper bound
let inter  = a.intersect(b);  // greatest lower bound
```

### Integration with Solana SBF

This crate powers the value analysis in the Solana eBPF verifier and interpreter. The `Tnum` domain tracks register values with efficient bit-level precision, catching undefined behavior and narrowing value ranges across instruction traces.

