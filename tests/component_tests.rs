//! Comprehensive tests for all GUI components
//!
//! Tests cover:
//! - Button creation, configuration, and state changes
//! - Label creation and text manipulation
//! - TextField creation, configuration, and text handling
//! - Layout composition and properties
//! - Styling and design system colors
//! - Spacing and alignment options

use cocoanut::prelude::*;

// ============================================================================
// BUTTON TESTS
// ============================================================================

#[test]
fn test_button_builder_creation() {
    let button = Button::builder()
        .title("Test Button")
        .build();
    
    assert!(button.is_ok());
    let button = button.unwrap();
    assert_eq!(button.title(), "Test Button");
}

#[test]
fn test_button_builder_with_size() {
    let button = Button::builder()
        .title("Sized Button")
        .size(100.0, 50.0)
        .build();
    
    assert!(button.is_ok());
}

#[test]
fn test_button_builder_with_enabled_state() {
    let button = Button::builder()
        .title("Enabled Button")
        .enabled(true)
        .build();
    
    assert!(button.is_ok());
}

#[test]
fn test_button_title_change() {
    let mut button = Button::builder()
        .title("Original")
        .build()
        .unwrap();
    
    assert_eq!(button.title(), "Original");
    
    let result = button.set_title("Updated");
    assert!(result.is_ok());
    assert_eq!(button.title(), "Updated");
}

#[test]
fn test_button_empty_title() {
    let button = Button::builder()
        .title("")
        .build();
    
    assert!(button.is_ok());
    assert_eq!(button.unwrap().title(), "");
}

#[test]
fn test_button_long_title() {
    let long_title = "This is a very long button title that contains many characters";
    let button = Button::builder()
        .title(long_title)
        .build();
    
    assert!(button.is_ok());
    assert_eq!(button.unwrap().title(), long_title);
}

#[test]
fn test_button_multiple_size_configurations() {
    let button1 = Button::builder()
        .title("Small")
        .size(50.0, 25.0)
        .build();
    
    let button2 = Button::builder()
        .title("Medium")
        .size(100.0, 50.0)
        .build();
    
    let button3 = Button::builder()
        .title("Large")
        .size(200.0, 100.0)
        .build();
    
    assert!(button1.is_ok());
    assert!(button2.is_ok());
    assert!(button3.is_ok());
}

// ============================================================================
// LABEL TESTS
// ============================================================================

#[test]
fn test_label_builder_creation() {
    let label = Label::builder()
        .text("Test Label")
        .build();
    
    assert!(label.is_ok());
    let label = label.unwrap();
    assert_eq!(label.text(), "Test Label");
}

#[test]
fn test_label_builder_with_size() {
    let label = Label::builder()
        .text("Sized Label")
        .size(200.0, 30.0)
        .build();
    
    assert!(label.is_ok());
}

#[test]
fn test_label_text_change() {
    let mut label = Label::builder()
        .text("Original")
        .build()
        .unwrap();
    
    assert_eq!(label.text(), "Original");
    
    let result = label.set_text("Updated");
    assert!(result.is_ok());
    assert_eq!(label.text(), "Updated");
}

#[test]
fn test_label_empty_text() {
    let label = Label::builder()
        .text("")
        .build();
    
    assert!(label.is_ok());
    assert_eq!(label.unwrap().text(), "");
}

#[test]
fn test_label_long_text() {
    let long_text = "This is a very long label text that contains many characters and demonstrates text handling in labels";
    let label = Label::builder()
        .text(long_text)
        .build();
    
    assert!(label.is_ok());
    assert_eq!(label.unwrap().text(), long_text);
}

#[test]
fn test_label_multiline_text() {
    let multiline_text = "Line 1\nLine 2\nLine 3";
    let label = Label::builder()
        .text(multiline_text)
        .build();
    
    assert!(label.is_ok());
    assert_eq!(label.unwrap().text(), multiline_text);
}

#[test]
fn test_label_special_characters() {
    let special_text = "Label with special chars: !@#$%^&*()";
    let label = Label::builder()
        .text(special_text)
        .build();
    
    assert!(label.is_ok());
    assert_eq!(label.unwrap().text(), special_text);
}

