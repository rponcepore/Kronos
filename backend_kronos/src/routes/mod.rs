//! handlers.mod.rs

// This file points to our various route handlers.

pub mod healthchecks;
pub mod order_edits;
pub mod api;

// I'm not sure why these are necessary:
pub use healthchecks::*;
// pub use order_edits::*;