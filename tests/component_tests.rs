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
