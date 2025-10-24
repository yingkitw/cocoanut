//! Comprehensive example demonstrating all GUI components with detailed documentation
//!
//! This example showcases:
//! - Button creation and styling
//! - Label usage and text manipulation
//! - TextField input handling
//! - Layout composition with VStack and HStack
//! - Carbon Design System styling
//! - Best practices for component organization

use cocoanut::prelude::*;

/// Example 1: Creating and Styling Buttons
/// 
/// Demonstrates various button configurations using the builder pattern.
fn example_buttons() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📘 Example 1: Button Components");
    println!("================================");

    // Basic button with minimal configuration
    let basic_button = Button::builder()
        .title("Basic Button")
        .build()?;
    println!("✓ Created basic button: {}", basic_button.title());

    // Button with size configuration
    let sized_button = Button::builder()
        .title("Sized Button")
        .size(150.0, 40.0)
        .build()?;
    println!("✓ Created sized button: {}", sized_button.title());

    // Button with enabled state
    let enabled_button = Button::builder()
        .title("Enabled Button")
        .size(150.0, 40.0)
        .enabled(true)
        .build()?;
    println!("✓ Created enabled button: {}", enabled_button.title());

    // Disabled button
    let disabled_button = Button::builder()
        .title("Disabled Button")
        .size(150.0, 40.0)
        .enabled(false)
        .build()?;
    println!("✓ Created disabled button: {}", disabled_button.title());

    // Button with title change
    let mut changeable_button = Button::builder()
        .title("Original Title")
        .build()?;
    changeable_button.set_title("Updated Title")?;
    println!("✓ Created button with title change: {}", changeable_button.title());

    Ok(())
}

/// Example 2: Creating and Styling Labels
/// 
/// Demonstrates label creation with various text configurations.
fn example_labels() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📗 Example 2: Label Components");
    println!("===============================");

    // Basic label
    let basic_label = Label::builder()
        .text("Basic Label")
        .build()?;
    println!("✓ Created basic label: {}", basic_label.text());

    // Label with size
    let sized_label = Label::builder()
        .text("Sized Label")
        .size(200.0, 30.0)
        .build()?;
    println!("✓ Created sized label: {}", sized_label.text());

    // Long text label
    let long_text = "This is a longer label with more text to demonstrate text handling";
    let long_label = Label::builder()
        .text(long_text)
        .size(400.0, 60.0)
        .build()?;
    println!("✓ Created long text label: {}", long_label.text());

    // Label with text change
    let mut changeable_label = Label::builder()
        .text("Initial Text")
        .build()?;
    changeable_label.set_text("Changed Text")?;
    println!("✓ Created label with text change: {}", changeable_label.text());

    // Empty label
    let empty_label = Label::builder()
        .text("")
        .build()?;
    println!("✓ Created empty label: '{}'", empty_label.text());

    Ok(())
}

/// Example 3: Creating and Configuring Text Fields
/// 
/// Demonstrates text field creation with various input configurations.
fn example_text_fields() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📙 Example 3: TextField Components");
    println!("==================================");

    // Basic text field
    let basic_field = TextField::builder()
        .text("Initial text")
        .build()?;
    println!("✓ Created basic text field: {}", basic_field.text());

    // Text field with placeholder
    let _field_with_placeholder = TextField::builder()
        .text("")
        .placeholder("Enter your name")
        .build()?;
    println!("✓ Created text field with placeholder");

    // Text field with size
    let sized_field = TextField::builder()
        .text("Sized field")
        .size(300.0, 40.0)
        .build()?;
    println!("✓ Created sized text field: {}", sized_field.text());

    // Editable text field
    let editable_field = TextField::builder()
        .text("Editable text")
        .editable(true)
        .size(300.0, 40.0)
        .build()?;
    println!("✓ Created editable text field: {}", editable_field.text());

    // Read-only text field
    let readonly_field = TextField::builder()
        .text("Read-only text")
        .editable(false)
        .size(300.0, 40.0)
        .build()?;
    println!("✓ Created read-only text field: {}", readonly_field.text());

    // Text field with text change
    let mut changeable_field = TextField::builder()
        .text("Original")
        .build()?;
    changeable_field.set_text("Modified")?;
    println!("✓ Created text field with text change: {}", changeable_field.text());

    // Text field with long text
    let long_text = "This is a longer text input that demonstrates text handling in text fields";
    let long_field = TextField::builder()
        .text(long_text)
        .size(400.0, 40.0)
        .build()?;
    println!("✓ Created text field with long text: {}", long_field.text());

    Ok(())
}

