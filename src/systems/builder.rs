//! Builder patterns for simplified UI creation
//!
//! This module provides fluent builder APIs for creating UI components,
//! reducing boilerplate and improving code readability compared to raw objc calls.

use crate::controls::{Button, Label, TextField};
use crate::window::Window;
use crate::core::error::Result;
use std::sync::Arc;

/// Callback type for button click events
pub type OnClickCallback = Arc<dyn Fn() + Send + Sync>;

/// Callback type for text field change events
pub type OnChangeCallback = Arc<dyn Fn(String) + Send + Sync>;

/// Builder for Button controls
pub struct ButtonBuilder {
    title: String,
    width: Option<f64>,
    height: Option<f64>,
    enabled: bool,
    on_click: Option<OnClickCallback>,
}

impl ButtonBuilder {
    /// Create a new button builder
    pub fn new() -> Self {
        Self {
            title: String::new(),
            width: None,
            height: None,
            enabled: true,
            on_click: None,
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

    /// Set the on_click callback
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Arc::new(callback));
        self
    }

    /// Get the on_click callback if set
    pub fn get_on_click(&self) -> Option<&OnClickCallback> {
        self.on_click.as_ref()
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
    on_change: Option<OnChangeCallback>,
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
            on_change: None,
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

    /// Set the on_change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    /// Get the on_change callback if set
    pub fn get_on_change(&self) -> Option<&OnChangeCallback> {
        self.on_change.as_ref()
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

/// Builder for Window creation with fluent API
pub struct WindowBuilder {
    title: String,
    width: f64,
    height: f64,
    center: bool,
    resizable: bool,
    minimizable: bool,
    closable: bool,
}

impl WindowBuilder {
    /// Create a new window builder
    pub fn new() -> Self {
        Self {
            title: "Window".to_string(),
            width: 800.0,
            height: 600.0,
            center: false,
            resizable: true,
            minimizable: true,
            closable: true,
        }
    }

    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the window size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the window width
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Set the window height
    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    /// Center the window on screen
    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    /// Set whether the window is resizable
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set whether the window is minimizable
    pub fn minimizable(mut self, minimizable: bool) -> Self {
        self.minimizable = minimizable;
        self
    }

    /// Set whether the window is closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Build the window
    pub fn build(self) -> Result<Window> {
        let mut window = Window::new(&self.title, self.width, self.height)?;
        
        if self.center {
            window.center()?;
        }
        
        Ok(window)
    }
}

impl Default for WindowBuilder {
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

    #[test]
    fn test_window_builder_creation() {
        let builder = WindowBuilder::new();
        assert_eq!(builder.title, "Window");
        assert_eq!(builder.width, 800.0);
        assert_eq!(builder.height, 600.0);
        assert!(!builder.center);
        assert!(builder.resizable);
    }

    #[test]
    fn test_window_builder_with_title() {
        let builder = WindowBuilder::new().title("My App");
        assert_eq!(builder.title, "My App");
    }

    #[test]
    fn test_window_builder_with_size() {
        let builder = WindowBuilder::new().size(1024.0, 768.0);
        assert_eq!(builder.width, 1024.0);
        assert_eq!(builder.height, 768.0);
    }

    #[test]
    fn test_window_builder_with_center() {
        let builder = WindowBuilder::new().center();
        assert!(builder.center);
    }

    #[test]
    fn test_window_builder_with_resizable() {
        let builder = WindowBuilder::new().resizable(false);
        assert!(!builder.resizable);
    }

    #[test]
    fn test_window_builder_fluent_api() {
        let builder = WindowBuilder::new()
            .title("Test App")
            .size(1024.0, 768.0)
            .center()
            .resizable(false)
            .minimizable(false)
            .closable(true);
        
        assert_eq!(builder.title, "Test App");
        assert_eq!(builder.width, 1024.0);
        assert_eq!(builder.height, 768.0);
        assert!(builder.center);
        assert!(!builder.resizable);
        assert!(!builder.minimizable);
        assert!(builder.closable);
    }

    #[test]
    fn test_window_builder_default() {
        let builder = WindowBuilder::default();
        assert_eq!(builder.title, "Window");
        assert_eq!(builder.width, 800.0);
        assert_eq!(builder.height, 600.0);
    }

    // Event Binding Tests

    #[test]
    fn test_button_builder_with_on_click() {
        let click_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let click_count_clone = click_count.clone();
        
        let builder = ButtonBuilder::new()
            .title("Click Me")
            .on_click(move || {
                click_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
        
        assert!(builder.get_on_click().is_some());
        
        // Call the callback
        if let Some(callback) = builder.get_on_click() {
            callback();
            assert_eq!(click_count.load(std::sync::atomic::Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn test_button_builder_on_click_multiple_calls() {
        let click_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let click_count_clone = click_count.clone();
        
        let builder = ButtonBuilder::new()
            .on_click(move || {
                click_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
        
        if let Some(callback) = builder.get_on_click() {
            callback();
            callback();
            callback();
            assert_eq!(click_count.load(std::sync::atomic::Ordering::SeqCst), 3);
        }
    }

    #[test]
    fn test_button_builder_without_on_click() {
        let builder = ButtonBuilder::new().title("No Click");
        assert!(builder.get_on_click().is_none());
    }

    #[test]
    fn test_textfield_builder_with_on_change() {
        let last_value = Arc::new(std::sync::Mutex::new(String::new()));
        let last_value_clone = last_value.clone();
        
        let builder = TextFieldBuilder::new()
            .text("Initial")
            .on_change(move |text| {
                *last_value_clone.lock().unwrap() = text;
            });
        
        assert!(builder.get_on_change().is_some());
        
        // Call the callback
        if let Some(callback) = builder.get_on_change() {
            callback("Updated".to_string());
            assert_eq!(*last_value.lock().unwrap(), "Updated");
        }
    }

    #[test]
    fn test_textfield_builder_on_change_multiple_calls() {
        let call_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let call_count_clone = call_count.clone();
        
        let builder = TextFieldBuilder::new()
            .on_change(move |_text| {
                call_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
        
        if let Some(callback) = builder.get_on_change() {
            callback("First".to_string());
            callback("Second".to_string());
            callback("Third".to_string());
            assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 3);
        }
    }

    #[test]
    fn test_textfield_builder_without_on_change() {
        let builder = TextFieldBuilder::new().text("No Change");
        assert!(builder.get_on_change().is_none());
    }

    #[test]
    fn test_button_builder_fluent_with_on_click() {
        let builder = ButtonBuilder::new()
            .title("Click Me")
            .size(100.0, 50.0)
            .enabled(true)
            .on_click(|| {});
        
        assert_eq!(builder.title, "Click Me");
        assert!(builder.get_on_click().is_some());
    }

    #[test]
    fn test_textfield_builder_fluent_with_on_change() {
        let builder = TextFieldBuilder::new()
            .text("Initial")
            .placeholder("Enter text")
            .size(300.0, 40.0)
            .editable(true)
            .on_change(|_| {});
        
        assert_eq!(builder.text, "Initial");
        assert!(builder.get_on_change().is_some());
    }

    #[test]
    fn test_button_on_click_with_closure_capture() {
        let message = Arc::new(std::sync::Mutex::new(String::new()));
        let message_clone = message.clone();
        
        let builder = ButtonBuilder::new()
            .on_click(move || {
                *message_clone.lock().unwrap() = "Button clicked!".to_string();
            });
        
        if let Some(callback) = builder.get_on_click() {
            callback();
            assert_eq!(*message.lock().unwrap(), "Button clicked!");
        }
    }
}
