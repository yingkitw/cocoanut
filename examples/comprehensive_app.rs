//! Comprehensive Cocoanut Application Example - Real GUI
//!
//! This example creates a real macOS GUI window with components.
//! Run with: cargo run --example comprehensive_app
//!
//! The window will stay open until you press Cmd+Q to quit.

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Comprehensive Cocoanut Application Example\n");
    println!("Creating real macOS GUI window...\n");

    // Create a window using SimpleApp
    app("Comprehensive Demo")
        .title("🥥 Cocoanut - Comprehensive Component Demo")
        .size(1000.0, 800.0)
        .centered(true)
        .run()?;

    println!("\n✅ Application closed");
    Ok(())
}
