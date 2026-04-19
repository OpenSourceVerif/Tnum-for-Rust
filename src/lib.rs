//! Tnum (truncated number) abstract domain implementation.
//!
//! This crate provides a tnum domain for abstract interpretation,
//! commonly used in static analysis of eBPF programs (particularly Solana SBF).
//!
//! # Overview
//!
//! A `Tnum` represents a set of possible 64-bit unsigned integer values using
//! a value/mask representation:
//! - `value`: known bits (bits known to be constant)
//! - `mask`: unknown bits (bits that may vary, `1` = unknown, `0` = known)

#![no_std]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::ptr_as_ptr)]

extern crate alloc;

mod tnum;

pub use tnum::{BitOps, Tnum};
