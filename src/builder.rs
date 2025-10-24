//! Builder patterns for simplified UI creation
//!
//! This module provides fluent builder APIs for creating UI components,
//! reducing boilerplate and improving code readability compared to raw objc calls.

use crate::controls::{Button, Label, TextField};
use crate::error::Result;
use std::sync::Arc;

/// Builder for Button controls
pub struct ButtonBuilder {
    title: String,
    width: Option<f64>,
    height: Option<f64>,
    enabled: bool,
}

impl ButtonBuilder {
    /// Create a new button builder
    pub fn new() -> Self {
        Self {
            title: String::new(),
            width: None,
            height: None,
            enabled: true,
        }
    }

    /// Set the button title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the button size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set the button width
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the button height
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set whether the button is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Build the button
    pub fn build(self) -> Result<Button> {
        Button::new(&self.title)
    }
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Label controls
pub struct LabelBuilder {
    text: String,
    width: Option<f64>,
    height: Option<f64>,
}

impl LabelBuilder {
    /// Create a new label builder
    pub fn new() -> Self {
        Self {
            text: String::new(),
            width: None,
            height: None,
        }
    }

    /// Set the label text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Set the label size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set the label width
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the label height
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Build the label
    pub fn build(self) -> Result<Label> {
        Label::new(&self.text)
    }
}

impl Default for LabelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for TextField controls
pub struct TextFieldBuilder {
    text: String,
    placeholder: Option<String>,
    width: Option<f64>,
    height: Option<f64>,
    editable: bool,
}

impl TextFieldBuilder {
    /// Create a new text field builder
    pub fn new() -> Self {
        Self {
            text: String::new(),
            placeholder: None,
            width: None,
            height: None,
            editable: true,
        }
    }

    /// Set the initial text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the text field size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set the text field width
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the text field height
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set whether the text field is editable
    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Build the text field
    pub fn build(self) -> Result<TextField> {
        TextField::new(&self.text)
    }
}

impl Default for TextFieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_builder() {
        let builder = ButtonBuilder::new()
            .title("Click Me")
            .size(100.0, 50.0)
            .enabled(true);
        
        assert_eq!(builder.title, "Click Me");
        assert_eq!(builder.width, Some(100.0));
        assert_eq!(builder.height, Some(50.0));
        assert!(builder.enabled);
    }

    #[test]
    fn test_label_builder() {
        let builder = LabelBuilder::new()
            .text("Hello")
            .size(200.0, 30.0);
        
        assert_eq!(builder.text, "Hello");
        assert_eq!(builder.width, Some(200.0));
        assert_eq!(builder.height, Some(30.0));
    }

    #[test]
    fn test_text_field_builder() {
        let builder = TextFieldBuilder::new()
            .text("Initial")
            .placeholder("Enter text")
            .size(300.0, 40.0)
            .editable(true);
        
        assert_eq!(builder.text, "Initial");
        assert_eq!(builder.placeholder, Some("Enter text".to_string()));
        assert_eq!(builder.width, Some(300.0));
        assert_eq!(builder.height, Some(40.0));
        assert!(builder.editable);
    }
}
