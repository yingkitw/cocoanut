//! Configurable Components Example - Shows how to customize which components appear
//!
//! Demonstrates the configurable SimpleApp API with different component combinations:
//! - Empty window (no components)
//! - Single component
//! - Multiple components
//! - Custom component ordering
//!
//! Run with: cargo run --example configurable_components

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Configurable Components Example\n");

    // Example 1: Empty window (no components)
    println!("Example 1: Empty Window");
    println!("Creating app with no components...\n");
    
    app("Empty App")
        .title("🥥 Cocoanut - Empty Window")
        .size(600.0, 400.0)
        .centered(true)
        .run()?;

    Ok(())
}
