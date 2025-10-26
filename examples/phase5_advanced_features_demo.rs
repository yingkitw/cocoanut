//! Phase 5: Advanced Features Demo - GUI Window
//! 
//! Demonstrates all Phase 5 advanced feature widgets:
//! - Multi-page navigation
//! - Custom components
//! - Component templates
//! - Component registry

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    app("Phase 5 Demo")
        .title("🥥 Phase 5: Advanced Features (5 widgets)")
        .size(800.0, 600.0)
        .centered(true)
        .layout(Layout::default())
        // Multi-Page Navigation
        .add(Comp::new(Kind::Label).text("📄 Multi-Page Navigation (2)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Navigation, SidebarNav - Multi-page app support").size(700.0, 20.0))
        // Custom Components
        .add(Comp::new(Kind::Label).text("🎨 Custom Components (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("CustomComponent - Reusable component definitions").size(700.0, 20.0))
        // Component Registry
        .add(Comp::new(Kind::Label).text("📦 Component Registry (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("ComponentRegistry - Component lifecycle management").size(700.0, 20.0))
        // Component Templates
        .add(Comp::new(Kind::Label).text("🏗️  Component Templates (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("ComponentTemplate - Template-based component creation").size(700.0, 20.0))
        // Summary
        .add(Comp::new(Kind::Label).text("📈 Phase 5 Summary").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("✓ Total: 5 advanced feature widgets").size(400.0, 20.0))
        .add(Comp::new(Kind::Label).text("✓ Streamlit Migration: 67 total elements").size(400.0, 20.0))
        .run()?;
    Ok(())
}
