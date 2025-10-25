//! Two Buttons Example - Simple app with multiple buttons
//!
//! Demonstrates how to add multiple components of the same type.
//! This example shows 2 buttons stacked vertically in a window.
//!
//! Run with: cargo run --example two_buttons

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Two Buttons Example\n");

    println!("Creating window with 2 buttons...");
    println!("✓ Window created (600x400, centered)\n");

    println!("🚀 Launching GUI window...\n");
    println!("Press Cmd+Q to quit\n");
    
    // Create app with 2 buttons
    app("Two Buttons App")
        .title("🥥 Cocoanut - Two Buttons Example")
        .size(600.0, 400.0)
        .centered(true)
        .with_component(ComponentType::Button)
        .with_component(ComponentType::Button)
        .run()?;

    println!("\n✅ Application closed");

    Ok(())
}
