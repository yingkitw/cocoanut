//! TextField V2 - Refactored to use Cacao patterns

use crate::core::{ObjcAccess, Layout};
use crate::utils::ObjcProperty;
use crate::core::error::Result;
use objc::runtime::Object;

/// A macOS text field control using Cacao patterns
#[derive(Debug, Clone)]
pub struct TextFieldV2 {
    pub is_handle: bool,
    pub objc: ObjcProperty,
    pub text: String,
    pub placeholder: String,
}

impl TextFieldV2 {
    /// Create a new text field
    pub fn new() -> Result<Self> {
        Ok(TextFieldV2 {
            is_handle: false,
            objc: ObjcProperty::retain(std::ptr::null_mut()),
            text: String::new(),
            placeholder: String::new(),
        })
    }

    /// Create a handle (clone) of this text field
    pub fn clone_as_handle(&self) -> TextFieldV2 {
        TextFieldV2 {
            is_handle: true,
            objc: self.objc.clone(),
            text: self.text.clone(),
            placeholder: self.placeholder.clone(),
        }
    }

    /// Set the text field text
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        self.text = text.to_string();
        self.objc.with_mut(|_obj| {
            // In real implementation: msg_send![obj, setStringValue: ns_string]
        });
        Ok(())
    }

    /// Get the text field text
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Set the placeholder text
    pub fn set_placeholder(&mut self, placeholder: &str) -> Result<()> {
        self.placeholder = placeholder.to_string();
        self.objc.with_mut(|_obj| {
            // In real implementation: msg_send![obj, setPlaceholderString: ns_string]
        });
        Ok(())
    }

    /// Get the placeholder text
    pub fn get_placeholder(&self) -> String {
        self.placeholder.clone()
    }
}

impl ObjcAccess for TextFieldV2 {
    fn with_backing_obj_mut<F: Fn(*mut Object)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for TextFieldV2 {}

impl Drop for TextFieldV2 {
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
    fn test_textfield_creation() {
        let tf = TextFieldV2::new().unwrap();
        assert_eq!(tf.get_text(), "");
        assert!(!tf.is_handle);
    }

    #[test]
    fn test_textfield_clone_as_handle() {
        let tf = TextFieldV2::new().unwrap();
        let handle = tf.clone_as_handle();
        assert!(handle.is_handle);
    }

    #[test]
    fn test_textfield_set_text() {
        let mut tf = TextFieldV2::new().unwrap();
        tf.set_text("Hello").unwrap();
        assert_eq!(tf.get_text(), "Hello");
    }

    #[test]
    fn test_textfield_placeholder() {
        let mut tf = TextFieldV2::new().unwrap();
        tf.set_placeholder("Enter text").unwrap();
        assert_eq!(tf.get_placeholder(), "Enter text");
    }
}
