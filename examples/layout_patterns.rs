//! Layout Patterns Demo
//!
//! This example demonstrates practical layout patterns and best practices:
//! - Form layouts (labels + inputs)
//! - Settings panels with grouped controls
//! - Multi-section layouts
//! - Responsive spacing and sizing

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Layout Patterns Demo");
    println!("========================================\n");

    app("Layout Patterns")
        .title("🥥 Cocoanut - Layout Patterns & Best Practices")
        .size(900.0, 1100.0)
        .centered(true)
        .layout(Layout::default())
        
        // ========== PATTERN 1: FORM LAYOUT ==========
        .add(Comp::new(Kind::Label)
            .text("📋 PATTERN 1: FORM LAYOUT")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("A classic form layout with labels and input fields")
            .size(850.0, 20.0))
        
        // Form Section
        .add(Comp::new(Kind::Label)
            .text("User Information:")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Label)
            .text("Full Name:")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Enter your full name")
            .size(800.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Email Address:")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Enter your email")
            .size(800.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Phone Number:")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Enter your phone number")
            .size(800.0, 30.0))
        
        // ========== PATTERN 2: SETTINGS PANEL ==========
        .add(Comp::new(Kind::Label)
            .text("⚙️  PATTERN 2: SETTINGS PANEL")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Grouped controls for application settings")
            .size(850.0, 20.0))
        
        // Appearance Settings
        .add(Comp::new(Kind::Label)
            .text("Appearance Settings:")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Enable dark mode")
            .size(800.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Use system appearance")
            .size(800.0, 25.0))
        
        .add(Comp::new(Kind::Label)
            .text("Theme:")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Dropdown)
            .text("▼ Select theme")
            .size(300.0, 30.0))
        
        // Notification Settings
        .add(Comp::new(Kind::Label)
            .text("Notification Settings:")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Enable notifications")
            .size(800.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Sound alerts")
            .size(800.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Badge count")
            .size(800.0, 25.0))
        
        // ========== PATTERN 3: MULTI-SECTION LAYOUT ==========
        .add(Comp::new(Kind::Label)
            .text("📑 PATTERN 3: MULTI-SECTION LAYOUT")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Organized content with clear visual separation")
            .size(850.0, 20.0))
        
        // Section 1
        .add(Comp::new(Kind::Label)
            .text("Section 1: Basic Options")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Option A")
            .size(800.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Option B")
            .size(800.0, 25.0))
        
        // Section 2
        .add(Comp::new(Kind::Label)
            .text("Section 2: Advanced Options")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Advanced Feature 1")
            .size(800.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Advanced Feature 2")
            .size(800.0, 25.0))
        
        // Section 3
        .add(Comp::new(Kind::Label)
            .text("Section 3: Experimental Features")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Beta Feature 1")
            .size(800.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Beta Feature 2")
            .size(800.0, 25.0))
        
        // ========== PATTERN 4: RESPONSIVE SIZING ==========
        .add(Comp::new(Kind::Label)
            .text("📏 PATTERN 4: RESPONSIVE SIZING")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Components adapt to available space")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("Full-width component (adapts to window):")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("This field takes full available width")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Half-width components (side by side):")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Left field")
            .size(400.0, 30.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Right field")
            .size(400.0, 30.0))
        
        // ========== PATTERN 5: SPACING VARIATIONS ==========
        .add(Comp::new(Kind::Label)
            .text("🎨 PATTERN 5: SPACING VARIATIONS")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Three layout presets with different spacing:")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("Default Layout:")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Top padding: 40px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Horizontal margin: 20px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Gap between components: 12px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("Compact Layout:")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Top padding: 20px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Horizontal margin: 10px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Gap between components: 8px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("Spacious Layout:")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Top padding: 60px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Horizontal margin: 40px")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Gap between components: 20px")
            .size(850.0, 20.0))
        
        // ========== PATTERN 6: CONTENT GROUPING ==========
        .add(Comp::new(Kind::Label)
            .text("🔗 PATTERN 6: CONTENT GROUPING")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Related content grouped with visual separation")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("Group 1: Related Controls")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Button)
            .text("Action 1")
            .size(140.0, 35.0))
        
        .add(Comp::new(Kind::Button)
            .text("Action 2")
            .size(140.0, 35.0))
        
        .add(Comp::new(Kind::Label)
            .text("Group 2: Additional Actions")
            .size(850.0, 25.0))
        
        .add(Comp::new(Kind::Button)
            .text("Save")
            .size(140.0, 35.0))
        
        .add(Comp::new(Kind::Button)
            .text("Cancel")
            .size(140.0, 35.0))
        
        // ========== TIPS AND TRICKS ==========
        .add(Comp::new(Kind::Label)
            .text("💡 TIPS & TRICKS")
            .size(850.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("1. Use section labels to organize content visually")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("2. Group related controls together for clarity")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("3. Use consistent sizing for professional appearance")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("4. Leave adequate spacing between sections")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("5. Use TextArea for multi-line content")
            .size(850.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("6. Choose layout preset based on content density")
            .size(850.0, 20.0))
        
        // ========== ACTION BUTTONS ==========
        .add(Comp::new(Kind::Button)
            .text("✓ Apply")
            .size(140.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("↻ Reset")
            .size(140.0, 40.0))
        
        .run()?;

    println!("\n✓ Layout patterns demo completed!");
    println!("Press Cmd+Q to quit the application");

    Ok(())
}
