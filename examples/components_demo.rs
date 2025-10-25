//! Comprehensive Cocoanut Components Demo
//!
//! This example demonstrates:
//! - Window creation with proper styling
//! - Button controls with event handling
//! - Label controls with text display
//! - Text field controls with input handling
//! - Layout management
//! - Event loop integration
//!
//! Run with: cargo run --example components_demo

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Cocoanut Components Demo\n");

    // Create the main application window
    println!("Creating main window...");
    let window = Window::builder()
        .title("Cocoanut Components Demo")
        .size(800.0, 600.0)
        .center()
        .build()?;

    println!("✓ Main window created (800x600, centered)\n");

    // Create UI components
    println!("Creating UI components...");

    // Create a label
    let label = Label::builder()
        .text("Welcome to Cocoanut!")
        .size(300.0, 30.0)
        .build()?;

    // Create a button
    let button = Button::builder()
        .title("Click Me!")
        .size(120.0, 40.0)
        .on_click(|| {
            println!("🎯 Button clicked!");
        })
        .build()?;

    // Create a text field
    let text_field = TextField::builder()
        .text("Type something here...")
        .placeholder("Enter your message")
        .size(300.0, 40.0)
        .on_change(|text| {
            println!("📝 Text changed: {}", text);
        })
        .build()?;

    println!("✓ UI components created:");
    println!("  - Label: 'Welcome to Cocoanut!' (300x30)");
    println!("  - Button: 'Click Me!' (120x40)");
    println!("  - TextField: 'Type something here...' (300x40)\n");

    // Add components to window
    println!("Adding components to window...");

    // Get the native button object and add it to the window
    let button_ns = button.as_view();
    window.add_subview(button_ns)?;

    // Get the native label object and add it to the window
    let label_ns = label.as_view();
    window.add_subview(label_ns)?;

    // Get the native text field object and add it to the window
    let text_field_ns = text_field.as_view();
    window.add_subview(text_field_ns)?;

    println!("✓ Components added to window\n");

    println!("🚀 Launching GUI with components...\n");
    println!("Features demonstrated:");
    println!("  ✅ Native macOS window");
    println!("  ✅ Button with click handler");
    println!("  ✅ Label with custom text");
    println!("  ✅ Text field with change handler");
    println!("  ✅ Event loop integration");
    println!("\nPress Cmd+Q to quit\n");

    // Run the application with the window
    app("Components Demo")
        .with_window(window)
        .run()?;

    println!("\n✅ Application closed");

    Ok(())
}