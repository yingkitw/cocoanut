//! Demonstration of container and layout components
//!
//! This example showcases:
//! - ScrollView for scrollable content
//! - TabView for tabbed interfaces
//! - SplitView for splittable layouts
//! - GroupBox for grouped content

use cocoanut::components::containers::{
    ScrollView, TabView, SplitView, SplitOrientation, GroupBox,
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n🥥 Container & Layout Components Demo\n");
    println!("=====================================\n");

    // =========================================================================
    // SCROLL VIEW EXAMPLE
    // =========================================================================
    println!("📜 ScrollView Example:");
    let scroll_view = ScrollView::builder()
        .size(400.0, 300.0)
        .content_size(400.0, 600.0)
        .build()?;
    
    let (width, height) = scroll_view.size();
    let (content_width, content_height) = scroll_view.scrollable_size();
    println!("  ✓ ScrollView created");
    println!("    - Visible size: {}x{}", width, height);
    println!("    - Content size: {}x{}", content_width, content_height);
    println!("    - Scrollable: Yes\n");

    // =========================================================================
    // TAB VIEW EXAMPLE
    // =========================================================================
    println!("📑 TabView Example:");
    let tab_view = TabView::builder()
        .tab("General")
        .tab("Advanced")
        .tab("Settings")
        .build()?;
    
    println!("  ✓ TabView created");
    println!("    - Tab 1: General");
    println!("    - Tab 2: Advanced");
    println!("    - Tab 3: Settings");
    println!("    - Total tabs: {}", tab_view.tabs().len());
    println!("    - Selected tab: {}\n", tab_view.selected_tab());

    // =========================================================================
    // SPLIT VIEW EXAMPLE
    // =========================================================================
    println!("⚔️  SplitView Example:");
    let mut split_view_h = SplitView::builder()
        .orientation(SplitOrientation::Horizontal)
        .divider_position(0.3)
        .build()?;
    
    let mut split_view_v = SplitView::builder()
        .orientation(SplitOrientation::Vertical)
        .divider_position(0.5)
        .build()?;
    
    println!("  ✓ Horizontal SplitView created");
    println!("    - Orientation: Horizontal (top/bottom)");
    println!("    - Divider position: 30%");
    println!("    - Top pane: 30%, Bottom pane: 70%");
    
    println!("  ✓ Vertical SplitView created");
    println!("    - Orientation: Vertical (left/right)");
    println!("    - Divider position: 50%");
    println!("    - Left pane: 50%, Right pane: 50%\n");

    // =========================================================================
    // GROUP BOX EXAMPLE
    // =========================================================================
    println!("📦 GroupBox Example:");
    let group_box = GroupBox::builder()
        .title("Settings")
        .build()?;
    
    println!("  ✓ GroupBox created");
    println!("    - Title: {}", group_box.title());
    println!("    - Border: Yes");
    println!("    - Grouping: Related controls\n");

    // =========================================================================
    // CONTAINER FEATURES SUMMARY
    // =========================================================================
    println!("✅ Container Components Features:\n");
    
    println!("📜 ScrollView:");
    println!("  - Scrollable content areas");
    println!("  - Configurable visible and content sizes");
    println!("  - Automatic scrollbars when needed");
    println!("  - Builder pattern for easy configuration\n");
    
    println!("📑 TabView:");
    println!("  - Tabbed interfaces");
    println!("  - Multiple tabs with labels");
    println!("  - Tab selection tracking");
    println!("  - Builder pattern for tab creation\n");
    
    println!("⚔️  SplitView:");
    println!("  - Resizable panes (horizontal or vertical)");
    println!("  - Configurable divider position (0.0-1.0)");
    println!("  - Two orientations: Horizontal & Vertical");
    println!("  - User-draggable divider\n");
    
    println!("📦 GroupBox:");
    println!("  - Grouped content with borders");
    println!("  - Title for group identification");
    println!("  - Visual grouping of related controls");
    println!("  - Builder pattern for configuration\n");

    // =========================================================================
    // USAGE PATTERNS
    // =========================================================================
    println!("🎯 Common Usage Patterns:\n");
    
    println!("Pattern 1: Scrollable List");
    println!("  ScrollView (400x300) → Content (400x600)");
    println!("  Use for: Long lists, documents\n");
    
    println!("Pattern 2: Settings Dialog");
    println!("  TabView with tabs:");
    println!("    - General settings");
    println!("    - Advanced settings");
    println!("    - About\n");
    
    println!("Pattern 3: Split Editor");
    println!("  SplitView (Vertical) → Left: File tree, Right: Editor");
    println!("  Use for: IDEs, file managers\n");
    
    println!("Pattern 4: Grouped Controls");
    println!("  GroupBox → Multiple controls inside");
    println!("  Use for: Related options, preferences\n");

    println!("🚀 All container types demonstrated successfully!\n");

    Ok(())
}
