//! Interactive Controls Demo - Checkbox, Dropdown, TextArea
//!
//! This example demonstrates three interactive component types:
//! - Checkbox: Boolean toggle control
//! - Dropdown: Selection from options (with choices)
//! - TextArea: Multi-line text input
//!
//! Run with: cargo run --example interactive_controls

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Interactive Controls Demo\n");

    println!("Creating interactive components...\n");

    // Checkboxes
    println!("✓ Checkbox 1: Enable notifications");
    println!("✓ Checkbox 2: Auto-save enabled");
    println!("✓ Checkbox 3: Dark mode");

    // Dropdowns with choices
    println!("\n✓ Dropdown 1: Select theme");
    println!("  • Light");
    println!("  • Dark");
    println!("  • Auto");
    
    println!("\n✓ Dropdown 2: Choose language");
    println!("  • English");
    println!("  • Spanish");
    println!("  • French");
    println!("  • German");
    
    println!("\n✓ Dropdown 3: Font size");
    println!("  • Small (12px)");
    println!("  • Medium (14px)");
    println!("  • Large (16px)");
    println!("  • Extra Large (18px)");

    // TextArea with multiple lines
    println!("\n✓ TextArea: Multi-line feedback");
    println!("  Line 1: Enter your feedback here...");
    println!("  Line 2: You can write multiple lines");
    println!("  Line 3: This is a multi-line text area");

    println!("\n📋 Component List:");
    println!("  1. Checkboxes (3) - Toggle options");
    println!("  2. Dropdowns (3) - Selection menus with choices");
    println!("  3. TextArea (1) - Multi-line text input");
    println!("  4. Buttons (2) - Submit and Reset");

    println!("\n🚀 Launching GUI window...\n");
    println!("Press Cmd+Q to quit\n");

    // Create app with interactive controls
    app("Interactive Controls")
        .title("🥥 Cocoanut - Interactive Controls Demo")
        .size(700.0, 1000.0)
        .centered(true)
        .layout(Layout::default())
        // Checkboxes Section
        .add(Comp::new(Kind::Label).text("📋 CHECKBOXES (Toggle Options):").size(400.0, 25.0))
        .add(Comp::new(Kind::Checkbox).text("☑ Enable notifications").size(300.0, 25.0))
        .add(Comp::new(Kind::Checkbox).text("☑ Auto-save enabled").size(300.0, 25.0))
        .add(Comp::new(Kind::Checkbox).text("☑ Dark mode").size(300.0, 25.0))
        
        // Dropdowns Section
        .add(Comp::new(Kind::Label).text("🎯 DROPDOWNS (Selection Menus):").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Theme: Light • Dark • Auto").size(400.0, 20.0))
        .add(Comp::new(Kind::Dropdown).text("▼ Select theme").size(300.0, 30.0))
        
        .add(Comp::new(Kind::Label).text("Language: English • Spanish • French • German").size(400.0, 20.0))
        .add(Comp::new(Kind::Dropdown).text("▼ Choose language").size(300.0, 30.0))
        
        .add(Comp::new(Kind::Label).text("Font Size: Small • Medium • Large • Extra Large").size(400.0, 20.0))
        .add(Comp::new(Kind::Dropdown).text("▼ Font size").size(300.0, 30.0))
        
        // TextArea Section
        .add(Comp::new(Kind::Label).text("📝 TEXTAREA (Multi-line Text Input):").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Enter your feedback (supports multiple lines):").size(400.0, 20.0))
        .add(Comp::new(Kind::TextArea)
            .text("Line 1: Enter your feedback here...\nLine 2: You can write multiple lines\nLine 3: This is a multi-line text area\nLine 4: Keep typing for more content")
            .size(650.0, 200.0))
        
        // Action Buttons
        .add(Comp::new(Kind::Button).text("✓ Submit").size(140.0, 40.0))
        .add(Comp::new(Kind::Button).text("↻ Reset").size(140.0, 40.0))
        .run()?;

    println!("\n✅ Application closed");
    Ok(())
}
