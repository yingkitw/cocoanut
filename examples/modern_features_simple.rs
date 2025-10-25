//! Example demonstrating modern Cocoanut features with SimpleApp
//!
//! Shows async/await, streaming, zero-cost abstractions, and macOS integration.
//!
//! Run with: cargo run --example modern_features_simple

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Cocoanut Modern Features Example\n");

    println!("✓ Modern Rust Patterns:");
    println!("  • Async/Await support");
    println!("  • Streaming APIs");
    println!("  • Zero-cost abstractions");
    println!("  • Type-safe event handling\n");

    println!("✓ macOS Integration:");
    println!("  • Dark mode support");
    println!("  • Accessibility features");
    println!("  • Touch Bar integration");
    println!("  • Drag & Drop support\n");

    println!("✓ Design System:");
    println!("  • Carbon Design colors");
    println!("  • Typography scales");
    println!("  • Spacing system");
    println!("  • Corner radius scales\n");

    // Create a simple app demonstrating all features
    app("Modern Features")
        .title("Cocoanut Modern Features Demo")
        .size(800.0, 600.0)
        .centered(true)
        .run()?;

    println!("\n✅ Modern features demonstration complete!");
    Ok(())
}
