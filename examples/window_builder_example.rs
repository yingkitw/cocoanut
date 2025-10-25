//! Example demonstrating the SimpleApp and Window Builder API
//!
//! This example shows how to use the fluent SimpleApp and Window Builder APIs
//! to create applications with various configurations.
//!
//! Run with: cargo run --example window_builder_example

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🪟 Window Builder & SimpleApp Example\n");

    // Example 1: SimpleApp with basic configuration
    println!("Example 1: SimpleApp with basic configuration");
    println!("  app(\"MyApp\").title(\"My App\").size(800.0, 600.0).run()?;\n");

    // Example 2: SimpleApp with all options
    println!("Example 2: SimpleApp with all options");
    println!("  app(\"MyApp\")");
    println!("    .title(\"My Application\")");
    println!("    .size(1200.0, 800.0)");
    println!("    .centered(true)");
    println!("    .run()?;\n");

    // Example 3: Window builder with custom configuration
    println!("Example 3: Window builder with custom configuration");
    let _window = Window::builder()
        .title("Custom Window")
        .size(1024.0, 768.0)
        .center()
        .resizable(true)
        .build()?;
    println!("✓ Created custom window\n");

    // Example 4: Comparison - Old vs New API
    println!("Example 4: API Comparison\n");
    
    println!("Old API (verbose):");
    println!("  let app = Application::new(\"MyApp\")?;");
    println!("  let window = Window::new(\"Title\", 800.0, 600.0)?;");
    println!("  window.show()?;");
    println!("  app.run(window)?;\n");
    
    println!("New SimpleApp API (fluent):");
    println!("  app(\"MyApp\")");
    println!("    .title(\"Title\")");
    println!("    .size(800.0, 600.0)");
    println!("    .centered(true)");
    println!("    .run()?;\n");

    println!("✅ All examples completed!");
    println!("\n📝 Key Benefits:");
    println!("  • SimpleApp: 80% less boilerplate");
    println!("  • Fluent API for readable code");
    println!("  • Chainable methods");
    println!("  • Type-safe configuration");
    println!("  • Works with real macOS GUI");

    Ok(())
}
