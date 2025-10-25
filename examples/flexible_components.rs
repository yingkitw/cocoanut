//! Flexible Components Example - Shows configurable component properties
//!
//! Demonstrates the new ComponentConfig API with customizable:
//! - Component text/title
//! - Component width and height
//! - Multiple components with different configurations
//!
//! Run with: cargo run --example flexible_components

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Flexible Components Example\n");

    println!("Creating window with custom-configured components...");
    println!("✓ Window created (700x500, centered)\n");

    println!("🚀 Launching GUI window...\n");
    println!("Press Cmd+Q to quit\n");
    
    // Create app with custom-configured components (using spacious layout)
    app("Flexible Components")
        .title("🥥 Cocoanut - Flexible Components Example")
        .size(700.0, 500.0)
        .centered(true)
        .layout(Layout::spacious())
        // Button with custom text and size
        .add(
            Comp::new(Kind::Button)
                .text("Save Document")
                .size(150.0, 40.0)
        )
        // Label with custom text
        .add(
            Comp::new(Kind::Label)
                .text("Enter your name:")
                .size(300.0, 25.0)
        )
        // TextField with custom placeholder
        .add(
            Comp::new(Kind::TextField)
                .text("John Doe")
                .size(300.0, 30.0)
        )
        // Another button with different text
        .add(
            Comp::new(Kind::Button)
                .text("Cancel")
                .size(150.0, 40.0)
        )
        .run()?;

    println!("\n✅ Application closed");

    Ok(())
}
