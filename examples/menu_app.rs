//! Menu application example for Cocoanut
//! 
//! This example demonstrates how to create a macOS application with menus using SimpleApp API.
//!
//! Run with: cargo run --example menu_app
//!
//! The window will stay open until you press Cmd+Q to quit.

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Menu Application Example\n");
    println!("Creating real macOS GUI window with menus...\n");
    
    // Create a window with menus using SimpleApp
    app("Menu App")
        .title("🥥 Cocoanut - Menu Application Demo")
        .size(600.0, 400.0)
        .centered(true)
        .run()?;

    println!("\n✅ Application closed");
    Ok(())
}
