//! Window management for macOS GUI applications

use crate::error::{CocoanutError, Result};
use crate::builder::WindowBuilder;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::ffi::CString;

/// A macOS window wrapper
pub struct Window {
    ns_window: *mut Object,
    title: String,
    width: f64,
    height: f64,
}

impl Window {
    /// Create a new window builder for fluent API
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use cocoanut::prelude::*;
    /// 
    /// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    ///     let window = Window::builder()
    ///         .title("My App")
    ///         .size(800.0, 600.0)
    ///         .center()
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    /// Create a Window from an existing NSWindow pointer
    /// 
    /// # Safety
    /// 
    /// This is unsafe because it takes ownership of a raw NSWindow pointer.
    /// The caller must ensure the pointer is valid.
    pub fn from_ns_window(ns_window: *mut Object) -> Self {
        Window {
            ns_window,
            title: String::new(),
            width: 800.0,
            height: 600.0,
        }
    }

    /// Create a new window
    /// 
    /// # Arguments
    /// 
    /// * `title` - The window title
    /// * `width` - Window width in points
    /// * `height` - Window height in points
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Window>` containing the new window instance
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use cocoanut::prelude::*;
    /// 
    /// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    ///     let window = Window::new("My Window", 800.0, 600.0)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(title: &str, width: f64, height: f64) -> Result<Self> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(Window {
                ns_window: std::ptr::null_mut(),
                title: title.to_string(),
                width,
                height,
            });
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};
            
            let window_class = objc::class!(NSWindow);
            
            // Create NSRect as a C struct (not an Objective-C class)
            let frame = NSRect {
                origin: NSPoint { x: 100.0, y: 100.0 },
                size: NSSize { width, height },
            };
            
            // Create window with proper initialization
            let style_mask = 15; // NSWindowStyleMaskTitled | NSWindowStyleMaskClosable | NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskResizable
            
            // Create the window
            let ns_window: *mut Object = msg_send![
                window_class,
                alloc
            ];
            
            let ns_window: *mut Object = msg_send![ns_window, initWithContentRect:frame styleMask:style_mask backing:2 defer:false];
            
            if ns_window.is_null() {
                return Err(CocoanutError::WindowCreationFailed(
                    "Failed to create NSWindow".to_string()
                ));
            }
            
            // Set window title
            let title_cstr = CString::new(title)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            let _: () = msg_send![ns_window, setTitle: title_cstr.as_ptr()];
            
            // Center the window
            let _: () = msg_send![ns_window, center];
            
            Ok(Window {
                ns_window,
                title: title.to_string(),
                width,
                height,
            })
        }
    }
    
    /// Get the window title
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// Set the window title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            self.title = title.to_string();
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let title_cstr = CString::new(title)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            let _: () = msg_send![self.ns_window, setTitle: title_cstr.as_ptr()];
            self.title = title.to_string();
            Ok(())
        }
    }
    
    /// Get window dimensions
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }
    
    /// Set window size
    pub fn set_size(&mut self, width: f64, height: f64) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            self.width = width;
            self.height = height;
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let size_class = objc::class!(NSSize);
            let size: *mut Object = objc::msg_send![size_class, new];
            let _: () = msg_send![size, setWidth: width];
            let _: () = msg_send![size, setHeight: height];
            let _: () = msg_send![self.ns_window, setContentSize: size];
            
            self.width = width;
            self.height = height;
            Ok(())
        }
    }
    
    /// Show the window
    pub fn show(&self) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_window, makeKeyAndOrderFront: self.ns_window];
            Ok(())
        }
    }
    
    /// Hide the window
    pub fn hide(&self) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_window, orderOut: self.ns_window];
            Ok(())
        }
    }
    
    /// Close the window
    pub fn close(&self) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_window, close];
            Ok(())
        }
    }
    
    /// Check if window is visible
    pub fn is_visible(&self) -> bool {
        #[cfg(feature = "test-mock")]
        {
            return false;
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let visible: bool = msg_send![self.ns_window, isVisible];
            visible
        }
    }
    
    /// Center the window on screen
    pub fn center(&mut self) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_window, center];
            Ok(())
        }
    }
    
    /// Get the underlying NSWindow pointer
    pub(crate) fn ns_window(&self) -> *mut Object {
        self.ns_window
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_window, release];
        }
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
