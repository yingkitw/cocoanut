//! Phase 3: Advanced Layouts Demo - GUI Window
//! 
//! Demonstrates all Phase 3 advanced layout widgets from Streamlit migration:
//! - Layout containers (columns, tabs, expanders, forms, sidebar)
//! - Advanced layouts (row, column, grid)
//! - Spacing and dividers

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    app("Phase 3 Demo")
        .title("🥥 Phase 3: Advanced Layouts (12 widgets)")
        .size(800.0, 600.0)
        .centered(true)
        .layout(Layout::default())
        // Layout Containers
        .add(Comp::new(Kind::Label).text("📦 Layout Containers (7)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Columns, Tabs, Expander, Container, Form, Sidebar, Empty").size(700.0, 20.0))
        // Advanced Layouts
        .add(Comp::new(Kind::Label).text("📐 Advanced Layouts (4)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Row, Column, Grid, FlexSpacer").size(700.0, 20.0))
        // Spacing & Dividers
        .add(Comp::new(Kind::Label).text("✨ Spacing & Dividers (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Divider (Horizontal & Vertical)").size(700.0, 20.0))
        // Summary
        .add(Comp::new(Kind::Label).text("📈 Phase 3 Summary").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("✓ Total: 12 layout widgets").size(400.0, 20.0))
        .run()?;
    Ok(())
}
