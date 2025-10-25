//! Phase 3: Advanced Layouts Demo
//! 
//! Demonstrates all Phase 3 advanced layout widgets from Streamlit migration:
//! - Layout containers (columns, tabs, expanders, forms, sidebar)
//! - Advanced layouts (row, column, grid)
//! - Spacing and dividers

use cocoanut::systems::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Phase 3: Advanced Layouts Demo\n");

    // Layout Containers
    println!("📦 Layout Containers:");

    let columns = Columns::new(3)?
        .gap(15.0)
        .weights(vec![1.0, 2.0, 1.0])?;
    println!("  ✓ Columns: num={}, gap={}, weights={:?}", 
        columns.get_num_columns(), columns.get_gap(), columns.get_weights());

    let tabs = Tabs::new(vec![
        "Tab 1".to_string(),
        "Tab 2".to_string(),
        "Tab 3".to_string(),
    ])?
    .active_tab(0)?
    .lazy_load(true);
    println!("  ✓ Tabs: count={}, active={}, lazy_load={}", 
        tabs.get_tab_names().len(), tabs.get_active_tab(), tabs.is_lazy_load());

    let expander = Expander::new("Click to expand")
        .expanded(false)
        .icon("▶");
    println!("  ✓ Expander: label='{}', expanded={}, icon={:?}", 
        expander.get_label(), expander.is_expanded(), expander.get_icon());

    let container = Container::new()
        .border(true)
        .padding(20.0)
        .background_color("#F5F5F5");
    println!("  ✓ Container: border={}, padding={}, bg_color={:?}", 
        container.has_border(), container.get_padding(), container.get_background_color());

    let form = Form::new("contact_form")
        .submit_button("Send Message")
        .clear_on_submit(true);
    println!("  ✓ Form: name='{}', submit='{}', clear={}", 
        form.get_name(), form.get_submit_button(), form.clears_on_submit());

    let sidebar = Sidebar::new()
        .width(280.0)
        .collapsible(true)
        .collapsed(false);
    println!("  ✓ Sidebar: width={}, collapsible={}, collapsed={}", 
        sidebar.get_width(), sidebar.is_collapsible(), sidebar.is_collapsed());

    let empty = Empty::new().height(30.0);
    println!("  ✓ Empty: height={}", empty.get_height());

    // Advanced Layouts
    println!("\n📐 Advanced Layouts:");

    let row = Row::new()
        .gap(12.0)
        .vertical_align(VerticalAlignment::Center)
        .wrap(true);
    println!("  ✓ Row: gap={}, v_align={:?}, wrap={}", 
        row.get_gap(), row.get_vertical_align(), row.wraps());

    let column = Column::new()
        .gap(8.0)
        .horizontal_align(HorizontalAlignment::Stretch)
        .wrap(false);
    println!("  ✓ Column: gap={}, h_align={:?}, wrap={}", 
        column.get_gap(), column.get_horizontal_align(), column.wraps());

    let grid = Grid::new(4, 3)?
        .gap(10.0)
        .column_gap(15.0)
        .row_gap(20.0);
    println!("  ✓ Grid: cols={}, rows={}, col_gap={}, row_gap={}", 
        grid.get_columns(), grid.get_rows(), grid.get_column_gap(), grid.get_row_gap());

    // Spacing and Dividers
    println!("\n✨ Spacing & Dividers:");

    let flex_spacer = FlexSpacer::new()
        .flex(2.0)
        .min_size(10.0)
        .max_size(100.0);
    println!("  ✓ FlexSpacer: flex={}, min={:?}, max={:?}", 
        flex_spacer.get_flex(), flex_spacer.get_min_size(), flex_spacer.get_max_size());

    let h_divider = Divider::new(Orientation::Horizontal)
        .color("#CCCCCC")
        .thickness(1.0);
    println!("  ✓ Divider (H): orientation={:?}, color={:?}, thickness={}", 
        h_divider.get_orientation(), h_divider.get_color(), h_divider.get_thickness());

    let v_divider = Divider::new(Orientation::Vertical)
        .color("#999999")
        .thickness(2.0);
    println!("  ✓ Divider (V): orientation={:?}, color={:?}, thickness={}", 
        v_divider.get_orientation(), v_divider.get_color(), v_divider.get_thickness());

    // Summary
    println!("\n📈 Phase 3 Summary:");
    println!("  ✓ Layout Containers: 7 types");
    println!("    - Columns, Tabs, Expander, Container, Form, Sidebar, Empty");
    println!("  ✓ Advanced Layouts: 4 types");
    println!("    - Row, Column, Grid, Spacer");
    println!("  ✓ Spacing & Dividers: 1 type");
    println!("    - Divider (Horizontal & Vertical)");
    println!("  ✓ Total: 12 layout widgets implemented");

    println!("\n✨ Phase 3 Complete!");
    println!("Next: Phase 4 - State & Caching (v0.6.0)");

    Ok(())
}