// ============================================================================
// TEXTFIELD TESTS
// ============================================================================

#[test]
fn test_textfield_builder_creation() {
    let field = TextField::builder()
        .text("Test Text")
        .build();
    
    assert!(field.is_ok());
    let field = field.unwrap();
    assert_eq!(field.text(), "Test Text");
}

#[test]
fn test_textfield_with_placeholder() {
    let field = TextField::builder()
        .text("")
        .placeholder("Enter text")
        .build();
    
    assert!(field.is_ok());
}

#[test]
fn test_textfield_with_size() {
    let field = TextField::builder()
        .text("Sized Field")
        .size(300.0, 40.0)
        .build();
    
    assert!(field.is_ok());
}

#[test]
fn test_textfield_editable() {
    let field = TextField::builder()
        .text("Editable")
        .editable(true)
        .build();
    
    assert!(field.is_ok());
}

#[test]
fn test_textfield_readonly() {
    let field = TextField::builder()
        .text("Read-only")
        .editable(false)
        .build();
    
    assert!(field.is_ok());
}

#[test]
fn test_textfield_text_change() {
    let mut field = TextField::builder()
        .text("Original")
        .build()
        .unwrap();
    
    assert_eq!(field.text(), "Original");
    
    let result = field.set_text("Updated");
    assert!(result.is_ok());
    assert_eq!(field.text(), "Updated");
}

#[test]
fn test_textfield_empty_text() {
    let field = TextField::builder()
        .text("")
        .build();
    
    assert!(field.is_ok());
    assert_eq!(field.unwrap().text(), "");
}

#[test]
fn test_textfield_long_text() {
    let long_text = "This is a very long text field content that contains many characters and demonstrates text handling";
    let field = TextField::builder()
        .text(long_text)
        .build();
    
    assert!(field.is_ok());
    assert_eq!(field.unwrap().text(), long_text);
}

#[test]
fn test_textfield_numeric_text() {
    let numeric_text = "12345.67";
    let field = TextField::builder()
        .text(numeric_text)
        .build();
    
    assert!(field.is_ok());
    assert_eq!(field.unwrap().text(), numeric_text);
}

// ============================================================================
// VSTACK LAYOUT TESTS
// ============================================================================

#[test]
fn test_vstack_creation() {
    let vstack = VStack::new();
    assert_eq!(vstack.get_spacing().value, 8.0); // Standard spacing
    assert_eq!(vstack.get_alignment(), Alignment::Center);
}

#[test]
fn test_vstack_with_spacing() {
    let vstack = VStack::new()
        .spacing(Spacing::relaxed());
    
    assert_eq!(vstack.get_spacing().value, 16.0);
}

#[test]
fn test_vstack_with_alignment() {
    let vstack = VStack::new()
        .alignment(Alignment::Leading);
    
    assert_eq!(vstack.get_alignment(), Alignment::Leading);
}

#[test]
fn test_vstack_with_size() {
    let vstack = VStack::new()
        .size(400.0, 600.0);
    
    assert_eq!(vstack.get_width(), Some(400.0));
    assert_eq!(vstack.get_height(), Some(600.0));
}

#[test]
fn test_vstack_with_width_only() {
    let vstack = VStack::new()
        .width(400.0);
    
    assert_eq!(vstack.get_width(), Some(400.0));
    assert_eq!(vstack.get_height(), None);
}

#[test]
fn test_vstack_with_height_only() {
    let vstack = VStack::new()
        .height(600.0);
    
    assert_eq!(vstack.get_width(), None);
    assert_eq!(vstack.get_height(), Some(600.0));
}

#[test]
fn test_vstack_all_alignments() {
    let alignments = vec![
        Alignment::Leading,
        Alignment::Center,
        Alignment::Trailing,
    ];
    
    for alignment in alignments {
        let vstack = VStack::new().alignment(alignment);
        assert_eq!(vstack.get_alignment(), alignment);
    }
}

