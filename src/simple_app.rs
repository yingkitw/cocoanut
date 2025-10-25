//! Simple high-level API for creating macOS apps with minimal boilerplate
//!
//! This module provides a simplified interface for building macOS GUI applications
//! without needing to deal with low-level Objective-C details.

use crate::core::error::Result;
use crate::window::Window;
use crate::layout::{VStack, HStack};
use crate::styling::CarbonColor;

/// Component types that can be added to a window
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    /// NSButton control
    Button,
    /// NSTextField used as label (non-editable)
    Label,
    /// NSTextField used as input field (editable)
    TextField,
}

/// Configurable component with customizable properties
#[derive(Debug, Clone)]
pub struct ComponentConfig {
    /// Component type
    pub component_type: ComponentType,
    /// Component title/text
    pub text: String,
    /// Component width
    pub width: f64,
    /// Component height
    pub height: f64,
}

impl ComponentConfig {
    /// Create a new component configuration
    pub fn new(component_type: ComponentType) -> Self {
        let (text, width, height) = match component_type {
            ComponentType::Button => ("Click Me!".to_string(), 100.0, 40.0),
            ComponentType::Label => ("Label".to_string(), 300.0, 30.0),
            ComponentType::TextField => ("Enter text".to_string(), 300.0, 30.0),
        };
        
        ComponentConfig {
            component_type,
            text,
            width,
            height,
        }
    }

    /// Set the component text/title
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Set the component width
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Set the component height
    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    /// Set the component size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

/// A simple macOS application builder with fluent API
pub struct SimpleApp {
    name: String,
    window: Option<Window>,
    title: String,
    width: f64,
    height: f64,
    centered: bool,
    components: Vec<ComponentConfig>,
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
            components: vec![],
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

    /// Add a configured component to the window
    pub fn add(mut self, config: ComponentConfig) -> Self {
        self.components.push(config);
        self
    }

    /// Add multiple configured components to the window
    pub fn add_all(mut self, configs: Vec<ComponentConfig>) -> Self {
        self.components.extend(configs);
        self
    }

    /// Clear all components
    pub fn clear_components(mut self) -> Self {
        self.components.clear();
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
                    return Err(crate::core::error::CocoanutError::ApplicationInitFailed(
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
                    let title_cstr = std::ffi::CString::new(&self.title[..])
                        .map_err(|e| crate::core::error::CocoanutError::InvalidParameter(e.to_string()))?;
                    let ns_string_class = objc::class!(NSString);
                    let title_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String:title_cstr.as_ptr()];
                    let _: () = msg_send![ns_window, setTitle:title_nsstring];

                    println!("✓ Window title set: {}\n", self.title);

                    // Center if requested
                    if self.centered {
                        let _: () = msg_send![ns_window, center];
                        println!("✓ Window centered\n");
                    }

                    Window::from_ns_window(ns_window)
                };

                // Step 3: Add components to window
                let content_view: *mut Object = msg_send![window.ns_window(), contentView];
                
                if !self.components.is_empty() {
                    println!("Adding {} component(s)...", self.components.len());
                    
                    let mut y_position = 320.0;
                    
                    for config in &self.components {
                        match config.component_type {
                            ComponentType::Button => {
                                let button_class = Class::get("NSButton")
                                    .ok_or("NSButton class not found")?;
                                let button: *mut Object = msg_send![button_class, alloc];
                                
                                let button_frame = NSRect {
                                    origin: NSPoint { x: 20.0, y: y_position },
                                    size: NSSize { width: config.width, height: config.height },
                                };
                                let button: *mut Object = msg_send![button, initWithFrame:button_frame];
                                let button_title = std::ffi::CString::new(config.text.as_str()).unwrap();
                                let button_ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:button_title.as_ptr()];
                                let _: () = msg_send![button, setTitle:button_ns_string];
                                let _: () = msg_send![button, setButtonType:0];
                                
                                let _: () = msg_send![content_view, addSubview:button];
                                println!("  ✓ Button added: \"{}\"", config.text);
                                y_position -= (config.height + 10.0);
                            }
                            ComponentType::Label => {
                                let label_class = Class::get("NSTextField")
                                    .ok_or("NSTextField class not found")?;
                                let label: *mut Object = msg_send![label_class, alloc];
                                
                                let label_frame = NSRect {
                                    origin: NSPoint { x: 20.0, y: y_position },
                                    size: NSSize { width: config.width, height: config.height },
                                };
                                let label: *mut Object = msg_send![label, initWithFrame:label_frame];
                                let label_text = std::ffi::CString::new(config.text.as_str()).unwrap();
                                let label_ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:label_text.as_ptr()];
                                let _: () = msg_send![label, setStringValue:label_ns_string];
                                let _: () = msg_send![label, setEditable:false];
                                let _: () = msg_send![label, setBezeled:false];
                                let _: () = msg_send![label, setDrawsBackground:false];
                                
                                let _: () = msg_send![content_view, addSubview:label];
                                println!("  ✓ Label added: \"{}\"", config.text);
                                y_position -= (config.height + 10.0);
                            }
                            ComponentType::TextField => {
                                let textfield_class = Class::get("NSTextField")
                                    .ok_or("NSTextField class not found")?;
                                let textfield: *mut Object = msg_send![textfield_class, alloc];
                                
                                let textfield_frame = NSRect {
                                    origin: NSPoint { x: 20.0, y: y_position },
                                    size: NSSize { width: config.width, height: config.height },
                                };
                                let textfield: *mut Object = msg_send![textfield, initWithFrame:textfield_frame];
                                let textfield_text = std::ffi::CString::new(config.text.as_str()).unwrap();
                                let textfield_ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:textfield_text.as_ptr()];
                                let _: () = msg_send![textfield, setStringValue:textfield_ns_string];
                                
                                let _: () = msg_send![content_view, addSubview:textfield];
                                println!("  ✓ TextField added: \"{}\"", config.text);
                                y_position -= (config.height + 10.0);
                            }
                        }
                    }
                    println!();
                } else {
                    println!("No components configured\n");
                }

                // Step 4: Display window
                let _: () = msg_send![window.ns_window(), makeKeyAndOrderFront:app];
                println!("✓ Window displayed\n");

                // Step 5: Activate app
                let _: () = msg_send![app, activateIgnoringOtherApps:true];
                println!("✓ Application activated\n");

                // Step 6: Run event loop
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
