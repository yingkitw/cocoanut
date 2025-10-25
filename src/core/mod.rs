//! Core foundation modules
//!
//! This module contains fundamental types and traits that form the foundation
//! of the Cocoanut framework.

pub mod error;
pub mod traits;
pub mod utils;

pub use error::{CocoanutError, Result};
pub use traits::*;
pub use utils::*;