#[test]
fn test_vstack_all_spacing_presets() {
    let spacings = vec![
        Spacing::compact(),
        Spacing::standard(),
        Spacing::relaxed(),
    ];
    
    for spacing in spacings {
        let vstack = VStack::new().spacing(spacing);
        assert_eq!(vstack.get_spacing().value, spacing.value);
    }
}

// ============================================================================
// HSTACK LAYOUT TESTS
// ============================================================================

#[test]
fn test_hstack_creation() {
    let hstack = HStack::new();
    assert_eq!(hstack.get_spacing().value, 8.0); // Standard spacing
    assert_eq!(hstack.get_alignment(), Alignment::Center);
}

#[test]
fn test_hstack_with_spacing() {
    let hstack = HStack::new()
        .spacing(Spacing::relaxed());
    
    assert_eq!(hstack.get_spacing().value, 16.0);
}

#[test]
fn test_hstack_with_alignment() {
    let hstack = HStack::new()
        .alignment(Alignment::Leading);
    
    assert_eq!(hstack.get_alignment(), Alignment::Leading);
}

#[test]
fn test_hstack_with_width() {
    let hstack = HStack::new()
        .width(800.0);
    
    assert_eq!(hstack.get_width(), Some(800.0));
}

#[test]
fn test_hstack_all_alignments() {
    let alignments = vec![
        Alignment::Leading,
        Alignment::Center,
        Alignment::Trailing,
    ];
    
    for alignment in alignments {
        let hstack = HStack::new().alignment(alignment);
        assert_eq!(hstack.get_alignment(), alignment);
    }
}

// ============================================================================
// SPACING TESTS
// ============================================================================

#[test]
fn test_spacing_compact() {
    let spacing = Spacing::compact();
    assert_eq!(spacing.value, 4.0);
}

#[test]
fn test_spacing_standard() {
    let spacing = Spacing::standard();
    assert_eq!(spacing.value, 8.0);
}

#[test]
fn test_spacing_relaxed() {
    let spacing = Spacing::relaxed();
    assert_eq!(spacing.value, 16.0);
}

#[test]
fn test_spacing_custom() {
    let spacing = Spacing::new(12.0);
    assert_eq!(spacing.value, 12.0);
}

// ============================================================================
// STYLING TESTS
// ============================================================================

#[test]
fn test_carbon_color_interactive() {
    let (r, g, b) = CarbonColor::Interactive.rgb();
    assert!(r >= 0.0 && r <= 1.0);
    assert!(g >= 0.0 && g <= 1.0);
    assert!(b >= 0.0 && b <= 1.0);
}

#[test]
fn test_carbon_color_support_success() {
    let (r, g, b) = CarbonColor::SupportSuccess.rgb();
    assert!(r >= 0.0 && r <= 1.0);
    assert!(g >= 0.0 && g <= 1.0);
    assert!(b >= 0.0 && b <= 1.0);
}

#[test]
fn test_carbon_color_support_error() {
    let (r, g, b) = CarbonColor::SupportError.rgb();
    assert!(r >= 0.0 && r <= 1.0);
    assert!(g >= 0.0 && g <= 1.0);
    assert!(b >= 0.0 && b <= 1.0);
}

#[test]
fn test_typography_display() {
    let scale = TypographyScale::Display;
    assert_eq!(scale.font_size(), 32.0);
    assert_eq!(scale.font_weight(), 1.0);
}

#[test]
fn test_typography_body() {
    let scale = TypographyScale::Body;
    assert_eq!(scale.font_size(), 16.0);
    assert_eq!(scale.font_weight(), 0.5);
}

#[test]
fn test_typography_caption() {
    let scale = TypographyScale::Caption;
    assert_eq!(scale.font_size(), 12.0);
    assert_eq!(scale.font_weight(), 0.5);
}

#[test]
fn test_component_style_button() {
    let style = ComponentStyle::button();
    assert_eq!(style.background, CarbonColor::Interactive);
    assert_eq!(style.text, CarbonColor::UIBackground);
    assert_eq!(style.typography, TypographyScale::Label);
}

#[test]
fn test_component_style_label() {
    let style = ComponentStyle::label();
    assert_eq!(style.background, CarbonColor::UIBackground);
    assert_eq!(style.text, CarbonColor::TextPrimary);
}

