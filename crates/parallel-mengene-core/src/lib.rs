//! Core compression algorithms and data structures for parallel-mengene
//!
//! This crate provides the fundamental compression algorithms, data structures,
//! and utilities that power the parallel-mengene compression tool.

pub mod algorithm_selector;
pub mod algorithms;
pub mod binary_pma_format;
pub mod compression;
pub mod error;

pub mod utils;

pub use error::{Error, Result};
