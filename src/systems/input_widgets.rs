//! Phase 2: Input Widgets
//! 
//! Implements Streamlit-inspired input widgets for macOS GUI.
//! These are interactive components that capture user input.

use crate::core::error::Result;

/// Text input widget - single line text field
pub struct TextInput {
    placeholder: Option<String>,
    value: String,
    max_chars: Option<usize>,
    disabled: bool,
}

impl TextInput {
    /// Create a new text input widget
    pub fn new() -> Self {
        TextInput {
            placeholder: None,
            value: String::new(),
            max_chars: None,
            disabled: false,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set initial value
    pub fn value(mut self, text: impl Into<String>) -> Self {
        self.value = text.into();
        self
    }

    /// Set maximum character limit
    pub fn max_chars(mut self, max: usize) -> Self {
        self.max_chars = Some(max);
        self
    }

    crate::disabled_field!();

    /// Get the current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Get placeholder
    pub fn get_placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Text area widget - multi-line text field
pub struct TextArea {
    placeholder: Option<String>,
    value: String,
    rows: usize,
    max_chars: Option<usize>,
    disabled: bool,
}

impl TextArea {
    /// Create a new text area widget
    pub fn new() -> Self {
        TextArea {
            placeholder: None,
            value: String::new(),
            rows: 4,
            max_chars: None,
            disabled: false,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set initial value
    pub fn value(mut self, text: impl Into<String>) -> Self {
        self.value = text.into();
        self
    }

    /// Set number of rows
    pub fn rows(mut self, num: usize) -> Self {
        self.rows = num;
        self
    }

    /// Set maximum character limit
    pub fn max_chars(mut self, max: usize) -> Self {
        self.max_chars = Some(max);
        self
    }

    crate::disabled_field!();

    /// Get the current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Get number of rows
    pub fn get_rows(&self) -> usize {
        self.rows
    }
}

impl Default for TextArea {
    fn default() -> Self {
        Self::new()
    }
}

/// Chat input widget - specialized text input for chat
pub struct ChatInput {
    placeholder: Option<String>,
    max_chars: Option<usize>,
}

impl ChatInput {
    /// Create a new chat input widget
    pub fn new() -> Self {
        ChatInput {
            placeholder: Some("Chat message...".to_string()),
            max_chars: None,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set maximum character limit
    pub fn max_chars(mut self, max: usize) -> Self {
        self.max_chars = Some(max);
        self
    }

    /// Get placeholder
    pub fn get_placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }

    /// Get max chars
    pub fn get_max_chars(&self) -> Option<usize> {
        self.max_chars
    }
}

impl Default for ChatInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Slider widget - numeric range input
pub struct Slider {
    min_value: f64,
    max_value: f64,
    current_value: f64,
    step: f64,
    label: Option<String>,
}

impl Slider {
    /// Create a new slider widget
    pub fn new(min: f64, max: f64) -> Result<Self> {
        if min >= max {
            return Err("Min must be less than max".into());
        }
        Ok(Slider {
            min_value: min,
            max_value: max,
            current_value: min,
            step: 1.0,
            label: None,
        })
    }

    /// Set current value
    pub fn value(mut self, val: f64) -> Result<Self> {
        if val < self.min_value || val > self.max_value {
            return Err("Value out of range".into());
        }
        self.current_value = val;
        Ok(self)
    }

    /// Set step size
    pub fn step(mut self, s: f64) -> Self {
        self.step = s;
        self
    }

    crate::label_field!();

    /// Get current value
    pub fn get_value(&self) -> f64 {
        self.current_value
    }

    /// Get min value
    pub fn get_min(&self) -> f64 {
        self.min_value
    }

    /// Get max value
    pub fn get_max(&self) -> f64 {
        self.max_value
    }
}

/// Number input widget
pub struct NumberInput {
    value: f64,
    min_value: Option<f64>,
    max_value: Option<f64>,
    step: f64,
    label: Option<String>,
}

impl NumberInput {
    /// Create a new number input widget
    pub fn new(initial: f64) -> Self {
        NumberInput {
            value: initial,
            min_value: None,
            max_value: None,
            step: 1.0,
            label: None,
        }
    }

    /// Set minimum value
    pub fn min(mut self, min: f64) -> Self {
        self.min_value = Some(min);
        self
    }

    /// Set maximum value
    pub fn max(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Set step size
    pub fn step(mut self, s: f64) -> Self {
        self.step = s;
        self
    }

    crate::label_field!();

    /// Get current value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Get min value
    pub fn get_min(&self) -> Option<f64> {
        self.min_value
    }

    /// Get max value
    pub fn get_max(&self) -> Option<f64> {
        self.max_value
    }
}

/// Color picker widget
pub struct ColorPicker {
    color: String,
    label: Option<String>,
}

impl ColorPicker {
    /// Create a new color picker widget
    pub fn new(initial_color: impl Into<String>) -> Self {
        ColorPicker {
            color: initial_color.into(),
            label: None,
        }
    }

    crate::label_field!();

    /// Get current color
    pub fn get_color(&self) -> &str {
        &self.color
    }
}

/// Date input widget
pub struct DateInput {
    value: String, // ISO 8601 format: YYYY-MM-DD
    label: Option<String>,
}

impl DateInput {
    /// Create a new date input widget
    pub fn new() -> Self {
        DateInput {
            value: String::new(),
            label: None,
        }
    }

    /// Set initial date (ISO 8601 format: YYYY-MM-DD)
    pub fn value(mut self, date: impl Into<String>) -> Self {
        self.value = date.into();
        self
    }

    /// Set label (alias for with_label for consistency)
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Set label
    pub fn with_label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Get current date
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Get label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl Default for DateInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Time input widget
pub struct TimeInput {
    value: String, // HH:MM format
    label: Option<String>,
}

impl TimeInput {
    /// Create a new time input widget
    pub fn new() -> Self {
        TimeInput {
            value: String::new(),
            label: None,
        }
    }

    /// Set initial time (HH:MM format)
    pub fn value(mut self, time: impl Into<String>) -> Self {
        self.value = time.into();
        self
    }

    /// Set label (alias for with_label for consistency)
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Set label
    pub fn with_label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Get current time
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Get label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl Default for TimeInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input_creation() {
        let input = TextInput::new().placeholder("Enter text");
        assert_eq!(input.get_placeholder(), Some("Enter text"));
    }

    #[test]
    fn test_text_area_rows() {
        let area = TextArea::new().rows(8);
        assert_eq!(area.get_rows(), 8);
    }

    #[test]
    fn test_chat_input_default() {
        let chat = ChatInput::new();
        assert_eq!(chat.get_placeholder(), Some("Chat message..."));
    }

    #[test]
    fn test_slider_creation() {
        let slider = Slider::new(0.0, 100.0).unwrap();
        assert_eq!(slider.get_min(), 0.0);
        assert_eq!(slider.get_max(), 100.0);
    }

    #[test]
    fn test_slider_invalid_range() {
        let result = Slider::new(100.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_number_input() {
        let num = NumberInput::new(42.0).min(0.0).max(100.0);
        assert_eq!(num.get_value(), 42.0);
        assert_eq!(num.get_min(), Some(0.0));
    }

    #[test]
    fn test_color_picker() {
        let picker = ColorPicker::new("#FF0000").label("Pick Color");
        assert_eq!(picker.get_color(), "#FF0000");
        assert_eq!(picker.get_label(), Some("Pick Color"));
    }

    #[test]
    fn test_date_input() {
        let date = DateInput::new().value("2025-10-25");
        assert_eq!(date.get_value(), "2025-10-25");
    }

    #[test]
    fn test_time_input() {
        let time = TimeInput::new().value("14:30");
        assert_eq!(time.get_value(), "14:30");
    }
}
