//! Library entry points for `sdlint`.
//!
//! This crate deliberately leaves process termination to the binary boundary.

pub mod app;
pub mod cli;
pub mod input;

pub use app::{run, RunOutcome};