/// Example 4: Layout Composition with VStack
/// 
/// Demonstrates vertical stack layout for organizing components.
fn example_vstack_layout() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📕 Example 4: VStack Layout");
    println!("============================");

    // Basic VStack
    let basic_vstack = VStack::new();
    println!("✓ Created basic VStack");
    println!("  - Spacing: {} pt", basic_vstack.get_spacing().value);
    println!("  - Alignment: {:?}", basic_vstack.get_alignment());

    // VStack with standard spacing
    let standard_vstack = VStack::new()
        .spacing(Spacing::standard())
        .alignment(Alignment::Leading);
    println!("✓ Created VStack with standard spacing");
    println!("  - Spacing: {} pt", standard_vstack.get_spacing().value);
    println!("  - Alignment: {:?}", standard_vstack.get_alignment());

    // VStack with relaxed spacing
    let relaxed_vstack = VStack::new()
        .spacing(Spacing::relaxed())
        .alignment(Alignment::Center);
    println!("✓ Created VStack with relaxed spacing");
    println!("  - Spacing: {} pt", relaxed_vstack.get_spacing().value);

    // VStack with compact spacing
    let compact_vstack = VStack::new()
        .spacing(Spacing::compact())
        .alignment(Alignment::Trailing);
    println!("✓ Created VStack with compact spacing");
    println!("  - Spacing: {} pt", compact_vstack.get_spacing().value);

    // VStack with size
    let sized_vstack = VStack::new()
        .spacing(Spacing::standard())
        .alignment(Alignment::Center)
        .size(400.0, 600.0);
    println!("✓ Created VStack with size: {}x{}", 
        sized_vstack.get_width().unwrap_or(0.0),
        sized_vstack.get_height().unwrap_or(0.0)
    );

    Ok(())
}

/// Example 5: Layout Composition with HStack
/// 
/// Demonstrates horizontal stack layout for side-by-side components.
fn example_hstack_layout() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📔 Example 5: HStack Layout");
    println!("============================");

    // Basic HStack
    let basic_hstack = HStack::new();
    println!("✓ Created basic HStack");
    println!("  - Spacing: {} pt", basic_hstack.get_spacing().value);
    println!("  - Alignment: {:?}", basic_hstack.get_alignment());

    // HStack with standard spacing
    let standard_hstack = HStack::new()
        .spacing(Spacing::standard())
        .alignment(Alignment::Center);
    println!("✓ Created HStack with standard spacing");
    println!("  - Spacing: {} pt", standard_hstack.get_spacing().value);

    // HStack with relaxed spacing
    let relaxed_hstack = HStack::new()
        .spacing(Spacing::relaxed())
        .alignment(Alignment::Leading);
    println!("✓ Created HStack with relaxed spacing");
    println!("  - Spacing: {} pt", relaxed_hstack.get_spacing().value);

    // HStack with size
    let sized_hstack = HStack::new()
        .spacing(Spacing::standard())
        .alignment(Alignment::Center)
        .width(800.0);
    println!("✓ Created HStack with width: {}", 
        sized_hstack.get_width().unwrap_or(0.0)
    );

    Ok(())
}

/// Example 6: Spacing Configurations
/// 
/// Demonstrates all available spacing presets.
fn example_spacing() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📓 Example 6: Spacing Presets");
    println!("==============================");

    let spacings = vec![
        ("Compact", Spacing::compact()),
        ("Standard", Spacing::standard()),
        ("Relaxed", Spacing::relaxed()),
    ];

    for (name, spacing) in spacings {
        println!("✓ {}: {} pt", name, spacing.value);
    }

    Ok(())
}

/// Example 7: Alignment Options
/// 
/// Demonstrates different alignment options for layouts.
fn example_alignment() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📒 Example 7: Alignment Options");
    println!("================================");

    let alignments = vec![
        ("Leading", Alignment::Leading),
        ("Center", Alignment::Center),
        ("Trailing", Alignment::Trailing),
    ];

    for (name, alignment) in alignments {
        let vstack = VStack::new().alignment(alignment);
        println!("✓ {}: {:?}", name, vstack.get_alignment());
    }

    Ok(())
}

/// Example 8: Carbon Design System Colors
/// 
/// Demonstrates all available Carbon Design System colors.
fn example_carbon_colors() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 Example 8: Carbon Design System Colors");
    println!("==========================================");

    let colors = vec![
        ("Interactive", CarbonColor::Interactive),
        ("Interactive Hover", CarbonColor::InteractiveHover),
        ("Interactive Active", CarbonColor::InteractiveActive),
        ("UI Background", CarbonColor::UIBackground),
        ("UI Light Background", CarbonColor::UILightBackground),
        ("UI Dark Background", CarbonColor::UIDarkBackground),
        ("Text Primary", CarbonColor::TextPrimary),
        ("Text Secondary", CarbonColor::TextSecondary),
        ("Text Tertiary", CarbonColor::TextTertiary),
        ("Text Disabled", CarbonColor::TextDisabled),
        ("Support Success", CarbonColor::SupportSuccess),
        ("Support Warning", CarbonColor::SupportWarning),
        ("Support Error", CarbonColor::SupportError),
        ("Support Info", CarbonColor::SupportInfo),
    ];

    for (name, color) in colors {
        let (r, g, b) = color.rgb();
        println!("✓ {}: RGB({:.2}, {:.2}, {:.2})", name, r, g, b);
    }

    Ok(())
}

