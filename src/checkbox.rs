//! Checkbox control for macOS GUI applications
//!
//! Provides a simple on/off toggle control with builder pattern support.

use crate::error::Result;

/// A checkbox control for boolean selection
pub struct Checkbox {
    label: String,
    checked: bool,
}

impl Checkbox {
    /// Create a new checkbox builder
    pub fn builder() -> CheckboxBuilder {
        CheckboxBuilder::new()
    }

    /// Create a new checkbox with a label
    pub fn new(label: &str) -> Result<Self> {
        Ok(Checkbox {
            label: label.to_string(),
            checked: false,
        })
    }

    /// Get the checkbox label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Check if the checkbox is checked
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    /// Set the checked state
    pub fn set_checked(&mut self, checked: bool) -> Result<()> {
        self.checked = checked;
        Ok(())
    }
}

/// Builder for Checkbox controls
pub struct CheckboxBuilder {
    label: String,
    checked: bool,
}

impl CheckboxBuilder {
    /// Create a new checkbox builder
    pub fn new() -> Self {
        Self {
            label: String::new(),
            checked: false,
        }
    }

    /// Set the checkbox label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set the initial checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Build the checkbox
    pub fn build(self) -> Result<Checkbox> {
        Ok(Checkbox {
            label: self.label,
            checked: self.checked,
        })
    }
}

impl Default for CheckboxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_creation() {
        let checkbox = Checkbox::new("Accept").unwrap();
        assert_eq!(checkbox.label(), "Accept");
        assert!(!checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_builder() {
        let checkbox = CheckboxBuilder::new()
            .label("Agree")
            .checked(true)
            .build()
            .unwrap();
        
        assert_eq!(checkbox.label(), "Agree");
        assert!(checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_set_checked() {
        let mut checkbox = Checkbox::new("Test").unwrap();
        assert!(!checkbox.is_checked());
        
        checkbox.set_checked(true).unwrap();
        assert!(checkbox.is_checked());
        
        checkbox.set_checked(false).unwrap();
        assert!(!checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_builder_default() {
        let checkbox = CheckboxBuilder::default()
            .label("Default")
            .build()
            .unwrap();
        
        assert_eq!(checkbox.label(), "Default");
        assert!(!checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_builder_fluent() {
        let checkbox = CheckboxBuilder::new()
            .label("Fluent")
            .checked(true)
            .build()
            .unwrap();
        
        assert_eq!(checkbox.label(), "Fluent");
        assert!(checkbox.is_checked());
    }
}
