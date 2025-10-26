//! Button V2 - Refactored to use Cacao patterns

use crate::core::{ObjcAccess, Layout};
use crate::utils::ObjcProperty;
use crate::core::error::Result;
use objc::runtime::Object;

/// A macOS button control using Cacao patterns
#[derive(Debug, Clone)]
pub struct ButtonV2 {
    pub is_handle: bool,
    pub objc: ObjcProperty,
    pub title: String,
}

impl ButtonV2 {
    /// Create a new button
    pub fn new(title: &str) -> Result<Self> {
        Ok(ButtonV2 {
            is_handle: false,
            objc: ObjcProperty::retain(std::ptr::null_mut()),
            title: title.to_string(),
        })
    }

    /// Create a handle (clone) of this button
    pub fn clone_as_handle(&self) -> ButtonV2 {
        ButtonV2 {
            is_handle: true,
            objc: self.objc.clone(),
            title: self.title.clone(),
        }
    }

    /// Set the button title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.title = title.to_string();
        self.objc.with_mut(|_obj| {
            // In real implementation: msg_send![obj, setTitle: ns_string]
        });
        Ok(())
    }

    /// Get the button title
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}

impl ObjcAccess for ButtonV2 {
    fn with_backing_obj_mut<F: Fn(*mut Object)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for ButtonV2 {}

impl Drop for ButtonV2 {
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
    fn test_button_creation() {
        let button = ButtonV2::new("Click Me").unwrap();
        assert_eq!(button.get_title(), "Click Me");
        assert!(!button.is_handle);
    }

    #[test]
    fn test_button_clone_as_handle() {
        let button = ButtonV2::new("Click Me").unwrap();
        let handle = button.clone_as_handle();
        assert!(handle.is_handle);
        assert_eq!(handle.get_title(), "Click Me");
    }

    #[test]
    fn test_button_set_title() {
        let mut button = ButtonV2::new("Click Me").unwrap();
        button.set_title("New Title").unwrap();
        assert_eq!(button.get_title(), "New Title");
    }
}
