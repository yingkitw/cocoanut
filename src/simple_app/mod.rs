//! Simple high-level API for creating macOS apps with minimal boilerplate
//!
//! This module provides a simplified interface for building macOS GUI applications
//! without needing to deal with low-level Objective-C details.
//!
//! # Modules
//!
//! - `component` - Component types and configuration
//! - `layout` - Layout configuration and management
//! - `app` - SimpleApp builder and event loop

pub mod component;
pub mod layout;
pub mod app;

pub use component::{Kind, Comp};
pub use layout::Layout;
pub use app::SimpleApp;

/// Quick builder for creating a minimal app in one line
pub fn app(name: &str) -> SimpleApp {
    SimpleApp::new(name)
}
