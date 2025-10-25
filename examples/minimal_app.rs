//! Minimal Cocoanut Application Example - With GUI Components
//!
//! Demonstrates the simplified Cocoanut API with:
//! - SimpleApp builder pattern
//! - Window configuration
//! - GUI components displayed in window
//!
//! Run with: cargo run --example minimal_app

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Minimal Cocoanut Application Example\n");

    println!("Creating window...");
    let window = Window::builder()
        .title("Minimal App - Cocoanut")
        .size(600.0, 400.0)
        .center()
        .build()?;
    
    println!("✓ Window created (600x400, centered)\n");

    println!("🚀 Launching GUI window...\n");
    println!("Press Cmd+Q to quit\n");
    
    // Create app and run - components will be added during app initialization
    app("Minimal App")
        .with_window(window)
        .run()?;

    println!("\n✅ Application closed");

    Ok(())
}
