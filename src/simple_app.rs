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
pub enum Kind {
    /// NSButton control
    Button,
    /// NSTextField used as label (non-editable)
    Label,
    /// NSTextField used as input field (editable)
    TextField,
    /// Checkbox control
    Checkbox,
    /// Radio button control
    Radio,
    /// Slider control
    Slider,
    /// Dropdown/Popup button
    Dropdown,
    /// TextArea (multi-line text)
    TextArea,
}

/// Configurable component with customizable properties
#[derive(Debug, Clone)]
pub struct Comp {
    /// Component type
    pub kind: Kind,
    /// Component title/text
    pub text: String,
    /// Component width
    pub width: f64,
    /// Component height
    pub height: f64,
}

impl Comp {
    /// Create a new component configuration
    pub fn new(kind: Kind) -> Self {
        let (text, width, height) = match kind {
            Kind::Button => ("Click Me!".to_string(), 100.0, 40.0),
            Kind::Label => ("Label".to_string(), 300.0, 30.0),
            Kind::TextField => ("Enter text".to_string(), 300.0, 30.0),
            Kind::Checkbox => ("Checkbox".to_string(), 150.0, 25.0),
            Kind::Radio => ("Radio Option".to_string(), 150.0, 25.0),
            Kind::Slider => ("Slider".to_string(), 250.0, 25.0),
            Kind::Dropdown => ("Select option".to_string(), 180.0, 30.0),
            Kind::TextArea => ("Multi-line text...".to_string(), 400.0, 80.0),
        };
        
        Comp {
            kind,
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

/// Layout configuration inspired by Streamlit
/// Components flow vertically in a single column with automatic spacing
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    /// Top padding from window edge
    pub top_padding: f64,
    /// Left/Right padding (margins)
    pub horizontal_margin: f64,
    /// Vertical spacing between components (gap)
    pub gap: f64,
}

impl Layout {
    /// Create default layout (Streamlit-like)
    /// - 40px top padding
    /// - 20px left/right margins
    /// - 12px gap between components
    pub fn default() -> Self {
        Layout {
            top_padding: 40.0,
            horizontal_margin: 20.0,
            gap: 12.0,
        }
    }

    /// Compact layout (less spacing)
    pub fn compact() -> Self {
        Layout {
            top_padding: 20.0,
            horizontal_margin: 10.0,
            gap: 8.0,
        }
    }

    /// Spacious layout (more spacing)
    pub fn spacious() -> Self {
        Layout {
            top_padding: 60.0,
            horizontal_margin: 40.0,
            gap: 20.0,
        }
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
    components: Vec<Comp>,
    layout: Layout,
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
            layout: Layout::default(),
        }
    }

    /// Set custom layout configuration
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
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
    pub fn add(mut self, comp: Comp) -> Self {
        self.components.push(comp);
        self
    }

    /// Add multiple configured components to the window
    pub fn add_all(mut self, comps: Vec<Comp>) -> Self {
        self.components.extend(comps);
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
                    
                    // Streamlit-like layout: single column, full width (minus margins)
                    let available_width = self.width - (self.layout.horizontal_margin * 2.0);
                    
                    // Calculate available height (with bottom padding)
                    let bottom_padding = 20.0;
                    let available_height = self.height - self.layout.top_padding - bottom_padding;
                    
                    // Start from top with padding
                    // Note: content_view coordinate system has Y=0 at BOTTOM of content view
                    // So to position at top, we use: content_view_height - top_padding
                    // The content view height is window height (title bar is separate)
                    let mut y_position = self.height - self.layout.top_padding;
                    let mut components_added = 0;
                    
                    for comp in &self.components {
                        // Check if component would overflow vertically
                        let next_y = y_position - comp.height - self.layout.gap;
                        if next_y < bottom_padding {
                            println!("  ⚠️  Component \"{}\" would overflow - skipping", comp.text);
                            continue;
                        }
                        
                        let class_name = match comp.kind {
                            Kind::Button | Kind::Checkbox | Kind::Radio => "NSButton",
                            Kind::Label | Kind::TextField => "NSTextField",
                            Kind::Slider => "NSSlider",
                            Kind::Dropdown => "NSPopUpButton",
                            Kind::TextArea => "NSTextView",
                        };
                        
                        let view_class = Class::get(class_name)
                            .ok_or(format!("{} class not found", class_name))?;
                        let view: *mut Object = msg_send![view_class, alloc];
                        
                        // Streamlit approach: components fill available width (with boundary check)
                        let comp_width = if comp.width > available_width {
                            available_width
                        } else {
                            comp.width
                        };
                        
                        // Ensure component doesn't exceed window boundaries
                        let comp_x = self.layout.horizontal_margin;
                        let comp_y = y_position - comp.height;
                        
                        // Clamp height if needed
                        let comp_height = if comp_y < bottom_padding {
                            y_position - bottom_padding
                        } else {
                            comp.height
                        };
                        
                        let frame = NSRect {
                            origin: NSPoint { x: comp_x, y: comp_y },
                            size: NSSize { width: comp_width, height: comp_height },
                        };
                        let view: *mut Object = msg_send![view, initWithFrame:frame];
                        
                        // Configure based on kind
                        match comp.kind {
                            Kind::Button => {
                                let title = std::ffi::CString::new(comp.text.as_str()).unwrap();
                                let ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:title.as_ptr()];
                                let _: () = msg_send![view, setTitle:ns_string];
                                let _: () = msg_send![view, setButtonType:0];
                                let _: () = msg_send![view, setBezelStyle:4]; // Rounded button
                                let _: () = msg_send![view, setEnabled:true];
                            }
                            Kind::Checkbox => {
                                let title = std::ffi::CString::new(comp.text.as_str()).unwrap();
                                let ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:title.as_ptr()];
                                let _: () = msg_send![view, setTitle:ns_string];
                                let _: () = msg_send![view, setButtonType:3];
                            }
                            Kind::Radio => {
                                let title = std::ffi::CString::new(comp.text.as_str()).unwrap();
                                let ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:title.as_ptr()];
                                let _: () = msg_send![view, setTitle:ns_string];
                                let _: () = msg_send![view, setButtonType:4];
                            }
                            Kind::Label => {
                                let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                                let ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                                let _: () = msg_send![view, setStringValue:ns_string];
                                let _: () = msg_send![view, setEditable:false];
                                let _: () = msg_send![view, setBezeled:false];
                                let _: () = msg_send![view, setDrawsBackground:false];
                            }
                            Kind::TextField => {
                                let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                                let ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                                let _: () = msg_send![view, setStringValue:ns_string];
                                let _: () = msg_send![view, setBezeled:true];
                                let _: () = msg_send![view, setDrawsBackground:true];
                                let _: () = msg_send![view, setEditable:true];
                            }
                            Kind::Slider => {
                                let _: () = msg_send![view, setMinValue:0.0];
                                let _: () = msg_send![view, setMaxValue:100.0];
                                let _: () = msg_send![view, setDoubleValue:50.0];
                            }
                            Kind::Dropdown => {
                                // Add placeholder item
                                let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                                let ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                                let _: () = msg_send![view, addItemWithTitle:ns_string];
                                
                                // Add sample choices based on dropdown title
                                let choices: Vec<&str> = if comp.text.contains("theme") {
                                    vec!["Light", "Dark", "Auto"]
                                } else if comp.text.contains("language") {
                                    vec!["English", "Spanish", "French", "German"]
                                } else if comp.text.contains("size") || comp.text.contains("Font") {
                                    vec!["Small", "Medium", "Large", "Extra Large"]
                                } else {
                                    vec!["Option 1", "Option 2", "Option 3"]
                                };
                                
                                for choice in choices {
                                    let choice_cstr = std::ffi::CString::new(choice).unwrap();
                                    let choice_ns: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:choice_cstr.as_ptr()];
                                    let _: () = msg_send![view, addItemWithTitle:choice_ns];
                                }
                            }
                            Kind::TextArea => {
                                // Configure TextArea for multi-line editing
                                let _: () = msg_send![view, setEditable:true];
                                let _: () = msg_send![view, setSelectable:true];
                                
                                // Set text with newlines preserved
                                let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                                let ns_string: *mut Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                                let _: () = msg_send![view, setString:ns_string];
                                
                                // Enable word wrapping
                                let _: () = msg_send![view, setHorizontallyResizable:false];
                                let _: () = msg_send![view, setVerticallyResizable:true];
                                
                                // Set background color to white
                                let white_color: *mut Object = msg_send![objc::class!(NSColor), whiteColor];
                                let _: () = msg_send![view, setBackgroundColor:white_color];
                            }
                        }
                        
                        let _: () = msg_send![content_view, addSubview:view];
                        println!("  ✓ {:?} added: \"{}\" ({}x{})", comp.kind, comp.text, comp_width as i32, comp_height as i32);
                        components_added += 1;
                        y_position -= (comp_height + self.layout.gap);
                    }
                    println!("  ℹ️  {} of {} components displayed (window height: {}px)", 
                        components_added, self.components.len(), self.height as i32);
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

                // Step 6: Configure window to stop app when closed
                let _: () = msg_send![window.ns_window(), setReleasedWhenClosed:true];
                
                // Make close button terminate the app
                let ns_window = window.ns_window();
                let _: () = msg_send![app, setDelegate:ns_window];

                // Step 7: Run event loop
                println!("🚀 Running event loop (close window or press Cmd+Q to quit)...\n");
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
