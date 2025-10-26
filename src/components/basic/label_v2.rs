//! Label V2 - Refactored to use Cacao patterns

use crate::core::{ObjcAccess, Layout};
use crate::utils::ObjcProperty;
use crate::core::error::Result;
use objc::runtime::Object;

/// A macOS label control using Cacao patterns
#[derive(Debug, Clone)]
pub struct LabelV2 {
    pub is_handle: bool,
    pub objc: ObjcProperty,
    pub text: String,
}

impl LabelV2 {
    /// Create a new label
    pub fn new(text: &str) -> Result<Self> {
        Ok(LabelV2 {
            is_handle: false,
            objc: ObjcProperty::retain(std::ptr::null_mut()),
            text: text.to_string(),
        })
    }

    /// Create a handle (clone) of this label
    pub fn clone_as_handle(&self) -> LabelV2 {
        LabelV2 {
            is_handle: true,
            objc: self.objc.clone(),
            text: self.text.clone(),
        }
    }

    /// Set the label text
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        self.text = text.to_string();
        self.objc.with_mut(|_obj| {
            // In real implementation: msg_send![obj, setStringValue: ns_string]
        });
        Ok(())
    }

    /// Get the label text
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl ObjcAccess for LabelV2 {
    fn with_backing_obj_mut<F: Fn(*mut Object)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for LabelV2 {}

impl Drop for LabelV2 {
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
    fn test_label_creation() {
        let label = LabelV2::new("Hello").unwrap();
        assert_eq!(label.get_text(), "Hello");
        assert!(!label.is_handle);
    }

    #[test]
    fn test_label_clone_as_handle() {
        let label = LabelV2::new("Hello").unwrap();
        let handle = label.clone_as_handle();
        assert!(handle.is_handle);
        assert_eq!(handle.get_text(), "Hello");
    }

    #[test]
    fn test_label_set_text() {
        let mut label = LabelV2::new("Hello").unwrap();
        label.set_text("World").unwrap();
        assert_eq!(label.get_text(), "World");
    }
}
