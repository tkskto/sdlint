//! Library entry points for `sdlint`.
//!
//! This crate deliberately leaves process termination to the binary boundary.

pub mod app;
pub mod cli;
pub mod diagnostic;
pub mod input;
pub mod lint;
pub mod parse;
pub mod report;

pub use app::{RunOutcome, run};
