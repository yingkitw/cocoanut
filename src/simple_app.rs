//! Simple high-level API for creating macOS apps with minimal boilerplate
//!
//! This module provides a simplified interface for building macOS GUI applications
//! without needing to deal with low-level Objective-C details.

use crate::error::Result;
use crate::window::Window;
use crate::layout::{VStack, HStack};
use crate::styling::CarbonColor;

/// A simple macOS application builder with fluent API
pub struct SimpleApp {
    name: String,
    window: Option<Window>,
    title: String,
    width: f64,
    height: f64,
    centered: bool,
}

impl SimpleApp {
    /// Create a new simple app
    pub fn new(name: &str) -> Self {
        SimpleApp {
            name: name.to_string(),
            title: name.to_string(),
            window: None,
            width: 800.0,
            height: 600.0,
            centered: true,
        }
    }

    /// Set the window title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the window size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Center the window on screen
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Set the main window
    pub fn with_window(mut self, window: Window) -> Self {
        self.window = Some(window);
        self
    }

    /// Run the application
    pub fn run(self) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            println!("✓ Application initialized: {}", self.name);
            println!("✓ Window: {} ({}x{})", self.title, self.width as i32, self.height as i32);
            if self.centered {
                println!("✓ Window centered");
            }
            if self.window.is_some() {
                println!("✓ Window displayed");
            }
            println!("✓ Event loop running (test-mock mode)");
            return Ok(());
        }

        #[cfg(not(feature = "test-mock"))]
        {
            use objc::runtime::{Class, Object};
            use objc::{msg_send, sel, sel_impl};
            use cocoa::foundation::{NSRect, NSPoint, NSSize};

            unsafe {
                // Step 1: Initialize NSApplication
                let app_class = Class::get("NSApplication")
                    .ok_or("NSApplication class not found")?;
                let app: *mut Object = msg_send![app_class, sharedApplication];
                
                if app.is_null() {
                    return Err(crate::error::CocoanutError::ApplicationInitFailed(
                        "Failed to get NSApplication".to_string()
                    ));
                }

                println!("✓ NSApplication initialized\n");

                // Step 2: Create or use provided window
                let window = if let Some(w) = self.window {
                    w
                } else {
                    // Create default window
                    let window_class = Class::get("NSWindow")
                        .ok_or("NSWindow class not found")?;
                    
                    let frame = NSRect {
                        origin: NSPoint { x: 100.0, y: 100.0 },
                        size: NSSize { width: self.width, height: self.height },
                    };
                    
                    let ns_window: *mut Object = msg_send![window_class, alloc];
                    let ns_window: *mut Object = msg_send![ns_window, initWithContentRect:frame styleMask:15 backing:2 defer:false];
                    
                    if ns_window.is_null() {
                        return Err("Failed to create window".into());
                    }

                    println!("✓ Window created ({}x{})\n", self.width as i32, self.height as i32);

                    // Set title
                    let ns_string_class = Class::get("NSString")
                        .ok_or("NSString class not found")?;
                    let title: *mut Object = msg_send![ns_string_class, stringWithUTF8String:self.title.as_ptr()];
                    let _: () = msg_send![ns_window, setTitle:title];

                    println!("✓ Window title set: {}\n", self.title);

                    // Center if requested
                    if self.centered {
                        let _: () = msg_send![ns_window, center];
                        println!("✓ Window centered\n");
                    }

                    Window::from_ns_window(ns_window)
                };

                // Step 3: Display window
                let _: () = msg_send![window.ns_window(), makeKeyAndOrderFront:app];
                println!("✓ Window displayed\n");

                // Step 4: Activate app
                let _: () = msg_send![app, activateIgnoringOtherApps:true];
                println!("✓ Application activated\n");

                // Step 5: Run event loop
                println!("🚀 Running event loop (press Cmd+Q to quit)...\n");
                let _: () = msg_send![app, run];
            }

            Ok(())
        }
    }
}

/// Quick builder for creating a minimal app in one line
pub fn app(name: &str) -> SimpleApp {
    SimpleApp::new(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_app_creation() {
        let app = SimpleApp::new("Test App");
        assert_eq!(app.name, "Test App");
        assert!(app.window.is_none());
    }

    #[test]
    fn test_simple_app_builder() {
        let app = app("Builder App");
        assert_eq!(app.name, "Builder App");
    }
}
