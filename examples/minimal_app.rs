//! Minimal Cocoanut Application Example - Enhanced with SimpleApp API
//!
//! Demonstrates the simplified Cocoanut API with:
//! - SimpleApp builder pattern
//! - Window configuration
//! - Layout system (VStack, HStack)
//! - Styling (colors, fonts)
//! - Event handlers
//!
//! Run with: cargo run --example minimal_app

use cocoanut::prelude::*;
use std::sync::{Arc, Mutex};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Minimal Cocoanut App - Enhanced with SimpleApp API\n");

    // Create a simple app with fluent API
    app("Minimal App")
        .title("My Cocoanut App")
        .size(600.0, 400.0)
        .centered(true)
        .run()?;

    println!("\n✅ Application closed successfully!");

    Ok(())
}
