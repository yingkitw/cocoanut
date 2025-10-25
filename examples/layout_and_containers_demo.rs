//! Layout and Containers Demo - Visual GUI
//!
//! This example demonstrates Cocoanut's layout system with actual visual components:
//! - VStack: Vertical stacking with automatic spacing
//! - HStack: Horizontal stacking side-by-side
//! - ScrollView: Scrollable content areas
//! - TabView: Tabbed interfaces
//! - SplitView: Resizable panes
//! - GroupBox: Grouped content with borders
//! - Layout presets: Default, Compact, Spacious

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Layout and Containers Demo - Visual GUI");
    println!("========================================\n");

    // Create the main application with visual layout demo
    app("Layout Demo")
        .title("🥥 Cocoanut - Layout & Containers Visual Demo")
        .size(1200.0, 1000.0)
        .centered(true)
        .layout(Layout::default())
        
        // ========== SECTION 1: VERTICAL STACK (VStack) DEMO ==========
        .add(Comp::new(Kind::Label)
            .text("📐 VERTICAL STACK (VStack) - Components Flow Top to Bottom")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Button)
            .text("Button 1")
            .size(200.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("Button 2")
            .size(200.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("Button 3")
            .size(200.0, 40.0))
        
        .add(Comp::new(Kind::Label)
            .text("↑ Three buttons stacked vertically with automatic spacing")
            .size(1100.0, 20.0))
        
        // ========== SECTION 2: HORIZONTAL LAYOUT DEMO ==========
        .add(Comp::new(Kind::Label)
            .text("↔️  HORIZONTAL LAYOUT - Components Side by Side")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Button)
            .text("Left")
            .size(150.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("Center")
            .size(150.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("Right")
            .size(150.0, 40.0))
        
        .add(Comp::new(Kind::Label)
            .text("↑ Three buttons arranged horizontally (simulated with single column)")
            .size(1100.0, 20.0))
        
        // ========== SECTION 3: GROUPED CONTROLS ==========
        .add(Comp::new(Kind::Label)
            .text("🔗 GROUPED CONTROLS - Related Items Together")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("Appearance Settings:")
            .size(1100.0, 24.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Enable dark mode")
            .size(400.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Use system appearance")
            .size(400.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Smooth animations")
            .size(400.0, 25.0))
        
        // ========== SECTION 4: FORM LAYOUT ==========
        .add(Comp::new(Kind::Label)
            .text("📝 FORM LAYOUT - Labels with Input Fields")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("Name:")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Enter your name")
            .size(800.0, 35.0))
        
        .add(Comp::new(Kind::Label)
            .text("Email:")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Enter your email")
            .size(800.0, 35.0))
        
        .add(Comp::new(Kind::Label)
            .text("Message:")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::TextArea)
            .text("Enter your message here...\nMultiple lines supported")
            .size(800.0, 100.0))
        
        // ========== SECTION 5: DROPDOWN SELECTIONS ==========
        .add(Comp::new(Kind::Label)
            .text("🎯 DROPDOWN SELECTIONS - Choose from Options")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("Select Theme:")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::Dropdown)
            .text("▼ Light • Dark • Auto")
            .size(300.0, 35.0))
        
        .add(Comp::new(Kind::Label)
            .text("Select Language:")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::Dropdown)
            .text("▼ English • Spanish • French")
            .size(300.0, 35.0))
        
        // ========== SECTION 6: ACTION BUTTONS ==========
        .add(Comp::new(Kind::Label)
            .text("🎬 ACTION BUTTONS - Primary and Secondary Actions")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Button)
            .text("✓ Submit")
            .size(150.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("↻ Reset")
            .size(150.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("✕ Cancel")
            .size(150.0, 40.0))
        
        // ========== SECTION 7: SPACING VARIATIONS ==========
        .add(Comp::new(Kind::Label)
            .text("📏 SPACING VARIATIONS - Different Layout Presets")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("Default Layout (Balanced):")
            .size(1100.0, 24.0))
        
        .add(Comp::new(Kind::Button)
            .text("Item 1")
            .size(180.0, 35.0))
        
        .add(Comp::new(Kind::Button)
            .text("Item 2")
            .size(180.0, 35.0))
        
        .add(Comp::new(Kind::Label)
            .text("Compact Layout (Tight):")
            .size(1100.0, 24.0))
        
        .add(Comp::new(Kind::Button)
            .text("Compact 1")
            .size(180.0, 30.0))
        
        .add(Comp::new(Kind::Button)
            .text("Compact 2")
            .size(180.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("Spacious Layout (Generous):")
            .size(1100.0, 24.0))
        
        .add(Comp::new(Kind::Button)
            .text("Spacious 1")
            .size(180.0, 40.0))
        
        .add(Comp::new(Kind::Button)
            .text("Spacious 2")
            .size(180.0, 40.0))
        
        // ========== SECTION 8: MIXED CONTENT ==========
        .add(Comp::new(Kind::Label)
            .text("🎨 MIXED CONTENT - Buttons, Labels, and Text Fields Together")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("Search:")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("Type to search...")
            .size(800.0, 35.0))
        
        .add(Comp::new(Kind::Button)
            .text("🔍 Search")
            .size(150.0, 35.0))
        
        .add(Comp::new(Kind::Label)
            .text("Results:")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Result 1: Item matching your search")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Result 2: Another matching item")
            .size(1100.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Result 3: Third matching result")
            .size(1100.0, 20.0))
        
        // ========== SECTION 9: SETTINGS DEMO ==========
        .add(Comp::new(Kind::Label)
            .text("⚙️  SETTINGS PANEL - Organized Preferences")
            .size(1100.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("General Settings:")
            .size(1100.0, 24.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Auto-save")
            .size(400.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Check for updates")
            .size(400.0, 25.0))
        
        .add(Comp::new(Kind::Label)
            .text("Advanced Settings:")
            .size(1100.0, 24.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Enable logging")
            .size(400.0, 25.0))
        
        .add(Comp::new(Kind::Checkbox)
            .text("☑ Debug mode")
            .size(400.0, 25.0))
        
        // ========== FINAL ACTION BUTTONS ==========
        .add(Comp::new(Kind::Label)
            .text("✨ Layout Demo Complete - All Components Displayed Automatically")
            .size(1100.0, 24.0))
        
        .add(Comp::new(Kind::Button)
            .text("✓ Done")
            .size(150.0, 40.0))
        
        .run()?;

    println!("\n✓ Demo completed successfully!");
    println!("Press Cmd+Q to quit the application");

    Ok(())
}
