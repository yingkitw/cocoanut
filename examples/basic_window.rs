//! Basic window example for Cocoanut
//! 
//! This example demonstrates how to create a simple macOS window using Cocoanut's SimpleApp API.
//!
//! Run with: cargo run --example basic_window
//!
//! The window will stay open until you press Cmd+Q to quit.

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Basic Window Example\n");
    println!("Creating a simple macOS window...\n");
    
    // Create and run a simple app with one line!
    app("Basic Window")
        .title("🥥 Hello, Cocoanut!")
        .size(800.0, 600.0)
        .centered(true)
        .run()?;
    
    println!("\n✅ Application closed");
    Ok(())
}
