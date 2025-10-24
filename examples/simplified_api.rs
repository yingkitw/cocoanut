//! Example demonstrating the simplified Cocoanut API
//!
//! This example shows how to use builder patterns, layout system, and styling
//! to create UIs more idiomatically than raw objc/cocoa calls.

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Cocoanut Simplified API Example");
    println!("==================================\n");

    // Example 1: Builder patterns for controls
    println!("1. Builder Patterns for Controls");
    println!("---------------------------------");
    
    let button = Button::builder()
        .title("Click Me")
        .size(100.0, 50.0)
        .build()?;
    println!("✓ Created button with builder: {}", button.title());

    let label = Label::builder()
        .text("Hello, Cocoanut!")
        .size(200.0, 30.0)
        .build()?;
    println!("✓ Created label with builder: {}", label.text());

    let text_field = TextField::builder()
        .text("Initial text")
        .placeholder("Enter text here")
        .size(300.0, 40.0)
        .editable(true)
        .build()?;
    println!("✓ Created text field with builder: {}", text_field.text());

    // Example 2: Layout system
    println!("\n2. Layout System (VStack, HStack)");
    println!("----------------------------------");
    
    let vstack = VStack::new()
        .spacing(Spacing::standard())
        .alignment(Alignment::Leading)
        .size(400.0, 600.0);
    println!("✓ Created VStack with spacing: {}", vstack.get_spacing().value);
    println!("  - Alignment: {:?}", vstack.get_alignment());
    println!("  - Size: {}x{}", 
        vstack.get_width().unwrap_or(0.0),
        vstack.get_height().unwrap_or(0.0)
    );

    let hstack = HStack::new()
        .spacing(Spacing::relaxed())
        .alignment(Alignment::Center)
        .width(800.0);
    println!("✓ Created HStack with spacing: {}", hstack.get_spacing().value);
    println!("  - Alignment: {:?}", hstack.get_alignment());

    // Example 3: Spacing presets
    println!("\n3. Spacing Presets");
    println!("------------------");
    println!("✓ Compact spacing: {} pt", Spacing::compact().value);
    println!("✓ Standard spacing: {} pt", Spacing::standard().value);
    println!("✓ Relaxed spacing: {} pt", Spacing::relaxed().value);

    // Example 4: Carbon Design System styling
    println!("\n4. Carbon Design System Styling");
    println!("-------------------------------");
    
    let button_style = ComponentStyle::button();
    println!("✓ Button style:");
    println!("  - Background: {:?}", button_style.background);
    println!("  - Text: {:?}", button_style.text);
    println!("  - Typography: {:?}", button_style.typography);
    println!("  - Corner radius: {:?}", button_style.corner_radius);

    let label_style = ComponentStyle::label();
    println!("✓ Label style:");
    println!("  - Background: {:?}", label_style.background);
    println!("  - Text: {:?}", label_style.text);

    let text_field_style = ComponentStyle::text_field();
    println!("✓ Text field style:");
    println!("  - Background: {:?}", text_field_style.background);

    // Example 5: Carbon colors
    println!("\n5. Carbon Design System Colors");
    println!("-----------------------------");
    
    let colors = vec![
        ("Interactive", CarbonColor::Interactive),
        ("Success", CarbonColor::SupportSuccess),
        ("Warning", CarbonColor::SupportWarning),
        ("Error", CarbonColor::SupportError),
        ("Info", CarbonColor::SupportInfo),
    ];

    for (name, color) in colors {
        let (r, g, b) = color.rgb();
        println!("✓ {}: RGB({:.2}, {:.2}, {:.2})", name, r, g, b);
    }

    // Example 6: Typography scales
    println!("\n6. Typography Scales");
    println!("-------------------");
    
    let scales = vec![
        ("Display", TypographyScale::Display),
        ("Heading 1", TypographyScale::Heading1),
        ("Body", TypographyScale::Body),
        ("Caption", TypographyScale::Caption),
    ];

    for (name, scale) in scales {
        println!("✓ {}: {} pt, weight: {}", 
            name, 
            scale.font_size(), 
            scale.font_weight()
        );
    }

    // Example 7: Spacing scales
    println!("\n7. Spacing Scales");
    println!("-----------------");
    
    let spacings = vec![
        ("Compact", SpacingScale::Compact),
        ("Standard", SpacingScale::Standard),
        ("Spacious", SpacingScale::Spacious),
    ];

    for (name, spacing) in spacings {
        println!("✓ {}: {} pt", name, spacing.value());
    }

    // Example 8: Corner radius scales
    println!("\n8. Corner Radius Scales");
    println!("----------------------");
    
    let radii = vec![
        ("Sharp", CornerRadiusScale::Sharp),
        ("Standard", CornerRadiusScale::Standard),
        ("Pronounced", CornerRadiusScale::Pronounced),
    ];

    for (name, radius) in radii {
        println!("✓ {}: {} pt", name, radius.value());
    }

    // Example 9: Custom styled component
    println!("\n9. Custom Styled Component");
    println!("-------------------------");
    
    let custom_style = ComponentStyle::button()
        .with_background(CarbonColor::SupportSuccess)
        .with_text(CarbonColor::UIBackground)
        .with_typography(TypographyScale::Heading3);
    
    println!("✓ Created custom button style:");
    println!("  - Background: {:?}", custom_style.background);
    println!("  - Text: {:?}", custom_style.text);
    println!("  - Typography: {:?}", custom_style.typography);

    println!("\n✅ All simplified API examples completed successfully!");
    println!("\nKey improvements over raw objc/cocoa:");
    println!("  • Builder patterns for fluent API");
    println!("  • Layout system (VStack, HStack)");
    println!("  • Carbon Design System integration");
    println!("  • Type-safe styling");
    println!("  • Reduced boilerplate");
    println!("  • Better readability");

    Ok(())
}
