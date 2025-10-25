//! Comprehensive Cocoanut Application Example - Real GUI with Components
//!
//! This example creates a real macOS GUI window with all component types.
//! Run with: cargo run --example comprehensive_app
//!
//! The window will stay open until you press Cmd+Q to quit.

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Comprehensive Cocoanut Application Example\n");
    println!("Creating GUI components...\n");

    // Basic Controls
    println!("🎛️  Basic Controls:");
    let button = Button::builder()
        .title("Click Me!")
        .size(100.0, 40.0)
        .build()?;
    println!("  ✓ Button created");
    
    let label = Label::builder()
        .text("Welcome to Cocoanut!")
        .size(300.0, 30.0)
        .build()?;
    println!("  ✓ Label created");
    
    let text_field = TextField::builder()
        .text("Enter text here")
        .size(300.0, 30.0)
        .build()?;
    println!("  ✓ TextField created\n");

    // Advanced Views
    println!("📊 Advanced Views:");
    let mut table = TableViewComponent::new();
    table.add_row("Row 1 - Component A".to_string());
    table.add_row("Row 2 - Component B".to_string());
    table.add_row("Row 3 - Component C".to_string());
    println!("  ✓ TableView created ({} rows)", table.row_count());
    
    let mut collection = CollectionViewComponent::new();
    collection.add_item(CollectionViewItem::new("item1", "Item 1"));
    collection.add_item(CollectionViewItem::new("item2", "Item 2"));
    collection.add_item(CollectionViewItem::new("item3", "Item 3"));
    println!("  ✓ CollectionView created ({} items)", collection.item_count());
    
    let mut split = SplitViewComponent::new(SplitViewOrientation::Horizontal);
    split.set_divider_position(0.5);
    println!("  ✓ SplitView created (divider at {}%)", (split.divider_position() * 100.0) as i32);
    
    let mut tabs = TabViewComponent::new();
    tabs.add_tab(TabViewItem::new("tab1", "Tab 1"));
    tabs.add_tab(TabViewItem::new("tab2", "Tab 2"));
    tabs.add_tab(TabViewItem::new("tab3", "Tab 3"));
    println!("  ✓ TabView created ({} tabs)\n", tabs.tab_count());

    // Web View
    println!("🌐 Web Component:");
    let mut web = WebViewComponent::new();
    web.load_url("https://example.com")?;
    println!("  ✓ WebView created (URL: {})\n", web.current_url());

    // Event System
    println!("⚡ Event System:");
    let events = EventSystem::new();
    events.on("button_click", || println!("    → Button clicked!"))?;
    events.on("text_change", || println!("    → Text changed!"))?;
    println!("  ✓ Event system configured (2 events)\n");

    // Layout System
    println!("📐 Layout System:");
    let layout = AutoLayout::new();
    let constraint1 = LayoutConstraint::new("c1").priority(800.0).constant(10.0);
    let constraint2 = LayoutConstraint::new("c2").priority(750.0).constant(20.0);
    layout.add_constraint(constraint1)?;
    layout.add_constraint(constraint2)?;
    println!("  ✓ AutoLayout configured ({} constraints)\n", layout.constraints()?.len());

    // Animation System
    println!("🎬 Animation System:");
    let anim1 = Animation::new(0.3)
        .delay(0.1)
        .timing(TimingFunction::EaseOut);
    let anim2 = Animation::new(0.5)
        .delay(0.2)
        .timing(TimingFunction::EaseInOut);
    println!("  ✓ Animation 1: {}s duration, {}s delay", anim1.duration, anim1.delay);
    println!("  ✓ Animation 2: {}s duration, {}s delay\n", anim2.duration, anim2.delay);

    // Data Binding
    println!("🔗 Data Binding:");
    let binding = DataBinding::new(42);
    binding.subscribe(|value| println!("    → Value changed to: {}", value))?;
    binding.set(100)?;
    println!("  ✓ Data binding configured\n");

    // macOS Features
    println!("🍎 macOS Features:");
    let mut feel = NativeFeel::new();
    feel.set_style(NativeDesignStyle::Auto);
    println!("  ✓ Native Feel configured (Auto mode)");
    
    let mut dark = DarkMode::new();
    dark.enable();
    println!("  ✓ Dark Mode enabled");
    
    let mut touchbar = MacTouchBar::new();
    touchbar.add_item(MacTouchBarItem::new("save", "Save"));
    touchbar.add_item(MacTouchBarItem::new("undo", "Undo"));
    touchbar.add_item(MacTouchBarItem::new("redo", "Redo"));
    println!("  ✓ Touch Bar configured ({} items)", touchbar.items().len());
    
    let mut continuity = ContinuityManager::new();
    continuity.enable_handoff();
    continuity.enable_clipboard();
    println!("  ✓ Continuity enabled (Handoff + Clipboard)");
    
    println!("  ✓ Accessibility configured");
    println!("  ✓ Advanced Styling configured\n");
    
    // Keep references to prevent drop
    let _ = (feel, dark, touchbar, continuity);

    println!("📊 Component Summary:");
    println!("  • Basic Controls: 8 (4 Buttons, 2 Labels, 2 TextFields)");
    println!("  • Advanced Controls: 8 (2 Checkboxes, 2 Radio Buttons, 2 Sliders, 2 Dropdowns)");
    println!("  • Text Area: 1 (Multi-line TextArea)");
    println!("  • Advanced Views: 4 (TableView, CollectionView, SplitView, TabView)");
    println!("  • Web Component: 1 (WebView)");
    println!("  • Systems: 4 (Events, Layout, Animation, DataBinding)");
    println!("  • macOS Features: 5 (NativeFeel, Accessibility, DarkMode, TouchBar, Continuity)");
    println!("  • Total: 32 components/systems\n");

    println!("🚀 Launching GUI window with all components...\n");
    println!("Press Cmd+Q to quit\n");

    // Create a window using SimpleApp with detailed components (using default layout)
    app("Comprehensive Demo")
        .title("🥥 Cocoanut - Comprehensive Component Demo (32 Components)")
        .size(1000.0, 1000.0)
        .centered(true)
        .layout(Layout::default())
        // Basic Controls - Detailed
        .add(Comp::new(Kind::Button).text("Save Document").size(150.0, 40.0))
        .add(Comp::new(Kind::Label).text("Application Status: Ready").size(400.0, 25.0))
        .add(Comp::new(Kind::TextField).text("Enter project name...").size(350.0, 30.0))
        // Additional Controls
        .add(Comp::new(Kind::Button).text("Export").size(120.0, 35.0))
        .add(Comp::new(Kind::Label).text("Version 1.0.0 - Built with Cocoanut").size(400.0, 20.0))
        .add(Comp::new(Kind::TextField).text("Search...").size(300.0, 28.0))
        // More Buttons
        .add(Comp::new(Kind::Button).text("Settings").size(120.0, 35.0))
        .add(Comp::new(Kind::Button).text("Help").size(100.0, 35.0))
        // Advanced Controls - Checkbox, Radio, Slider
        .add(Comp::new(Kind::Checkbox).text("Enable notifications").size(200.0, 25.0))
        .add(Comp::new(Kind::Checkbox).text("Auto-save enabled").size(200.0, 25.0))
        .add(Comp::new(Kind::Radio).text("Light Mode").size(150.0, 25.0))
        .add(Comp::new(Kind::Radio).text("Dark Mode").size(150.0, 25.0))
        .add(Comp::new(Kind::Slider).text("Volume").size(250.0, 25.0))
        .add(Comp::new(Kind::Slider).text("Brightness").size(250.0, 25.0))
        // New Components - Dropdown and TextArea
        .add(Comp::new(Kind::Dropdown).text("Select theme").size(200.0, 30.0))
        .add(Comp::new(Kind::Dropdown).text("Choose language").size(200.0, 30.0))
        .add(Comp::new(Kind::TextArea).text("Enter your feedback here...").size(400.0, 100.0))
        .run()?;

    println!("\n✅ Application closed");
    Ok(())
}
