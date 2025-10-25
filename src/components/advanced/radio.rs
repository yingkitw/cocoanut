//! Radio button control for macOS GUI applications
//!
//! Provides mutually exclusive selection controls with builder pattern support.

use crate::core::error::Result;

/// A radio button control for exclusive selection
pub struct RadioButton {
    label: String,
    selected: bool,
    group_id: String,
}

impl RadioButton {
    /// Create a new radio button builder
    pub fn builder() -> RadioButtonBuilder {
        RadioButtonBuilder::new()
    }

    /// Create a new radio button with a label and group
    pub fn new(label: &str, group_id: &str) -> Result<Self> {
        Ok(RadioButton {
            label: label.to_string(),
            selected: false,
            group_id: group_id.to_string(),
        })
    }

    /// Get the radio button label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Check if the radio button is selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Set the selected state
    pub fn set_selected(&mut self, selected: bool) -> Result<()> {
        self.selected = selected;
        Ok(())
    }

    /// Get the group ID
    pub fn group_id(&self) -> &str {
        &self.group_id
    }
}

/// Builder for RadioButton controls
pub struct RadioButtonBuilder {
    label: String,
    selected: bool,
    group_id: String,
}

impl RadioButtonBuilder {
    /// Create a new radio button builder
    pub fn new() -> Self {
        Self {
            label: String::new(),
            selected: false,
            group_id: String::new(),
        }
    }

    /// Set the radio button label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set the initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set the group ID (for mutually exclusive selection)
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
        self
    }

    /// Build the radio button
    pub fn build(self) -> Result<RadioButton> {
        Ok(RadioButton {
            label: self.label,
            selected: self.selected,
            group_id: self.group_id,
        })
    }
}

impl Default for RadioButtonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_button_creation() {
        let radio = RadioButton::new("Option 1", "group1").unwrap();
        assert_eq!(radio.label(), "Option 1");
        assert_eq!(radio.group_id(), "group1");
        assert!(!radio.is_selected());
    }

    #[test]
    fn test_radio_button_builder() {
        let radio = RadioButtonBuilder::new()
            .label("Option 2")
            .group_id("group1")
            .selected(true)
            .build()
            .unwrap();
        
        assert_eq!(radio.label(), "Option 2");
        assert_eq!(radio.group_id(), "group1");
        assert!(radio.is_selected());
    }

    #[test]
    fn test_radio_button_set_selected() {
        let mut radio = RadioButton::new("Test", "group").unwrap();
        assert!(!radio.is_selected());
        
        radio.set_selected(true).unwrap();
        assert!(radio.is_selected());
    }

    #[test]
    fn test_radio_button_group_id() {
        let radio = RadioButtonBuilder::new()
            .label("Option")
            .group_id("my_group")
            .build()
            .unwrap();
        
        assert_eq!(radio.group_id(), "my_group");
    }

    #[test]
    fn test_radio_button_builder_fluent() {
        let radio = RadioButtonBuilder::new()
            .label("Fluent")
            .group_id("fluent_group")
            .selected(true)
            .build()
            .unwrap();
        
        assert_eq!(radio.label(), "Fluent");
        assert_eq!(radio.group_id(), "fluent_group");
        assert!(radio.is_selected());
    }
}
