//! Menu application example for Cocoanut
//! 
//! This example demonstrates how to create a macOS application with menus.

use cocoanut::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Creating menu application example...");
    
    // Create the application
    let app = Application::new("Menu App Example")?;
    println!("Application created: {}", app.name());
    
    // Create a window
    let window = Window::new("Menu App", 600.0, 400.0)?;
    println!("Window created: {}", window.title());
    
    // Create a menu bar
    let menu_bar = Menu::new("Main Menu")?;
    println!("Menu bar created: {}", menu_bar.title());
    
    // Create File menu
    let file_menu = Menu::new("File")?;
    let new_item = MenuItem::new("New", Some("newDocument:"))?;
    let open_item = MenuItem::new("Open", Some("openDocument:"))?;
    let save_item = MenuItem::new("Save", Some("saveDocument:"))?;
    let separator = MenuItem::separator()?;
    let quit_item = MenuItem::new("Quit", Some("terminate:"))?;
    
    file_menu.add_item(new_item)?;
    file_menu.add_item(open_item)?;
    file_menu.add_item(save_item)?;
    file_menu.add_item(separator)?;
    file_menu.add_item(quit_item)?;
    
    // Create Edit menu
    let edit_menu = Menu::new("Edit")?;
    let cut_item = MenuItem::new("Cut", Some("cut:"))?;
    let copy_item = MenuItem::new("Copy", Some("copy:"))?;
    let paste_item = MenuItem::new("Paste", Some("paste:"))?;
    
    edit_menu.add_item(cut_item)?;
    edit_menu.add_item(copy_item)?;
    edit_menu.add_item(paste_item)?;
    
    // Create Help menu
    let help_menu = Menu::new("Help")?;
    let about_item = MenuItem::new("About", Some("orderFrontStandardAboutPanel:"))?;
    
    help_menu.add_item(about_item)?;
    
    // Note: In a real application, you would set up the menu bar properly
    // For this example, we'll just create the individual menus
    println!("File menu created: {}", file_menu.title());
    println!("Edit menu created: {}", edit_menu.title());
    println!("Help menu created: {}", help_menu.title());
    
    println!("Menus created and configured");
    
    // Show the window
    window.show()?;
    println!("Window shown");
    
    // Run the application
    println!("Running application...");
    app.run(window)?;
    
    println!("Application finished");
    Ok(())
}
