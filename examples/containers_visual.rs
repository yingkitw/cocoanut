//! Visual demonstration of container components in a GUI window
//!
//! This example displays actual container components (ScrollView, TabView, SplitView, GroupBox)
//! rendered as NSView objects in a macOS window.

use cocoanut::prelude::*;
use cocoanut::simple_app::{Comp, Kind, Layout};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n🥥 Container Components - Visual Demo\n");
    println!("=====================================\n");

    app("Container Components")
        .title("🥥 Container Components - Visual Demo")
        .size(900.0, 900.0)
        .centered(true)
        .layout(Layout::default())
        
        // Title
        .add(Comp::new(Kind::Label).text("🥥 Container Components - Now Displaying!").size(700.0, 30.0))
        .add(Comp::new(Kind::Label).text("Actual NSView containers rendered in the window").size(700.0, 20.0))
        
        // ScrollView Section
        .add(Comp::new(Kind::Label).text("📜 ScrollView Container").size(700.0, 25.0))
        .add(Comp::new(Kind::Label).text("Scrollable content area with vertical scrollbar").size(700.0, 20.0))
        .add(Comp::new(Kind::ScrollView).text("ScrollView").size(400.0, 120.0))
        
        // TabView Section
        .add(Comp::new(Kind::Label).text("📑 TabView Container").size(700.0, 25.0))
        .add(Comp::new(Kind::Label).text("Tabbed interface with multiple tabs").size(700.0, 20.0))
        .add(Comp::new(Kind::TabView).text("TabView").size(400.0, 120.0))
        
        // SplitView Section
        .add(Comp::new(Kind::Label).text("⚔️  SplitView Container").size(700.0, 25.0))
        .add(Comp::new(Kind::Label).text("Resizable panes with draggable divider").size(700.0, 20.0))
        .add(Comp::new(Kind::SplitView).text("SplitView").size(400.0, 120.0))
        
        // GroupBox Section
        .add(Comp::new(Kind::Label).text("📦 GroupBox Container").size(700.0, 25.0))
        .add(Comp::new(Kind::Label).text("Grouped controls with border and title").size(700.0, 20.0))
        .add(Comp::new(Kind::GroupBox).text("Display Options").size(400.0, 120.0))
        
        // Summary
        .add(Comp::new(Kind::Label).text("✅ All Containers Now Displaying!").size(700.0, 25.0))
        .add(Comp::new(Kind::Label).text("Containers are rendered as actual NSView components").size(700.0, 20.0))
        
        .run()?;

    Ok(())
}
