//! Example demonstrating modern Rust patterns and macOS integration features
//! 
//! This example shows how to use async/await, streaming APIs, zero-cost abstractions,
//! and macOS integration features in Cocoanut.

use cocoanut::prelude::*;
use futures::stream::StreamExt;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Cocoanut Modern Features Example");
    println!("=====================================");
    
    // Create macOS integration manager
    let mut integration = MacOSIntegrationManager::new();
    integration.update_from_system()?;
    
    // Demonstrate async/await features
    demonstrate_async_features().await?;
    
    // Demonstrate streaming APIs
    demonstrate_streaming_apis().await?;
    
    // Demonstrate zero-cost abstractions
    demonstrate_zero_cost_abstractions()?;
    
    // Demonstrate macOS integration
    demonstrate_macos_integration(&mut integration)?;
    
    println!("✅ All modern features demonstrated successfully!");
    Ok(())
}

/// Demonstrate async/await features
async fn demonstrate_async_features() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Async/Await Features");
    println!("------------------------");
    
    // Create async UI context
    let context = AsyncUIContext::new();
    let executor = context.executor();
    
    // Create async window
    let window = AsyncWindow::new(executor.clone());
    
    // Perform async operations
    println!("Creating window asynchronously...");
    window.show_async().await?;
    
    println!("Setting window title asynchronously...");
    window.set_title_async("Async Window".to_string()).await?;
    
    // Create async button
    let button = AsyncButton::new(executor);
    
    println!("Setting button title asynchronously...");
    button.set_title_async("Async Button".to_string()).await?;
    
    println!("Enabling button asynchronously...");
    button.set_enabled_async(true).await?;
    
    // Simulate async UI operation
    println!("Hello from async UI operation!");
    let result = 42;
    println!("Async UI operation result: {}", result);
    
    Ok(())
}

/// Demonstrate streaming APIs
async fn demonstrate_streaming_apis() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n📡 Streaming APIs");
    println!("------------------");
    
    // Create reactive components
    let (mut button, mut button_stream) = ReactiveButton::new("demo_button".to_string());
    let (mut text_field, mut text_stream) = ReactiveTextField::new("demo_field".to_string());
    let (mut window, mut window_stream) = ReactiveWindow::new("demo_window".to_string(), 800.0, 600.0);
    
    // Simulate events
    println!("Simulating button click...");
    button.click()?;
    
    println!("Simulating text change...");
    text_field.set_text("Hello, Streams!".to_string())?;
    
    println!("Simulating window resize...");
    window.resize(1024.0, 768.0)?;
    
    // Process events
    let mut event_count = 0;
    while let Some(event) = button_stream.next().await {
        match event {
            UIEvent::ButtonClick { id } => {
                println!("Button clicked: {}", id);
                event_count += 1;
            }
            _ => {}
        }
        if event_count >= 1 {
            break;
        }
    }
    
    while let Some(event) = text_stream.next().await {
        match event {
            UIEvent::TextChanged { id, text } => {
                println!("Text changed in {}: {}", id, text);
                event_count += 1;
            }
            _ => {}
        }
        if event_count >= 2 {
            break;
        }
    }
    
    while let Some(event) = window_stream.next().await {
        match event {
            UIEvent::WindowResize { width, height } => {
                println!("Window resized to {}x{}", width, height);
                event_count += 1;
            }
            _ => {}
        }
        if event_count >= 3 {
            break;
        }
    }
    
    Ok(())
}

/// Demonstrate zero-cost abstractions
fn demonstrate_zero_cost_abstractions() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n⚡ Zero-Cost Abstractions");
    println!("--------------------------");
    
    // Zero-cost point
    let point = ZeroCostPoint::new(10.0, 20.0);
    println!("Point: ({}, {})", point.x(), point.y());
    
    // Zero-cost size
    let size = ZeroCostSize::new(100.0, 200.0);
    println!("Size: {}x{}", size.width(), size.height());
    
    // Zero-cost rectangle
    let rect = ZeroCostRect::from_xywh(10.0, 20.0, 100.0, 200.0);
    println!("Rectangle: ({}, {}) {}x{}", rect.x(), rect.y(), rect.width(), rect.height());
    
    // Zero-cost color
    let color = ZeroCostColor::rgb(1.0, 0.5, 0.0);
    println!("Color: RGB({}, {}, {})", color.red(), color.green(), color.blue());
    
    // Zero-cost array
    let data = [1, 2, 3, 4, 5];
    let array = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    println!("Array length: {}", array.len());
    println!("Array elements: {:?}", array.as_slice());
    
    // Zero-cost array iteration
    let collected: Vec<_> = array.as_slice().iter().collect();
    println!("Iterated elements: {:?}", collected);
    
    // Zero-cost string
    let c_str = b"Hello, Zero-Cost!\0".as_ptr() as *const i8;
    let string = unsafe { ZeroCostString::from_c_str(c_str) };
    println!("Zero-cost string: {}", string.as_str()?);
    
    Ok(())
}

/// Demonstrate macOS integration features
fn demonstrate_macos_integration(integration: &mut MacOSIntegrationManager) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n🍎 macOS Integration");
    println!("--------------------");
    
    // Design language
    println!("Setting design language to Modern...");
    integration.design_language_mut().set_style(DesignStyle::Modern);
    integration.design_language_mut().set_appearance(Appearance::Dark);
    
    println!("Current style: {:?}", integration.design_language().style());
    println!("Current appearance: {:?}", integration.design_language().appearance());
    
    // Accessibility
    println!("Updating accessibility settings...");
    integration.accessibility_mut().update_from_system()?;
    
    println!("VoiceOver enabled: {}", integration.accessibility().is_voice_over_enabled());
    println!("Reduced motion: {}", integration.accessibility().is_reduced_motion_enabled());
    println!("High contrast: {}", integration.accessibility().is_high_contrast_enabled());
    println!("Large text: {}", integration.accessibility().is_large_text_enabled());
    
    // Dark mode
    println!("Setting dark mode to Dark...");
    integration.dark_mode_mut().set_appearance(Appearance::Dark)?;
    
    println!("Current appearance: {:?}", integration.dark_mode().current_appearance());
    println!("System appearance: {:?}", integration.dark_mode().system_appearance());
    
    // Touch Bar
    println!("Adding Touch Bar items...");
    let button_item = TouchBarItem::Button {
        identifier: "demo_button".to_string(),
        title: "Demo".to_string(),
        action: Box::new(|| println!("Touch Bar button pressed!")),
    };
    
    let slider_item = TouchBarItem::Slider {
        identifier: "demo_slider".to_string(),
        value: 0.5,
        min_value: 0.0,
        max_value: 1.0,
        action: Box::new(|value| println!("Slider value: {}", value)),
    };
    
    integration.touch_bar_mut().add_item(button_item)?;
    integration.touch_bar_mut().add_item(slider_item)?;
    
    println!("Touch Bar available: {}", integration.touch_bar().is_available());
    println!("Touch Bar items: {}", integration.touch_bar().items().len());
    
    // Note: Native components would be created here in a real application
    println!("Native components would be created here");
    println!("Integration manager configured successfully");
    
    Ok(())
}

// Note: Clone implementations removed as they cannot be implemented for external types

// TouchBarItem Clone implementation removed as it cannot be implemented for external types
