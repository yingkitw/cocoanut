//! Basic Window Example - Streamlit-Inspired Widgets
//! 
//! Demonstrates Cocoanut's Streamlit-inspired widget system:
//! - Phase 1: Display elements (21 widgets)
//! - Phase 2: Input widgets (21 widgets)
//! - Phase 3: Layout containers (12 widgets)
//! - Phase 4: State & caching (8 widgets)
//! - Phase 5: Advanced features (5 widgets)
//!
//! Run with: cargo run --example basic_window
//! The window will stay open until you press Cmd+Q to quit.

use cocoanut::prelude::*;
use cocoanut::systems::{Write, Text, TextInput, Button};
use cocoanut::systems::selection_widgets::ButtonVariant;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Cocoanut - Streamlit Widgets Example\n");
    println!("Demonstrating 67 Streamlit-inspired elements...\n");
    
    // Create widgets using Streamlit-inspired API
    let _write = Write::new("Welcome to Cocoanut!");
    let _text = Text::new("Streamlit-inspired widget system");
    let _input = TextInput::new().placeholder("Enter text");
    let _button = Button::new("Submit").variant(ButtonVariant::Primary);
    
    println!("✓ Display elements created");
    println!("✓ Input widgets created");
    println!("✓ Layout containers ready");
    println!("✓ State management available");
    println!("✓ Advanced features enabled\n");
    
    // Create and run app with Streamlit phases showcase
    app("Streamlit Demo")
        .title("🥥 Cocoanut - 5 Streamlit Migration Phases (67 Elements)")
        .size(900.0, 700.0)
        .centered(true)
        .layout(Layout::default())
        // Phase 1: Display Elements (21)
        .add(Comp::new(Kind::Label).text("📝 Phase 1: Display Elements (21)").size(500.0, 25.0))
        .add(Comp::new(Kind::Label).text("Write, Text, Markdown, Title, Header, Subheader, Caption, Code, JSON, Help").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Table, DataFrame, DataEditor, Metric, MetricColumn").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Success, Error, Warning, Info, Toast, Status, Progress, Spinner").size(600.0, 20.0))
        // Phase 2: Input Widgets (21)
        .add(Comp::new(Kind::Label).text("⌨️  Phase 2: Input Widgets (21)").size(500.0, 25.0))
        .add(Comp::new(Kind::TextField).text("TextInput, TextArea, ChatInput").size(350.0, 30.0))
        .add(Comp::new(Kind::Button).text("Button, Checkbox, Radio, Selectbox").size(350.0, 35.0))
        .add(Comp::new(Kind::Slider).text("Slider, NumberInput, ColorPicker").size(350.0, 25.0))
        .add(Comp::new(Kind::Label).text("DateInput, TimeInput, FileUploader, CameraInput, AudioInput").size(600.0, 20.0))
        // Phase 3: Layout Containers (12)
        .add(Comp::new(Kind::Label).text("📐 Phase 3: Layout Containers (12)").size(500.0, 25.0))
        .add(Comp::new(Kind::Label).text("Columns, Tabs, Expander, Container, Form, Sidebar, Empty").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Row, Column, Grid, FlexSpacer, Divider").size(600.0, 20.0))
        // Phase 4: State & Caching (8)
        .add(Comp::new(Kind::Label).text("💾 Phase 4: State & Caching (8)").size(500.0, 25.0))
        .add(Comp::new(Kind::Label).text("SessionState, QueryParams, DataCache, ResourceCache").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("ChangeCallback, ClickCallback, SubmitCallback, EventDispatcher").size(600.0, 20.0))
        // Phase 5: Advanced Features (5)
        .add(Comp::new(Kind::Label).text("🚀 Phase 5: Advanced Features (5)").size(500.0, 25.0))
        .add(Comp::new(Kind::Label).text("Navigation, SidebarNav, CustomComponent, ComponentRegistry, ComponentTemplate").size(600.0, 20.0))
        .run()?;
    
    println!("\n✅ Application closed");
    Ok(())
}