/// Example 9: Typography Scales
/// 
/// Demonstrates all available typography scales.
fn example_typography() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 Example 9: Typography Scales");
    println!("=================================");

    let scales = vec![
        ("Display", TypographyScale::Display),
        ("Heading 1", TypographyScale::Heading1),
        ("Heading 2", TypographyScale::Heading2),
        ("Heading 3", TypographyScale::Heading3),
        ("Body", TypographyScale::Body),
        ("Label", TypographyScale::Label),
        ("Caption", TypographyScale::Caption),
    ];

    for (name, scale) in scales {
        println!("✓ {}: {} pt, weight: {}, line-height: {}x", 
            name,
            scale.font_size(),
            scale.font_weight(),
            scale.line_height_multiplier()
        );
    }

    Ok(())
}

/// Example 10: Component Styling
/// 
/// Demonstrates predefined component styles.
fn example_component_styles() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n🎭 Example 10: Component Styles");
    println!("=================================");

    // Button style
    let button_style = ComponentStyle::button();
    println!("✓ Button Style:");
    println!("  - Background: {:?}", button_style.background);
    println!("  - Text: {:?}", button_style.text);
    println!("  - Typography: {:?}", button_style.typography);
    println!("  - Corner Radius: {:?}", button_style.corner_radius);
    println!("  - Padding: {:?}", button_style.padding);

    // Label style
    let label_style = ComponentStyle::label();
    println!("✓ Label Style:");
    println!("  - Background: {:?}", label_style.background);
    println!("  - Text: {:?}", label_style.text);

    // Text field style
    let text_field_style = ComponentStyle::text_field();
    println!("✓ Text Field Style:");
    println!("  - Background: {:?}", text_field_style.background);
    println!("  - Text: {:?}", text_field_style.text);

    Ok(())
}

/// Example 11: Custom Styled Components
/// 
/// Demonstrates creating custom styled components.
fn example_custom_styles() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n✨ Example 11: Custom Styled Components");
    println!("========================================");

    // Success button
    let success_style = ComponentStyle::button()
        .with_background(CarbonColor::SupportSuccess)
        .with_text(CarbonColor::UIBackground)
        .with_typography(TypographyScale::Label);
    println!("✓ Success Button Style:");
    println!("  - Background: {:?}", success_style.background);
    println!("  - Text: {:?}", success_style.text);

    // Error button
    let error_style = ComponentStyle::button()
        .with_background(CarbonColor::SupportError)
        .with_text(CarbonColor::UIBackground)
        .with_typography(TypographyScale::Label);
    println!("✓ Error Button Style:");
    println!("  - Background: {:?}", error_style.background);

    // Info button
    let info_style = ComponentStyle::button()
        .with_background(CarbonColor::SupportInfo)
        .with_text(CarbonColor::UIBackground)
        .with_typography(TypographyScale::Label);
    println!("✓ Info Button Style:");
    println!("  - Background: {:?}", info_style.background);

    // Large heading label
    let heading_style = ComponentStyle::label()
        .with_typography(TypographyScale::Heading1);
    println!("✓ Heading Label Style:");
    println!("  - Typography: {:?}", heading_style.typography);

    Ok(())
}

/// Example 12: Spacer and Flexible Spacing
/// 
/// Demonstrates spacer usage for flexible layouts.
fn example_spacers() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📏 Example 12: Spacers");
    println!("======================");

    // Flexible spacer
    let flexible_spacer = Spacer::new();
    println!("✓ Created flexible spacer");
    println!("  - Min length: {:?}", flexible_spacer.min_length());

    // Spacer with minimum length
    let min_spacer = Spacer::with_min_length(20.0);
    println!("✓ Created spacer with minimum length");
    println!("  - Min length: {:?}", min_spacer.min_length());

    Ok(())
}

/// Main function running all examples
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Comprehensive GUI Components Example");
    println!("========================================");

    // Run all examples
    example_buttons()?;
    example_labels()?;
    example_text_fields()?;
    example_vstack_layout()?;
    example_hstack_layout()?;
    example_spacing()?;
    example_alignment()?;
    example_carbon_colors()?;
    example_typography()?;
    example_component_styles()?;
    example_custom_styles()?;
    example_spacers()?;

    println!("\n✅ All examples completed successfully!");
    println!("\n📚 Key Takeaways:");
    println!("  • Use builders for all component creation");
    println!("  • Leverage VStack/HStack for layout");
    println!("  • Apply Carbon Design System colors");
    println!("  • Use semantic typography scales");
    println!("  • Customize styles with builder pattern");

    Ok(())
}
