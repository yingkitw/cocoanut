//! Phase 2: Selection Widgets
//! 
//! Implements selection-based input widgets for macOS GUI.

use crate::core::error::Result;

/// Button widget - clickable button
pub struct Button {
    label: String,
    disabled: bool,
    variant: ButtonVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Primary action button
    Primary,
    /// Secondary action button
    Secondary,
    /// Danger/destructive action
    Danger,
}

impl Button {
    /// Create a new button widget
    pub fn new(label: impl Into<String>) -> Self {
        Button {
            label: label.into(),
            disabled: false,
            variant: ButtonVariant::Primary,
        }
    }

    /// Set button variant
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }

    crate::disabled_field!();

    /// Get button label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Get button variant
    pub fn get_variant(&self) -> ButtonVariant {
        self.variant
    }
}

/// Checkbox widget - boolean toggle
pub struct Checkbox {
    label: String,
    checked: bool,
    disabled: bool,
}

impl Checkbox {
    /// Create a new checkbox widget
    pub fn new(label: impl Into<String>) -> Self {
        Checkbox {
            label: label.into(),
            checked: false,
            disabled: false,
        }
    }

    /// Set initial checked state
    pub fn checked(mut self, state: bool) -> Self {
        self.checked = state;
        self
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Check if checked
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

/// Radio button group widget
pub struct Radio {
    label: String,
    options: Vec<String>,
    selected: Option<usize>,
    disabled: bool,
}

impl Radio {
    /// Create a new radio button group
    pub fn new(label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.is_empty() {
            return Err("Radio must have at least one option".into());
        }
        Ok(Radio {
            label: label.into(),
            options,
            selected: Some(0),
            disabled: false,
        })
    }

    /// Set selected option index
    pub fn selected(mut self, index: usize) -> Result<Self> {
        if index >= self.options.len() {
            return Err("Selected index out of range".into());
        }
        self.selected = Some(index);
        Ok(self)
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Get options
    pub fn get_options(&self) -> &[String] {
        &self.options
    }

    /// Get selected index
    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }

    /// Get selected option text
    pub fn get_selected_text(&self) -> Option<&str> {
        self.selected.and_then(|i| self.options.get(i).map(|s| s.as_str()))
    }
}

/// Selectbox widget - dropdown selection
pub struct Selectbox {
    label: String,
    options: Vec<String>,
    selected: Option<usize>,
    placeholder: Option<String>,
    disabled: bool,
}

impl Selectbox {
    /// Create a new selectbox widget
    pub fn new(label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.is_empty() {
            return Err("Selectbox must have at least one option".into());
        }
        Ok(Selectbox {
            label: label.into(),
            options,
            selected: None,
            placeholder: None,
            disabled: false,
        })
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set selected option index
    pub fn selected(mut self, index: usize) -> Result<Self> {
        if index >= self.options.len() {
            return Err("Selected index out of range".into());
        }
        self.selected = Some(index);
        Ok(self)
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Get options
    pub fn get_options(&self) -> &[String] {
        &self.options
    }

    /// Get selected index
    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }
}

/// Multiselect widget - multiple selection
pub struct Multiselect {
    label: String,
    options: Vec<String>,
    selected: Vec<usize>,
    placeholder: Option<String>,
    disabled: bool,
}

impl Multiselect {
    /// Create a new multiselect widget
    pub fn new(label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.is_empty() {
            return Err("Multiselect must have at least one option".into());
        }
        Ok(Multiselect {
            label: label.into(),
            options,
            selected: Vec::new(),
            placeholder: None,
            disabled: false,
        })
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Add selected option index
    pub fn select(mut self, index: usize) -> Result<Self> {
        if index >= self.options.len() {
            return Err("Index out of range".into());
        }
        if !self.selected.contains(&index) {
            self.selected.push(index);
        }
        Ok(self)
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Get options
    pub fn get_options(&self) -> &[String] {
        &self.options
    }

    /// Get selected indices
    pub fn get_selected(&self) -> &[usize] {
        &self.selected
    }

    /// Get selected option texts
    pub fn get_selected_texts(&self) -> Vec<&str> {
        self.selected
            .iter()
            .filter_map(|&i| self.options.get(i).map(|s| s.as_str()))
            .collect()
    }
}

/// Select slider widget - range selection
pub struct SelectSlider {
    label: String,
    options: Vec<String>,
    selected_range: (usize, usize),
    disabled: bool,
}

impl SelectSlider {
    /// Create a new select slider widget
    pub fn new(label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.len() < 2 {
            return Err("SelectSlider must have at least 2 options".into());
        }
        Ok(SelectSlider {
            label: label.into(),
            options,
            selected_range: (0, 0),
            disabled: false,
        })
    }

    /// Set selected range (start, end)
    pub fn range(mut self, start: usize, end: usize) -> Result<Self> {
        if start >= self.options.len() || end >= self.options.len() || start > end {
            return Err("Invalid range".into());
        }
        self.selected_range = (start, end);
        Ok(self)
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Get options
    pub fn get_options(&self) -> &[String] {
        &self.options
    }

    /// Get selected range
    pub fn get_range(&self) -> (usize, usize) {
        self.selected_range
    }
}

/// Button group widget - group of buttons
pub struct ButtonGroup {
    label: String,
    options: Vec<String>,
    selected: Option<usize>,
    disabled: bool,
}

impl ButtonGroup {
    /// Create a new button group widget
    pub fn new(label: impl Into<String>, options: Vec<String>) -> Result<Self> {
        if options.is_empty() {
            return Err("ButtonGroup must have at least one option".into());
        }
        Ok(ButtonGroup {
            label: label.into(),
            options,
            selected: Some(0),
            disabled: false,
        })
    }

    /// Set selected option index
    pub fn selected(mut self, index: usize) -> Result<Self> {
        if index >= self.options.len() {
            return Err("Selected index out of range".into());
        }
        self.selected = Some(index);
        Ok(self)
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Get options
    pub fn get_options(&self) -> &[String] {
        &self.options
    }

    /// Get selected index
    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let btn = Button::new("Click me").variant(ButtonVariant::Primary);
        assert_eq!(btn.get_label(), "Click me");
        assert_eq!(btn.get_variant(), ButtonVariant::Primary);
    }

    #[test]
    fn test_checkbox() {
        let cb = Checkbox::new("Accept").checked(true);
        assert!(cb.is_checked());
    }

    #[test]
    fn test_radio_creation() {
        let radio = Radio::new("Choose", vec!["A".to_string(), "B".to_string()]).unwrap();
        assert_eq!(radio.get_selected_text(), Some("A"));
    }

    #[test]
    fn test_radio_empty_options() {
        let result = Radio::new("Choose", vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_selectbox() {
        let sb = Selectbox::new("Pick", vec!["X".to_string(), "Y".to_string()])
            .unwrap()
            .placeholder("Select...");
        assert_eq!(sb.get_options().len(), 2);
    }

    #[test]
    fn test_multiselect() {
        let ms = Multiselect::new("Multi", vec!["A".to_string(), "B".to_string(), "C".to_string()])
            .unwrap()
            .select(0)
            .unwrap()
            .select(2)
            .unwrap();
        assert_eq!(ms.get_selected().len(), 2);
    }

    #[test]
    fn test_select_slider() {
        let ss = SelectSlider::new(
            "Range",
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        )
        .unwrap()
        .range(0, 2)
        .unwrap();
        assert_eq!(ss.get_range(), (0, 2));
    }

    #[test]
    fn test_button_group() {
        let bg = ButtonGroup::new("Group", vec!["Opt1".to_string(), "Opt2".to_string()]).unwrap();
        assert_eq!(bg.get_selected(), Some(0));
    }
}
