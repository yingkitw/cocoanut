//! Application management for macOS GUI applications

use crate::core::error::{CocoanutError, Result};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::ffi::CString;

/// Main application class for managing the macOS application lifecycle
pub struct Application {
    app: *mut Object,
    name: String,
}

impl Application {
    /// Create a new application instance
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the application
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Application>` containing the new application instance
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use cocoanut::prelude::*;
    /// 
    /// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    ///     let app = Application::new("My App")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(name: &str) -> Result<Self> {
        unsafe {
            let app_class = objc::class!(NSApplication);
            let app: *mut Object = msg_send![app_class, sharedApplication];
            
            if app.is_null() {
                return Err(CocoanutError::ApplicationInitFailed(
                    "Failed to get shared application".to_string()
                ));
            }
            
            // Set the application name
            let name_cstr = CString::new(name)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            let ns_string_class = objc::class!(NSString);
            let name_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String:name_cstr.as_ptr()];
            let _: () = msg_send![app, setApplicationName: name_nsstring];
            
            Ok(Application {
                app,
                name: name.to_string(),
            })
        }
    }
    
    /// Get the application name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Run the application with the main window
    /// 
    /// # Arguments
    /// 
    /// * `window` - The main window to display
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<()>` indicating success or failure
    pub fn run(&self, window: crate::window::Window) -> Result<()> {
        unsafe {
            // Make the window key and order front
            let _: () = msg_send![window.ns_window(), makeKeyAndOrderFront: self.app];
            
            // Run the application
            let _: () = msg_send![self.app, run];
            
            Ok(())
        }
    }
    
    /// Terminate the application
    pub fn terminate(&self) -> Result<()> {
        unsafe {
            let _: () = msg_send![self.app, terminate: self.app];
            Ok(())
        }
    }
    
    /// Check if the application is running
    pub fn is_running(&self) -> bool {
        unsafe {
            let running: bool = msg_send![self.app, isRunning];
            running
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // Application cleanup is handled by the system
    }
}

unsafe impl Send for Application {}
unsafe impl Sync for Application {}
