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
    println!("✓ Window created (600x400, centered)\n");

    println!("🚀 Launching GUI window...\n");
    println!("Press Cmd+Q to quit\n");
    
    // Create app with components and run
    app("Minimal App")
        .title("🥥 Cocoanut - Minimal Example")
        .size(600.0, 400.0)
        .centered(true)
        .add(ComponentConfig::new(ComponentType::Button))
        .add(ComponentConfig::new(ComponentType::Label))
        .add(ComponentConfig::new(ComponentType::TextField))
        .run()?;

    println!("\n✅ Application closed");

    Ok(())
}
