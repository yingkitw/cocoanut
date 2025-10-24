//! Basic window example for Cocoanut
//! 
//! This example demonstrates how to create a simple macOS window using Cocoanut.

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Creating basic window example...");
    
    // Create the application
    let app = Application::new("Basic Window Example")?;
    println!("Application created: {}", app.name());
    
    // Create a window
    let window = Window::new("Hello, Cocoanut!", 800.0, 600.0)?;
    println!("Window created: {}", window.title());
    
    // Show the window
    window.show()?;
    println!("Window shown");
    
    // Run the application
    println!("Running application...");
    app.run(window)?;
    
    println!("Application finished");
    Ok(())
}
