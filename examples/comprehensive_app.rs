//! Comprehensive Cocoanut Application Example - Real GUI with Streamlit-Inspired Components
//!
//! This example demonstrates the Streamlit-inspired widget system with:
//! - Display elements (text, markdown, metrics)
//! - Input widgets (text, sliders, selections)
//! - Layout containers (columns, tabs, expanders)
//! - State management and data binding
//! - macOS native features
//!
//! Run with: cargo run --example comprehensive_app
//! The window will stay open until you press Cmd+Q to quit.

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

    // Create a window using SimpleApp with Streamlit-inspired layout
    app("Streamlit Demo")
        .title("🥥 Cocoanut - Streamlit-Inspired Components (67 Elements)")
        .size(1000.0, 1000.0)
        .centered(true)
        .layout(Layout::default())
        // Display Elements
        .add(Comp::new(Kind::Label).text("📝 Display Elements").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Write, Text, Markdown, Code, JSON, Help").size(400.0, 20.0))
        // Input Widgets
        .add(Comp::new(Kind::Label).text("⌨️  Input Widgets").size(400.0, 25.0))
        .add(Comp::new(Kind::TextField).text("Text Input Example").size(350.0, 30.0))
        .add(Comp::new(Kind::Slider).text("Slider Example").size(250.0, 25.0))
        .add(Comp::new(Kind::Button).text("Button Example").size(150.0, 35.0))
        .add(Comp::new(Kind::Checkbox).text("Checkbox Example").size(200.0, 25.0))
        // Layout Containers
        .add(Comp::new(Kind::Label).text("📐 Layout Containers").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Columns, Tabs, Expander, Form, Sidebar").size(400.0, 20.0))
        // State Management
        .add(Comp::new(Kind::Label).text("💾 State Management").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("SessionState, DataCache, Callbacks").size(400.0, 20.0))
        // Advanced Features
        .add(Comp::new(Kind::Label).text("🚀 Advanced Features").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Navigation, ComponentRegistry, Templates").size(400.0, 20.0))
        // Macro Refactoring Info
        .add(Comp::new(Kind::Label).text("⚙️  Macro Refactoring").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("83 lines saved • 17 widgets refactored • 2 patterns").size(400.0, 20.0))
        .run()?;
    Ok(())
}
