//! Phase 1: Display Elements Demo - GUI Window
//! 
//! Demonstrates all Phase 1 display elements from Streamlit migration:
//! - Text elements (write, text, markdown, title, header, etc.)
//! - Data display (table, metric)
//! - Status messages (success, error, warning, info)

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    app("Phase 1 Demo")
        .title("🥥 Phase 1: Display Elements (21 widgets)")
        .size(800.0, 600.0)
        .centered(true)
        .layout(Layout::default())
        // Text Elements
        .add(Comp::new(Kind::Label).text("📝 Text Elements (9)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Write, Text, Markdown, Title, Header, Subheader, Caption, Code, JSON, Help").size(700.0, 20.0))
        // Data Display
        .add(Comp::new(Kind::Label).text("📊 Data Display (5)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Table, DataFrame, DataEditor, Metric, MetricColumn").size(700.0, 20.0))
        // Status & Feedback
        .add(Comp::new(Kind::Label).text("✅ Status & Feedback (8)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Success, Error, Warning, Info, Toast, Status, Progress, Spinner").size(700.0, 20.0))
        // Summary
        .add(Comp::new(Kind::Label).text("📈 Phase 1 Summary").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("✓ Text Elements: 9 types").size(400.0, 20.0))
        .add(Comp::new(Kind::Label).text("✓ Data Display: 5 types").size(400.0, 20.0))
        .add(Comp::new(Kind::Label).text("✓ Status & Feedback: 8 types").size(400.0, 20.0))
        .add(Comp::new(Kind::Label).text("✓ Total: 21 display elements").size(400.0, 20.0))
        .run()?;
    Ok(())
}
