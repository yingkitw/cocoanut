//! Phase 1: Display Elements Demo
//! 
//! Demonstrates all Phase 1 display elements from Streamlit migration:
//! - Text elements (write, text, markdown, title, header, etc.)
//! - Data display (table, metric)
//! - Status messages (success, error, warning, info)

use cocoanut::systems::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Phase 1: Display Elements Demo\n");

    // Text Elements
    println!("📝 Text Elements:");
    
    let write = Write::new("Universal write element")?;
    println!("  ✓ Write: {}", write.render());

    let text = Text::new("Plain text").size(16.0);
    println!("  ✓ Text: {} (size: {})", text.content(), text.font_size());

    let markdown = Markdown::new("# Markdown Title\n\n**Bold text**");
    println!("  ✓ Markdown: {} chars", markdown.content().len());

    let title = Title::new("Page Title");
    println!("  ✓ Title: {}", title.content());

    let header = Header::new("Section Header").level(2);
    println!("  ✓ Header: {} (level {})", header.content(), header.level_value());

    let subheader = Subheader::new("Subsection");
    println!("  ✓ Subheader: {}", subheader.content());

    let caption = Caption::new("Small caption text");
    println!("  ✓ Caption: {}", caption.content());

    let code = Code::new("fn main() { println!(\"Hello\"); }")
        .with_language("rust")
        .line_numbers(true);
    println!("  ✓ Code: {} (lang: {:?})", code.content().len(), code.get_language());

    let json = Json::new(r#"{"name": "Cocoanut", "version": "0.3.0"}"#).expanded(true);
    println!("  ✓ JSON: {} (expanded: {})", json.content().len(), json.is_expanded());

    let help = Help::new("This is the API documentation").with_title("API Reference");
    println!("  ✓ Help: {} (title: {:?})", help.content().len(), help.get_title());

    // Data Display
    println!("\n📊 Data Display:");

    let mut table = Table::new(vec!["Name".to_string(), "Age".to_string(), "City".to_string()]);
    table.add_row(vec!["Alice".to_string(), "30".to_string(), "NYC".to_string()])?;
    table.add_row(vec!["Bob".to_string(), "25".to_string(), "LA".to_string()])?;
    table.add_row(vec!["Charlie".to_string(), "35".to_string(), "Chicago".to_string()])?;
    println!("  ✓ Table: {} rows × {} cols", table.row_count(), table.headers().len());

    let mut dataframe = DataFrame::new(vec!["Product".to_string(), "Sales".to_string()])
        .editable(true)
        .with_height(300.0);
    dataframe.add_row(vec!["Widget A".to_string(), "1000".to_string()])?;
    dataframe.add_row(vec!["Widget B".to_string(), "1500".to_string()])?;
    println!("  ✓ DataFrame: {} rows (editable: {}, height: {:?})", 
        dataframe.rows().len(), dataframe.is_editable(), dataframe.get_height());

    let mut editor = DataEditor::new(vec!["Column1".to_string(), "Column2".to_string()])
        .with_num_rows(15);
    editor.add_row(vec!["Value1".to_string(), "Value2".to_string()])?;
    println!("  ✓ DataEditor: {} rows to display", editor.get_num_rows());

    let metric1 = Metric::new("Revenue", "$10,000")
        .with_delta("+20%")
        .with_delta_color("green");
    let metric2 = Metric::new("Users", "5,000")
        .with_delta("-5%")
        .with_delta_color("red");
    
    let mut metrics = MetricColumn::new();
    metrics.add_metric(metric1);
    metrics.add_metric(metric2);
    println!("  ✓ Metrics: {} metrics in column", metrics.metric_count());

    // Status & Feedback
    println!("\n✅ Status & Feedback:");

    let success = Success::new("Operation completed successfully").icon(true);
    println!("  ✓ Success: {} (icon: {})", success.message(), success.shows_icon());

    let error = Error::new("An error occurred").icon(true);
    println!("  ✓ Error: {}", error.message());

    let warning = Warning::new("Please be careful").icon(true);
    println!("  ✓ Warning: {}", warning.message());

    let info = Info::new("This is informational").icon(true);
    println!("  ✓ Info: {}", info.message());

    let toast = Toast::new("Action completed", StatusType::Success).with_duration(3.0);
    println!("  ✓ Toast: {} ({:?}, {}s)", toast.message(), toast.toast_type(), toast.get_duration());

    let status = Status::new("Processing data").with_state(StatusState::Running);
    println!("  ✓ Status: {} ({:?})", status.label(), status.get_state());

    let progress = Progress::new(75.0, 100.0)?;
    println!("  ✓ Progress: {:.1}% complete", progress.percentage());

    let spinner = Spinner::new().with_text("Loading...");
    println!("  ✓ Spinner: {:?}", spinner.get_text());

    // Summary
    println!("\n📈 Phase 1 Summary:");
    println!("  ✓ Text Elements: 9 types");
    println!("  ✓ Data Display: 4 types");
    println!("  ✓ Status & Feedback: 8 types");
    println!("  ✓ Total: 21 display elements implemented");

    println!("\n✨ Phase 1 Complete!");
    println!("Next: Phase 2 - Input Widgets (v0.4.0)");

    Ok(())
}
