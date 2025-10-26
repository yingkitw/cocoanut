//! SimpleApp builder and event loop management

use crate::core::error::Result;
use crate::window::Window;
use super::component::Comp;
use super::layout::Layout;

/// High-level app builder for creating macOS applications with minimal boilerplate
///
/// # Example
///
/// ```ignore
/// app("MyApp")
///     .title("My Application")
///     .size(600.0, 400.0)
///     .centered(true)
///     .add(Comp::new(Kind::Button).text("Click"))
///     .run()?;
/// ```
pub struct SimpleApp {
    /// Application name
    pub name: String,
    /// Window title
    pub title: String,
    /// Window width
    pub width: f64,
    /// Window height
    pub height: f64,
    /// Whether to center window
    pub centered: bool,
    /// Optional custom window
    pub window: Option<Window>,
    /// Layout configuration
    pub layout: Layout,
    /// Components to display
    pub components: Vec<Comp>,
}

impl SimpleApp {
    /// Create a new application
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            title: name.to_string(),
            width: 600.0,
            height: 400.0,
            centered: true,
            window: None,
            layout: Layout::default(),
            components: Vec::new(),
        }
    }

    /// Set window title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set window size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Center window on screen
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Use custom window
    pub fn with_window(mut self, window: Window) -> Self {
        self.window = Some(window);
        self
    }

    /// Set layout configuration
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Add a single component
    pub fn add(mut self, comp: Comp) -> Self {
        self.components.push(comp);
        self
    }

    /// Add multiple components
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
    pub fn run(mut self) -> Result<()> {
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
                let window = if let Some(w) = self.window.take() {
                    w
                } else {
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

                    let title_cstr = std::ffi::CString::new(&self.title[..])
                        .map_err(|e| crate::core::error::CocoanutError::InvalidParameter(e.to_string()))?;
                    let ns_string_class = objc::class!(NSString);
                    let title_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String:title_cstr.as_ptr()];
                    let _: () = msg_send![ns_window, setTitle:title_nsstring];

                    println!("✓ Window title set: {}\n", self.title);

                    if self.centered {
                        let _: () = msg_send![ns_window, center];
                        println!("✓ Window centered\n");
                    }

                    Window::from_ns_window(ns_window)
                };

                // Step 3: Add components to window
                let content_view: *mut Object = msg_send![window.ns_window(), contentView];
                
                if !self.components.is_empty() {
                    self.add_components_to_window(content_view, app)?;
                } else {
                    println!("No components configured\n");
                }

                // Step 4: Display window
                let ns_window = window.ns_window();
                let _: () = msg_send![ns_window, makeKeyAndOrderFront:app];
                println!("✓ Window displayed\n");

                // Step 5: Activate app and bring window to front
                let _: () = msg_send![app, activateIgnoringOtherApps:true];
                println!("✓ Application activated\n");
                
                // Ensure window is on top
                let _: () = msg_send![ns_window, orderFrontRegardless];

                // Step 6: Configure window to stop app when closed
                let _: () = msg_send![ns_window, setReleasedWhenClosed:true];
                
                // Make close button terminate the app
                let _: () = msg_send![app, setDelegate:ns_window];

                // Step 7: Run event loop
                println!("🚀 Running event loop (close window or press Cmd+Q to quit)...\n");
                
                // Small delay to ensure window is rendered before event loop
                std::thread::sleep(std::time::Duration::from_millis(100));
                
                let _: () = msg_send![app, run];
            }

            Ok(())
        }
    }

    #[cfg(not(feature = "test-mock"))]
    fn add_components_to_window(&self, content_view: *mut objc::runtime::Object, app: *mut objc::runtime::Object) -> Result<()> {
        use objc::runtime::{Class, Object};
        use objc::{msg_send, sel, sel_impl};
        use cocoa::foundation::{NSRect, NSPoint, NSSize};

        unsafe {
            println!("Adding {} component(s)...", self.components.len());
            
            let available_width = self.width - (self.layout.horizontal_margin * 2.0);
            let bottom_padding = 20.0;
            let mut y_position = self.height - self.layout.top_padding;
            let mut components_added = 0;
            
            for comp in &self.components {
                let comp_y = y_position - comp.height;
                
                if comp_y < bottom_padding {
                    println!("  ⚠️  Component \"{}\" would overflow - skipping", comp.text);
                    continue;
                }
                
                let class_name = match comp.kind {
                    super::component::Kind::Button | super::component::Kind::Checkbox | super::component::Kind::Radio => "NSButton",
                    super::component::Kind::Label | super::component::Kind::TextField => "NSTextField",
                    super::component::Kind::Slider => "NSSlider",
                    super::component::Kind::Dropdown => "NSPopUpButton",
                    super::component::Kind::TextArea => "NSTextView",
                    super::component::Kind::ScrollView => "NSScrollView",
                    super::component::Kind::TabView => "NSTabView",
                    super::component::Kind::SplitView => "NSSplitView",
                    super::component::Kind::GroupBox => "NSBox",
                };
                
                let view_class = Class::get(class_name)
                    .ok_or(format!("{} class not found", class_name))?;
                let view: *mut Object = msg_send![view_class, alloc];
                
                let comp_width = if comp.width > available_width {
                    available_width
                } else {
                    comp.width
                };
                
                let comp_x = self.layout.horizontal_margin;
                let comp_height = comp.height;
                
                let frame = NSRect {
                    origin: NSPoint { x: comp_x, y: comp_y },
                    size: NSSize { width: comp_width, height: comp_height },
                };
                let view: *mut Object = msg_send![view, initWithFrame:frame];
                
                self.configure_component(view, comp)?;
                
                let _: () = msg_send![content_view, addSubview:view];
                println!("  ✓ {:?} added: \"{}\" ({}x{})", comp.kind, comp.text, comp_width as i32, comp_height as i32);
                components_added += 1;
                y_position -= (comp_height + self.layout.gap);
            }
            println!("  ℹ️  {} of {} components displayed (window height: {}px)", 
                components_added, self.components.len(), self.height as i32);
            println!();
        }
        Ok(())
    }

    #[cfg(not(feature = "test-mock"))]
    fn configure_component(&self, view: *mut objc::runtime::Object, comp: &Comp) -> Result<()> {
        use objc::{msg_send, sel, sel_impl};

        unsafe {
            match comp.kind {
                super::component::Kind::Button => {
                    let title = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:title.as_ptr()];
                    let _: () = msg_send![view, setTitle:ns_string];
                    let _: () = msg_send![view, setButtonType:0];
                    let _: () = msg_send![view, setBezelStyle:4];
                }
                super::component::Kind::Checkbox => {
                    let title = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:title.as_ptr()];
                    let _: () = msg_send![view, setTitle:ns_string];
                    let _: () = msg_send![view, setButtonType:3];
                }
                super::component::Kind::Radio => {
                    let title = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:title.as_ptr()];
                    let _: () = msg_send![view, setTitle:ns_string];
                    let _: () = msg_send![view, setButtonType:4];
                }
                super::component::Kind::Label => {
                    let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                    let _: () = msg_send![view, setStringValue:ns_string];
                    let _: () = msg_send![view, setBezeled:false];
                    let _: () = msg_send![view, setDrawsBackground:false];
                }
                super::component::Kind::TextField => {
                    let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                    let _: () = msg_send![view, setStringValue:ns_string];
                    let _: () = msg_send![view, setBezeled:true];
                    let _: () = msg_send![view, setDrawsBackground:true];
                    let _: () = msg_send![view, setEditable:true];
                }
                super::component::Kind::Slider => {
                    let _: () = msg_send![view, setMinValue:0.0];
                    let _: () = msg_send![view, setMaxValue:100.0];
                    let _: () = msg_send![view, setDoubleValue:50.0];
                }
                super::component::Kind::Dropdown => {
                    let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                    let _: () = msg_send![view, addItemWithTitle:ns_string];
                    
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
                        let choice_ns: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:choice_cstr.as_ptr()];
                        let _: () = msg_send![view, addItemWithTitle:choice_ns];
                    }
                }
                super::component::Kind::TextArea => {
                    let _: () = msg_send![view, setEditable:true];
                    let _: () = msg_send![view, setSelectable:true];
                    
                    let text = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:text.as_ptr()];
                    let _: () = msg_send![view, setString:ns_string];
                    
                    let _: () = msg_send![view, setHorizontallyResizable:false];
                    let _: () = msg_send![view, setVerticallyResizable:true];
                    
                    let white_color: *mut objc::runtime::Object = msg_send![objc::class!(NSColor), whiteColor];
                    let _: () = msg_send![view, setBackgroundColor:white_color];
                }
                super::component::Kind::ScrollView => {
                    let _: () = msg_send![view, setHasVerticalScroller:true];
                    let _: () = msg_send![view, setHasHorizontalScroller:false];
                    let _: () = msg_send![view, setAutohidesScrollers:true];
                    
                    let light_gray: *mut objc::runtime::Object = msg_send![objc::class!(NSColor), lightGrayColor];
                    let _: () = msg_send![view, setBackgroundColor:light_gray];
                }
                super::component::Kind::TabView => {
                    let _: () = msg_send![view, setTabPosition:0]; // NSTopTabsBezelBorder
                    
                    let tab_item_class = objc::class!(NSTabViewItem);
                    let tab1: *mut objc::runtime::Object = msg_send![tab_item_class, alloc];
                    let tab1: *mut objc::runtime::Object = msg_send![tab1, initWithIdentifier:objc::class!(NSString)];
                    let label1 = std::ffi::CString::new("Tab 1").unwrap();
                    let label1_ns: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:label1.as_ptr()];
                    let _: () = msg_send![tab1, setLabel:label1_ns];
                    let _: () = msg_send![view, addTabViewItem:tab1];
                    
                    let tab2: *mut objc::runtime::Object = msg_send![tab_item_class, alloc];
                    let tab2: *mut objc::runtime::Object = msg_send![tab2, initWithIdentifier:objc::class!(NSString)];
                    let label2 = std::ffi::CString::new("Tab 2").unwrap();
                    let label2_ns: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:label2.as_ptr()];
                    let _: () = msg_send![tab2, setLabel:label2_ns];
                    let _: () = msg_send![view, addTabViewItem:tab2];
                }
                super::component::Kind::SplitView => {
                    let _: () = msg_send![view, setVertical:true];
                    let _: () = msg_send![view, setDividerStyle:1]; // NSSplitViewDividerStyleThin
                }
                super::component::Kind::GroupBox => {
                    let title = std::ffi::CString::new(comp.text.as_str()).unwrap();
                    let ns_string: *mut objc::runtime::Object = msg_send![objc::class!(NSString), stringWithUTF8String:title.as_ptr()];
                    let _: () = msg_send![view, setTitle:ns_string];
                    let _: () = msg_send![view, setBorderType:1]; // NSGrooveBorder
                }
            }
        }
        Ok(())
    }
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
        let app = crate::simple_app::app("Builder App");
        assert_eq!(app.name, "Builder App");
    }
}
