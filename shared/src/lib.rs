//! Shared types and utilities for TREZZA TERMINAL
//!
//! This crate contains common data types, constants, and utilities
//! shared between the frontend and backend components.

pub mod types;
pub mod errors;
pub mod constants;

// Re-export commonly used types
pub use types::*;
pub use errors::*;
pub use constants::*;