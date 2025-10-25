//! Example demonstrating Event Binding with Button and TextField
//!
//! This example shows how to use on_click() and on_change() callbacks
//! for event handling in a more intuitive way.

use cocoanut::prelude::*;
use std::sync::{Arc, Mutex};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Event Binding Example\n");

    // Example 1: Button with on_click callback
    println!("Example 1: Button with on_click");
    let click_count = Arc::new(Mutex::new(0));
    let click_count_clone = click_count.clone();
    
    let _button = Button::builder()
        .title("Click Me")
        .size(100.0, 50.0)
        .on_click(move || {
            let mut count = click_count_clone.lock().unwrap();
            *count += 1;
            println!("  Button clicked! Count: {}", *count);
        })
        .build()?;
    
    println!("✓ Created button with on_click callback\n");

    // Example 2: TextField with on_change callback
    println!("Example 2: TextField with on_change");
    let last_value = Arc::new(Mutex::new(String::new()));
    let last_value_clone = last_value.clone();
    
    let _field = TextField::builder()
        .text("Initial")
        .placeholder("Type something...")
        .size(300.0, 40.0)
        .on_change(move |text| {
            let mut value = last_value_clone.lock().unwrap();
            *value = text.clone();
            println!("  Text changed to: {}", text);
        })
        .build()?;
    
    println!("✓ Created text field with on_change callback\n");

    // Example 3: Multiple buttons with different callbacks
    println!("Example 3: Multiple buttons with different callbacks");
    
    let _button1 = Button::builder()
        .title("Save")
        .on_click(|| println!("  Save button clicked"))
        .build()?;
    
    let _button2 = Button::builder()
        .title("Cancel")
        .on_click(|| println!("  Cancel button clicked"))
        .build()?;
    
    let _button3 = Button::builder()
        .title("Delete")
        .on_click(|| println!("  Delete button clicked"))
        .build()?;
    
    println!("✓ Created 3 buttons with different callbacks\n");

    // Example 4: Fluent API with event binding
    println!("Example 4: Fluent API with event binding");
    
    let _button = Button::builder()
        .title("Action")
        .size(120.0, 40.0)
        .enabled(true)
        .on_click(|| println!("  Action performed!"))
        .build()?;
    
    println!("✓ Created button with fluent API and event binding\n");

    // Example 5: TextField with validation callback
    println!("Example 5: TextField with validation");
    
    let _field = TextField::builder()
        .text("")
        .placeholder("Enter email")
        .size(300.0, 40.0)
        .editable(true)
        .on_change(|text| {
            if text.contains('@') {
                println!("  Valid email: {}", text);
            } else {
                println!("  Invalid email: {}", text);
            }
        })
        .build()?;
    
    println!("✓ Created text field with validation callback\n");

    // Example 6: Comparison with old API
    println!("Example 6: Comparison");
    println!("Old API (verbose):");
    println!("  let button = Button::new(\"Click\")?;");
    println!("  // Manual event handling elsewhere...");
    println!();
    println!("New API (with event binding):");
    println!("  let button = Button::builder()");
    println!("    .title(\"Click\")");
    println!("    .on_click(|| println!(\"Clicked!\"))");
    println!("    .build()?;");
    println!();

    println!("✅ All event binding examples completed!");
    println!("\n📝 Key Benefits:");
    println!("  • Callbacks defined at component creation time");
    println!("  • Fluent API for readable code");
    println!("  • Type-safe event handling");
    println!("  • Support for closures with captured variables");
    println!("  • Works with both Button and TextField");

    Ok(())
}