#[test]
fn test_component_style_text_field() {
    let style = ComponentStyle::text_field();
    assert_eq!(style.background, CarbonColor::UILightBackground);
    assert_eq!(style.text, CarbonColor::TextPrimary);
}

#[test]
fn test_component_style_custom() {
    let style = ComponentStyle::button()
        .with_background(CarbonColor::SupportSuccess)
        .with_text(CarbonColor::UIBackground)
        .with_typography(TypographyScale::Heading1);
    
    assert_eq!(style.background, CarbonColor::SupportSuccess);
    assert_eq!(style.text, CarbonColor::UIBackground);
    assert_eq!(style.typography, TypographyScale::Heading1);
}

// ============================================================================
// SPACER TESTS
// ============================================================================

#[test]
fn test_spacer_flexible() {
    let spacer = Spacer::new();
    assert_eq!(spacer.min_length(), None);
}

#[test]
fn test_spacer_with_min_length() {
    let spacer = Spacer::with_min_length(20.0);
    assert_eq!(spacer.min_length(), Some(20.0));
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_multiple_buttons_creation() {
    let buttons: std::result::Result<Vec<_>, _> = (0..5)
        .map(|i| Button::builder().title(&format!("Button {}", i)).build())
        .collect();
    
    assert!(buttons.is_ok());
    assert_eq!(buttons.unwrap().len(), 5);
}

#[test]
fn test_multiple_labels_creation() {
    let labels: std::result::Result<Vec<_>, _> = (0..5)
        .map(|i| Label::builder().text(&format!("Label {}", i)).build())
        .collect();
    
    assert!(labels.is_ok());
    assert_eq!(labels.unwrap().len(), 5);
}

#[test]
fn test_multiple_textfields_creation() {
    let fields: std::result::Result<Vec<_>, _> = (0..5)
        .map(|i| TextField::builder().text(&format!("Field {}", i)).build())
        .collect();
    
    assert!(fields.is_ok());
    assert_eq!(fields.unwrap().len(), 5);
}

#[test]
fn test_mixed_components_creation() {
    let button = Button::builder().title("Button").build();
    let label = Label::builder().text("Label").build();
    let field = TextField::builder().text("Field").build();
    
    assert!(button.is_ok());
    assert!(label.is_ok());
    assert!(field.is_ok());
}

#[test]
fn test_layout_with_components() {
    let vstack = VStack::new()
        .spacing(Spacing::standard())
        .alignment(Alignment::Center)
        .size(400.0, 600.0);
    
    let button = Button::builder().title("Button").build();
    let label = Label::builder().text("Label").build();
    
    assert!(button.is_ok());
    assert!(label.is_ok());
    assert_eq!(vstack.get_spacing().value, 8.0);
}

// ============================================================================
// PHASE 2: BASIC CONTROLS TESTS
// ============================================================================

#[test]
fn test_checkbox_builder_creation() {
    let checkbox = Checkbox::builder()
        .label("Accept Terms")
        .checked(true)
        .build();
    
    assert!(checkbox.is_ok());
    let cb = checkbox.unwrap();
    assert_eq!(cb.label(), "Accept Terms");
    assert!(cb.is_checked());
}

#[test]
fn test_checkbox_unchecked() {
    let checkbox = Checkbox::builder()
        .label("Unchecked")
        .build();
    
    assert!(checkbox.is_ok());
    assert!(!checkbox.unwrap().is_checked());
}

#[test]
fn test_checkbox_state_change() {
    let mut checkbox = Checkbox::new("Test").unwrap();
    assert!(!checkbox.is_checked());
    
    checkbox.set_checked(true).unwrap();
    assert!(checkbox.is_checked());
    
    checkbox.set_checked(false).unwrap();
    assert!(!checkbox.is_checked());
}

#[test]
fn test_radio_button_builder() {
    let radio = RadioButton::builder()
        .label("Option 1")
        .group_id("group1")
        .selected(true)
        .build();
    
    assert!(radio.is_ok());
    let rb = radio.unwrap();
    assert_eq!(rb.label(), "Option 1");
    assert_eq!(rb.group_id(), "group1");
    assert!(rb.is_selected());
}

#[test]
fn test_radio_button_group_management() {
    let radio1 = RadioButton::new("Option 1", "group1").unwrap();
    let radio2 = RadioButton::new("Option 2", "group1").unwrap();
    
    assert_eq!(radio1.group_id(), radio2.group_id());
    assert!(!radio1.is_selected());
    assert!(!radio2.is_selected());
}

#[test]
fn test_slider_builder() {
    let slider = Slider::builder()
        .min(0.0)
        .max(100.0)
        .value(50.0)
        .build();
    
    assert!(slider.is_ok());
    let s = slider.unwrap();
    assert_eq!(s.min_value(), 0.0);
    assert_eq!(s.max_value(), 100.0);
    assert_eq!(s.current_value(), 50.0);
}

#[test]
fn test_slider_value_validation() {
    let mut slider = Slider::new(0.0, 100.0).unwrap();
    
    assert!(slider.set_value(50.0).is_ok());
    assert_eq!(slider.current_value(), 50.0);
    
    assert!(slider.set_value(150.0).is_err());
    assert_eq!(slider.current_value(), 50.0);
}

#[test]
fn test_segmented_control_builder() {
    let control = cocoanut::prelude::SegmentedControl::builder()
        .segment("Option A")
        .segment("Option B")
        .segment("Option C")
        .build();
    
    assert!(control.is_ok());
    let sc = control.unwrap();
    assert_eq!(sc.segments().len(), 3);
}

#[test]
fn test_stepper_builder() {
    let stepper = cocoanut::prelude::Stepper::builder()
        .min(0)
        .max(10)
        .value(5)
        .build();
    
    assert!(stepper.is_ok());
    let s = stepper.unwrap();
    assert_eq!(s.value(), 5);
}

#[test]
fn test_stepper_increment_decrement() {
    let mut stepper = cocoanut::prelude::Stepper::new(0, 10).unwrap();
    
    assert_eq!(stepper.value(), 0);
    stepper.increment().unwrap();
    assert_eq!(stepper.value(), 1);
    
    stepper.decrement().unwrap();
    assert_eq!(stepper.value(), 0);
}

#[test]
fn test_switch_builder() {
    let switch = cocoanut::prelude::Switch::builder()
        .label("WiFi")
        .enabled(true)
        .build();
    
    assert!(switch.is_ok());
    let sw = switch.unwrap();
    assert_eq!(sw.label(), "WiFi");
    assert!(sw.is_enabled());
}

#[test]
fn test_switch_toggle() {
    let mut switch = cocoanut::prelude::Switch::new("Bluetooth").unwrap();
    
    assert!(!switch.is_enabled());
    switch.set_enabled(true).unwrap();
    assert!(switch.is_enabled());
}

// ============================================================================
// PHASE 2: CONTAINER VIEWS TESTS
// ============================================================================

#[test]
fn test_scroll_view_builder() {
    let scroll = cocoanut::prelude::ScrollView::builder()
        .size(400.0, 300.0)
        .content_size(400.0, 800.0)
        .build();
    
    assert!(scroll.is_ok());
    let sv = scroll.unwrap();
    assert_eq!(sv.size(), (400.0, 300.0));
    assert_eq!(sv.scrollable_size(), (400.0, 800.0));
}

#[test]
fn test_tab_view_builder() {
    let tabs = cocoanut::prelude::TabView::builder()
        .tab("General")
        .tab("Advanced")
        .tab("About")
        .build();
    
    assert!(tabs.is_ok());
    let tv = tabs.unwrap();
    assert_eq!(tv.tabs().len(), 3);
}

#[test]
fn test_tab_view_selection() {
    let mut tabs = cocoanut::prelude::TabView::new(vec![
        "Tab 1".to_string(),
        "Tab 2".to_string(),
    ]).unwrap();
    
    assert_eq!(tabs.selected_tab(), 0);
    tabs.set_selected_tab(1).unwrap();
    assert_eq!(tabs.selected_tab(), 1);
}

#[test]
fn test_split_view_builder() {
    let split = cocoanut::prelude::SplitView::builder()
        .orientation(cocoanut::prelude::SplitOrientation::Vertical)
        .divider_position(0.5)
        .build();
    
    assert!(split.is_ok());
    let sv = split.unwrap();
    assert_eq!(sv.orientation(), cocoanut::prelude::SplitOrientation::Vertical);
}

#[test]
fn test_split_view_divider_position() {
    let mut split = cocoanut::prelude::SplitView::new(
        cocoanut::prelude::SplitOrientation::Horizontal
    ).unwrap();
    
    assert!(split.set_divider_position(0.3).is_ok());
    assert_eq!(split.divider_position(), 0.3);
    
    assert!(split.set_divider_position(1.5).is_err());
}

#[test]
fn test_group_box_builder() {
    let group = cocoanut::prelude::GroupBox::builder()
        .title("Settings")
        .build();
    
    assert!(group.is_ok());
    assert_eq!(group.unwrap().title(), "Settings");
}

// ============================================================================
// PHASE 3: DATA DISPLAY TESTS
// ============================================================================

#[test]
fn test_table_view_builder() {
    let table = cocoanut::prelude::TableView::builder()
        .column("Name")
        .column("Email")
        .column("Phone")
        .build();
    
    assert!(table.is_ok());
    let tv = table.unwrap();
    assert_eq!(tv.columns().len(), 3);
}

#[test]
fn test_table_view_add_row() {
    let mut table = cocoanut::prelude::TableView::new(vec![
        "Name".to_string(),
        "Age".to_string(),
    ]).unwrap();
    
    assert_eq!(table.row_count(), 0);
    table.add_row(vec!["Alice".to_string(), "30".to_string()]).unwrap();
    assert_eq!(table.row_count(), 1);
}

#[test]
fn test_table_view_multiple_rows() {
    let mut table = cocoanut::prelude::TableView::new(vec![
        "A".to_string(),
        "B".to_string(),
    ]).unwrap();
    
    for i in 0..5 {
        table.add_row(vec![i.to_string(), (i * 2).to_string()]).unwrap();
    }
    
    assert_eq!(table.row_count(), 5);
}

#[test]
fn test_outline_view_builder() {
    let item1 = cocoanut::prelude::OutlineItem::new("Item 1");
    let item2 = cocoanut::prelude::OutlineItem::new("Item 2");
    
    let view = cocoanut::prelude::OutlineView::builder()
        .item(item1)
        .item(item2)
        .build();
    
    assert!(view.is_ok());
    assert_eq!(view.unwrap().items().len(), 2);
}

#[test]
fn test_outline_item_hierarchy() {
    let mut parent = cocoanut::prelude::OutlineItem::new("Parent");
    let child1 = cocoanut::prelude::OutlineItem::new("Child 1");
    let child2 = cocoanut::prelude::OutlineItem::new("Child 2");
    
    parent.add_child(child1);
    parent.add_child(child2);
    
    assert_eq!(parent.children().len(), 2);
}

#[test]
fn test_collection_view_builder() {
    let view = cocoanut::prelude::CollectionView::builder()
        .columns(3)
        .item("Item 1")
        .item("Item 2")
        .item("Item 3")
        .build();
    
    assert!(view.is_ok());
    let cv = view.unwrap();
    assert_eq!(cv.columns(), 3);
    assert_eq!(cv.item_count(), 3);
}

#[test]
fn test_collection_view_many_items() {
    let mut view = cocoanut::prelude::CollectionView::new(4).unwrap();
    
    for i in 0..20 {
        view.add_item(format!("Item {}", i));
    }
    
    assert_eq!(view.item_count(), 20);
}

// ============================================================================
// PHASE 3: MACOS FEATURES TESTS
// ============================================================================

#[test]
fn test_grid_view_builder() {
    let grid = cocoanut::prelude::GridView::builder()
        .columns(3)
        .rows(4)
        .spacing(8.0)
        .build();
    
    assert!(grid.is_ok());
    let gv = grid.unwrap();
    assert_eq!(gv.columns(), 3);
    assert_eq!(gv.rows(), 4);
}

#[test]
fn test_touch_bar_builder() {
    let bar = cocoanut::prelude::TouchBarFeatureBuilder::new()
        .item(cocoanut::prelude::TouchBarFeatureItem::new("id1", "Save"))
        .item(cocoanut::prelude::TouchBarFeatureItem::new("id2", "Cancel"))
        .build();
    
    assert!(bar.is_ok());
    assert_eq!(bar.unwrap().items().len(), 2);
}

#[test]
fn test_accessibility_builder() {
    let acc = cocoanut::prelude::AccessibilityBuilder::new()
        .label("Submit Button")
        .help_text("Click to submit the form")
        .enabled(true)
        .build();
    
    assert!(acc.is_ok());
    let a = acc.unwrap();
    assert_eq!(a.label(), "Submit Button");
    assert!(a.help_text().is_some());
}

#[test]
fn test_dark_mode_manager() {
    let dm = cocoanut::prelude::DarkModeFeature::new(
        cocoanut::prelude::AppearanceMode::Light
    ).unwrap();
    
    assert!(!dm.is_dark());
}

#[test]
fn test_dark_mode_toggle() {
    let mut dm = cocoanut::prelude::DarkModeFeature::new(
        cocoanut::prelude::AppearanceMode::Light
    ).unwrap();
    
    assert!(!dm.is_dark());
    dm.set_appearance(cocoanut::prelude::AppearanceMode::Dark).unwrap();
    assert!(dm.is_dark());
}

#[test]
fn test_drag_drop_manager() {
    let mut dd = cocoanut::prelude::DragDropManager::new().unwrap();
    
    assert!(!dd.is_enabled());
    dd.enable().unwrap();
    assert!(dd.is_enabled());
}

#[test]
fn test_drag_drop_allowed_types() {
    let mut dd = cocoanut::prelude::DragDropManager::new().unwrap();
    
    dd.add_allowed_type("text");
    dd.add_allowed_type("image");
    dd.add_allowed_type("file");
    
    assert_eq!(dd.allowed_types().len(), 3);
}

#[test]
fn test_advanced_styling_builder() {
    let style = cocoanut::prelude::AdvancedStylingBuilder::new()
        .corner_radius(8.0)
        .shadow(true)
        .shadow_opacity(0.7)
        .border_width(1.0)
        .build();
    
    assert!(style.is_ok());
    let s = style.unwrap();
    assert_eq!(s.corner_radius(), 8.0);
    assert!(s.shadow_enabled());
    assert_eq!(s.shadow_opacity(), 0.7);
    assert_eq!(s.border_width(), 1.0);
}

// ============================================================================
// WINDOW BUILDER TESTS
// ============================================================================

#[test]
fn test_window_builder_basic() {
    let window = Window::builder()
        .title("Test Window")
        .size(800.0, 600.0)
        .build();
    
    assert!(window.is_ok());
}

#[test]
fn test_window_builder_centered() {
    let window = Window::builder()
        .title("Centered")
        .size(640.0, 480.0)
        .center()
        .build();
    
    assert!(window.is_ok());
}

#[test]
fn test_window_builder_resizable() {
    let window = Window::builder()
        .title("Resizable")
        .resizable(true)
        .minimizable(true)
        .closable(true)
        .build();
    
    assert!(window.is_ok());
}

// ============================================================================
// EVENT BINDING TESTS
// ============================================================================

#[test]
fn test_button_with_on_click() {
    use std::sync::{Arc, Mutex};
    
    let clicked = Arc::new(Mutex::new(false));
    let clicked_clone = clicked.clone();
    
    let button = Button::builder()
        .title("Click Me")
        .on_click(move || {
            *clicked_clone.lock().unwrap() = true;
        })
        .build();
    
    assert!(button.is_ok());
}

#[test]
fn test_text_field_with_on_change() {
    use std::sync::{Arc, Mutex};
    
    let text_value = Arc::new(Mutex::new(String::new()));
    let text_clone = text_value.clone();
    
    let field = TextField::builder()
        .on_change(move |text| {
            *text_clone.lock().unwrap() = text;
        })
        .build();
    
    assert!(field.is_ok());
}
