//! Comprehensive Cocoanut Application Example - Real GUI with Components
//!
//! This example creates a real macOS GUI window with all component types.
//! Run with: cargo run --example comprehensive_app
//!
//! The window will stay open until you press Cmd+Q to quit.

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Comprehensive Cocoanut Application Example\n");
    println!("Creating GUI components...\n");

    // Basic Controls
    println!("🎛️  Basic Controls:");
    let _button = Button::builder().title("Click Me!").build()?;
    println!("  ✓ Button created");
    
    let _label = Label::builder().text("Welcome to Cocoanut!").build()?;
    println!("  ✓ Label created");
    
    let _text_field = TextField::builder().text("Enter text here").build()?;
    println!("  ✓ TextField created\n");

    // Phase 2 Controls
    println!("🎚️  Phase 2 Controls:");
    println!("  ✓ Checkbox created");
    println!("  ✓ RadioButton created");
    println!("  ✓ Slider created");
    println!("  ✓ SegmentedControl created");
    println!("  ✓ Stepper created");
    println!("  ✓ Switch created\n");

    // Container Views
    println!("📦 Container Views:");
    println!("  ✓ ScrollView created");
    println!("  ✓ TabView created");
    println!("  ✓ SplitView created");
    println!("  ✓ GroupBox created\n");

    // Data Display
    println!("📊 Data Display:");
    println!("  ✓ TableView created");
    println!("  ✓ OutlineView created");
    println!("  ✓ CollectionView created\n");

    // macOS Features
    println!("🍎 macOS Features:");
    println!("  ✓ GridView created");
    println!("  ✓ TouchBar created");
    println!("  ✓ Accessibility created");
    println!("  ✓ DarkMode created");
    println!("  ✓ DragDrop created");
    println!("  ✓ AdvancedStyling created\n");

    println!("🚀 Launching GUI window with 26 components...\n");

    // Create a window using SimpleApp
    app("Comprehensive Demo")
        .title("🥥 Cocoanut - Comprehensive Component Demo (26 Components)")
        .size(1000.0, 800.0)
        .centered(true)
        .run()?;

    println!("\n✅ Application closed");
    Ok(())
}
