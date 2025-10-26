//! Window V2 - Refactored to use Cacao patterns

use crate::core::{ObjcAccess, Layout};
use crate::utils::ObjcProperty;
use crate::core::error::Result;
use objc::runtime::Object;

/// A macOS window using Cacao patterns
#[derive(Debug, Clone)]
pub struct WindowV2 {
    pub is_handle: bool,
    pub objc: ObjcProperty,
    pub title: String,
    pub width: f64,
    pub height: f64,
}

impl WindowV2 {
    /// Create a new window
    pub fn new(title: &str, width: f64, height: f64) -> Result<Self> {
        Ok(WindowV2 {
            is_handle: false,
            objc: ObjcProperty::retain(std::ptr::null_mut()),
            title: title.to_string(),
            width,
            height,
        })
    }

    /// Create a handle (clone) of this window
    pub fn clone_as_handle(&self) -> WindowV2 {
        WindowV2 {
            is_handle: true,
            objc: self.objc.clone(),
            title: self.title.clone(),
            width: self.width,
            height: self.height,
        }
    }

    /// Set the window title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.title = title.to_string();
        self.objc.with_mut(|_obj| {
            // In real implementation: msg_send![obj, setTitle: ns_string]
        });
        Ok(())
    }

    /// Get the window title
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Set the window size
    pub fn set_size(&mut self, width: f64, height: f64) -> Result<()> {
        self.width = width;
        self.height = height;
        self.objc.with_mut(|_obj| {
            // In real implementation: msg_send![obj, setFrame: frame]
        });
        Ok(())
    }

    /// Get the window size
    pub fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }
}

impl ObjcAccess for WindowV2 {
    fn with_backing_obj_mut<F: Fn(*mut Object)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for WindowV2 {}

impl Drop for WindowV2 {
    fn drop(&mut self) {
        if !self.is_handle {
            // Cleanup only for original
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let window = WindowV2::new("Test", 800.0, 600.0).unwrap();
        assert_eq!(window.get_title(), "Test");
        assert_eq!(window.get_size(), (800.0, 600.0));
        assert!(!window.is_handle);
    }

    #[test]
    fn test_window_clone_as_handle() {
        let window = WindowV2::new("Test", 800.0, 600.0).unwrap();
        let handle = window.clone_as_handle();
        assert!(handle.is_handle);
        assert_eq!(handle.get_title(), "Test");
    }

    #[test]
    fn test_window_set_title() {
        let mut window = WindowV2::new("Test", 800.0, 600.0).unwrap();
        window.set_title("New Title").unwrap();
        assert_eq!(window.get_title(), "New Title");
    }

    #[test]
    fn test_window_set_size() {
        let mut window = WindowV2::new("Test", 800.0, 600.0).unwrap();
        window.set_size(1024.0, 768.0).unwrap();
        assert_eq!(window.get_size(), (1024.0, 768.0));
    }
}
