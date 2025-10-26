//! GUI demonstration of container components
//!
//! This example shows containers displayed in a macOS window with descriptions.

use cocoanut::prelude::*;
use cocoanut::simple_app::{Comp, Kind, Layout};
use cocoanut::components::containers::{ScrollView, TabView, SplitView, SplitOrientation, GroupBox};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n🥥 Container GUI Components Demo\n");
    println!("===================================\n");

    // =========================================================================
    // CREATE CONTAINERS (Data structures)
    // =========================================================================
    println!("Creating container data structures...\n");

    // ScrollView
    let _scroll_view = ScrollView::builder()
        .size(350.0, 200.0)
        .content_size(350.0, 400.0)
        .build()?;
    println!("  ✓ ScrollView created");
    println!("    - Visible: 350x200");
    println!("    - Content: 350x400\n");

    // TabView
    let _tab_view = TabView::builder()
        .tab("Settings")
        .tab("Advanced")
        .tab("About")
        .build()?;
    println!("  ✓ TabView created");
    println!("    - Tabs: Settings, Advanced, About\n");

    // SplitView (Vertical)
    let _split_view = SplitView::builder()
        .orientation(SplitOrientation::Vertical)
        .divider_position(0.4)
        .build()?;
    println!("  ✓ SplitView created");
    println!("    - Orientation: Vertical (left/right)");
    println!("    - Divider: 40% left, 60% right\n");

    // GroupBox
    let _group_box = GroupBox::builder()
        .title("Display Options")
        .build()?;
    println!("  ✓ GroupBox created");
    println!("    - Title: Display Options\n");

    // =========================================================================
    // DISPLAY GUI WINDOW WITH CONTAINER INFORMATION
    // =========================================================================
    println!("Launching GUI window with container information...\n");

    app("Container Components")
        .title("🥥 Container Components Demo")
        .size(900.0, 1100.0)
        .centered(true)
        .layout(Layout::default())
        // Title
        .add(Comp::new(Kind::Label).text("🥥 Container Components Demo").size(600.0, 30.0))
        .add(Comp::new(Kind::Label).text("Cocoanut provides 4 powerful container types").size(600.0, 20.0))
        
        // ScrollView Section
        .add(Comp::new(Kind::Label).text("📜 ScrollView - Scrollable Content Areas").size(600.0, 25.0))
        .add(Comp::new(Kind::Label).text("Visible: 350x200 | Content: 350x400").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Use for: Long lists, documents, large content").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Features: Automatic scrollbars, configurable sizes").size(600.0, 20.0))
        
        // TabView Section
        .add(Comp::new(Kind::Label).text("📑 TabView - Tabbed Interfaces").size(600.0, 25.0))
        .add(Comp::new(Kind::Label).text("Tabs: Settings, Advanced, About").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Use for: Settings dialogs, preferences, multi-section content").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Features: Tab selection, builder pattern").size(600.0, 20.0))
        
        // SplitView Section
        .add(Comp::new(Kind::Label).text("⚔️  SplitView - Resizable Panes").size(600.0, 25.0))
        .add(Comp::new(Kind::Label).text("Orientation: Vertical (left/right) | Divider: 40/60").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Use for: File browsers, split editors, IDE layouts").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Features: User-draggable divider, configurable position").size(600.0, 20.0))
        
        // GroupBox Section
        .add(Comp::new(Kind::Label).text("📦 GroupBox - Grouped Controls").size(600.0, 25.0))
        .add(Comp::new(Kind::Label).text("Title: Display Options").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Use for: Grouping related controls, visual organization").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Features: Border, title, visual grouping").size(600.0, 20.0))
        
        // Builder Pattern Section
        .add(Comp::new(Kind::Label).text("🛠️  Builder Pattern - Easy Configuration").size(600.0, 25.0))
        .add(Comp::new(Kind::Label).text("All containers use fluent builder API").size(600.0, 20.0))
        .add(Comp::new(Kind::Label).text("Example: ScrollView::builder().size(350, 200).build()?").size(600.0, 20.0))
        
        // Summary
        .add(Comp::new(Kind::Label).text("✅ Container Components Ready for Use").size(600.0, 25.0))
        .add(Comp::new(Kind::Label).text("Run 'containers_demo' for detailed console output").size(600.0, 20.0))
        
        .run()?;

    Ok(())
}
