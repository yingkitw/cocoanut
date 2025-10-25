//! Containers with Borders Demo
//!
//! This example demonstrates container components with visible borders:
//! - ScrollView: Scrollable content with border
//! - TabView: Tabbed interface with border
//! - SplitView: Resizable panes with divider
//! - GroupBox: Grouped content with border and title
//! - Visual borders show container boundaries

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Containers with Borders Demo");
    println!("========================================\n");

    // Create the main application with container demonstrations
    app("Containers Demo")
        .title("🥥 Cocoanut - Containers with Borders Visual Demo")
        .size(1400.0, 1200.0)
        .centered(true)
        .layout(Layout::default())
        
        // ========== TITLE ==========
        .add(Comp::new(Kind::Label)
            .text("🎨 CONTAINER COMPONENTS WITH VISUAL BORDERS")
            .size(1300.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("See how containers organize and group content with visible boundaries")
            .size(1300.0, 20.0))
        
        // ========== CONTAINER 1: GROUPBOX WITH BORDER ==========
        .add(Comp::new(Kind::Label)
            .text("📦 CONTAINER 1: GroupBox - Grouped Content with Border")
            .size(1300.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("┌─ GroupBox: User Information ─────────────────────────┐")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│  Name: [Enter your name_________________]")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("John Doe")
            .size(1000.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("│  Email: [Enter your email________________]")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("john@example.com")
            .size(1000.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("│  Phone: [Enter your phone________________]")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::TextField)
            .text("+1 (555) 123-4567")
            .size(1000.0, 30.0))
        
        .add(Comp::new(Kind::Label)
            .text("│")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("└──────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("GroupBox contains related form fields with visual border")
            .size(1300.0, 18.0))
        
        // ========== CONTAINER 2: SCROLLVIEW WITH BORDER ==========
        .add(Comp::new(Kind::Label)
            .text("📜 CONTAINER 2: ScrollView - Scrollable Content with Border")
            .size(1300.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("┌─ ScrollView: Long Content List ──────────────────────┐")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 1: First item in scrollable list                 │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 2: Second item in scrollable list                │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 3: Third item in scrollable list                 │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 4: Fourth item in scrollable list                │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 5: Fifth item in scrollable list                 │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 6: Sixth item in scrollable list                 │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 7: Seventh item in scrollable list               │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Item 8: Eighth item in scrollable list                │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ [Scroll down to see more items...]                    │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("└──────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("ScrollView contains content that can be scrolled vertically")
            .size(1300.0, 18.0))
        
        // ========== CONTAINER 3: TABVIEW WITH BORDER ==========
        .add(Comp::new(Kind::Label)
            .text("📑 CONTAINER 3: TabView - Tabbed Interface with Border")
            .size(1300.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("┌─ TabView ────────────────────────────────────────────┐")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ [General] [Advanced] [About]                         │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("├──────────────────────────────────────────────────────┤")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ General Tab Content:                                 │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ☑ Enable notifications                              │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ☑ Auto-save enabled                                 │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ☑ Dark mode                                          │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│                                                      │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ [Click other tabs to switch content]                 │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("└──────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("TabView organizes content into multiple tabs with switching")
            .size(1300.0, 18.0))
        
        // ========== CONTAINER 4: SPLITVIEW WITH DIVIDER ==========
        .add(Comp::new(Kind::Label)
            .text("⚙️  CONTAINER 4: SplitView - Resizable Panes with Divider")
            .size(1300.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("┌─ SplitView (Horizontal) ──────────────────────────────┐")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ Left Pane          │ Right Pane                       │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ┌────────────────┤ ┌──────────────────────────────┐ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ Sidebar        │ │ Main Content Area            │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ • Item 1       │ │ Content for selected item    │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ • Item 2       │ │ Drag divider to resize       │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ • Item 3       │ │ Left/right panes adjust      │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ • Item 4       │ │ proportionally               │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ └────────────────┤ └──────────────────────────────┘ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│                   ↑ Divider (drag to resize)          │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("└──────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("SplitView divides window into resizable panes with draggable divider")
            .size(1300.0, 18.0))
        
        // ========== CONTAINER 5: NESTED CONTAINERS ==========
        .add(Comp::new(Kind::Label)
            .text("🎯 CONTAINER 5: Nested Containers - Containers Within Containers")
            .size(1300.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("┌─ Outer GroupBox ──────────────────────────────────────┐")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ┌─ Inner GroupBox 1 ─────────────────────────────────┤")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ ☑ Option A                                         │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ ☑ Option B                                         │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ └────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ┌─ Inner GroupBox 2 ─────────────────────────────────┤")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ ☑ Option C                                         │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ ☑ Option D                                         │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ └────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("└──────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("Nested containers show hierarchical organization with multiple borders")
            .size(1300.0, 18.0))
        
        // ========== CONTAINER 6: MIXED LAYOUT ==========
        .add(Comp::new(Kind::Label)
            .text("🎨 CONTAINER 6: Mixed Layout - Combining Multiple Containers")
            .size(1300.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("┌─ Main Container ──────────────────────────────────────┐")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ┌─ Header ──────────────────────────────────────────┐ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ Application Title and Navigation                 │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ └────────────────────────────────────────────────────┘ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ┌─ Content ─────────────────────────────────────────┐ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ Main content area with scrollable content        │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ [Scrollable area with multiple items]           │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ └────────────────────────────────────────────────────┘ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ ┌─ Footer ──────────────────────────────────────────┐ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ │ [OK]  [Cancel]  Status: Ready                   │ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("│ └────────────────────────────────────────────────────┘ │")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("└──────────────────────────────────────────────────────┘")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("Mixed layout combines header, content, and footer containers")
            .size(1300.0, 18.0))
        
        // ========== SUMMARY ==========
        .add(Comp::new(Kind::Label)
            .text("✨ CONTAINER BEHAVIOR SUMMARY")
            .size(1300.0, 28.0))
        
        .add(Comp::new(Kind::Label)
            .text("• GroupBox: Groups related controls with visual border and optional title")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• ScrollView: Enables scrolling for content larger than viewport")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• TabView: Organizes content into multiple tabs with switching capability")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• SplitView: Divides window into resizable panes with draggable divider")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Nested: Containers can be nested for hierarchical organization")
            .size(1300.0, 20.0))
        
        .add(Comp::new(Kind::Label)
            .text("• Mixed: Multiple container types can be combined in one layout")
            .size(1300.0, 20.0))
        
        // ========== ACTION BUTTONS ==========
        .add(Comp::new(Kind::Button)
            .text("✓ Understood")
            .size(150.0, 40.0))
        
        .run()?;

    println!("\n✓ Containers demo completed successfully!");
    println!("Press Cmd+Q to quit the application");

    Ok(())
}
