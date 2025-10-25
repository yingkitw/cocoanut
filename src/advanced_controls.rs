//! Advanced control components for macOS GUI applications
//!
//! Includes SegmentedControl, Stepper, and Switch controls.

use crate::error::Result;

// ============================================================================
// SEGMENTED CONTROL
// ============================================================================

/// A segmented control for multiple choice selection
pub struct SegmentedControl {
    segments: Vec<String>,
    selected_index: usize,
}

impl SegmentedControl {
    /// Create a new segmented control builder
    pub fn builder() -> SegmentedControlBuilder {
        SegmentedControlBuilder::new()
    }

    /// Create a new segmented control with segments
    pub fn new(segments: Vec<String>) -> Result<Self> {
        if segments.is_empty() {
            return Err(crate::error::CocoanutError::InvalidParameter(
                "Segments cannot be empty".to_string()
            ));
        }
        Ok(SegmentedControl {
            segments,
            selected_index: 0,
        })
    }

    /// Get the segments
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Get the selected index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Set the selected index
    pub fn set_selected_index(&mut self, index: usize) -> Result<()> {
        if index < self.segments.len() {
            self.selected_index = index;
            Ok(())
        } else {
            Err(crate::error::CocoanutError::InvalidParameter(
                format!("Index {} out of bounds", index)
            ))
        }
    }
}

/// Builder for SegmentedControl
pub struct SegmentedControlBuilder {
    segments: Vec<String>,
    selected_index: usize,
}

impl SegmentedControlBuilder {
    /// Create a new segmented control builder
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            selected_index: 0,
        }
    }

    /// Add a segment
    pub fn segment(mut self, label: impl Into<String>) -> Self {
        self.segments.push(label.into());
        self
    }

    /// Set the selected index
    pub fn selected(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    /// Build the segmented control
    pub fn build(self) -> Result<SegmentedControl> {
        SegmentedControl::new(self.segments)
    }
}

impl Default for SegmentedControlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STEPPER
// ============================================================================

/// A stepper control for incrementing/decrementing values
pub struct Stepper {
    min_value: i32,
    max_value: i32,
    current_value: i32,
}

impl Stepper {
    /// Create a new stepper builder
    pub fn builder() -> StepperBuilder {
        StepperBuilder::new()
    }

    /// Create a new stepper with range
    pub fn new(min: i32, max: i32) -> Result<Self> {
        Ok(Stepper {
            min_value: min,
            max_value: max,
            current_value: min,
        })
    }

    /// Get the current value
    pub fn value(&self) -> i32 {
        self.current_value
    }

    /// Increment the value
    pub fn increment(&mut self) -> Result<()> {
        if self.current_value < self.max_value {
            self.current_value += 1;
            Ok(())
        } else {
            Err(crate::error::CocoanutError::InvalidParameter(
                "Cannot increment beyond max value".to_string()
            ))
        }
    }

    /// Decrement the value
    pub fn decrement(&mut self) -> Result<()> {
        if self.current_value > self.min_value {
            self.current_value -= 1;
            Ok(())
        } else {
            Err(crate::error::CocoanutError::InvalidParameter(
                "Cannot decrement below min value".to_string()
            ))
        }
    }
}

/// Builder for Stepper
pub struct StepperBuilder {
    min_value: i32,
    max_value: i32,
    current_value: i32,
}

impl StepperBuilder {
    /// Create a new stepper builder
    pub fn new() -> Self {
        Self {
            min_value: 0,
            max_value: 100,
            current_value: 0,
        }
    }

    /// Set the minimum value
    pub fn min(mut self, min: i32) -> Self {
        self.min_value = min;
        self
    }

    /// Set the maximum value
    pub fn max(mut self, max: i32) -> Self {
        self.max_value = max;
        self
    }

    /// Set the current value
    pub fn value(mut self, value: i32) -> Self {
        self.current_value = value;
        self
    }

    /// Build the stepper
    pub fn build(self) -> Result<Stepper> {
        Ok(Stepper {
            min_value: self.min_value,
            max_value: self.max_value,
            current_value: self.current_value,
        })
    }
}

impl Default for StepperBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SWITCH
// ============================================================================

/// A switch control for on/off toggling
pub struct Switch {
    label: String,
    enabled: bool,
}

impl Switch {
    /// Create a new switch builder
    pub fn builder() -> SwitchBuilder {
        SwitchBuilder::new()
    }

    /// Create a new switch with a label
    pub fn new(label: &str) -> Result<Self> {
        Ok(Switch {
            label: label.to_string(),
            enabled: false,
        })
    }

    /// Get the switch label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Check if the switch is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set the enabled state
    pub fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.enabled = enabled;
        Ok(())
    }
}

/// Builder for Switch
pub struct SwitchBuilder {
    label: String,
    enabled: bool,
}

impl SwitchBuilder {
    /// Create a new switch builder
    pub fn new() -> Self {
        Self {
            label: String::new(),
            enabled: false,
        }
    }

    /// Set the switch label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set the initial enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Build the switch
    pub fn build(self) -> Result<Switch> {
        Ok(Switch {
            label: self.label,
            enabled: self.enabled,
        })
    }
}

impl Default for SwitchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SegmentedControl Tests
    #[test]
    fn test_segmented_control_creation() {
        let control = SegmentedControl::new(vec!["Option 1".to_string(), "Option 2".to_string()]).unwrap();
        assert_eq!(control.segments().len(), 2);
        assert_eq!(control.selected_index(), 0);
    }

    #[test]
    fn test_segmented_control_builder() {
        let control = SegmentedControlBuilder::new()
            .segment("A")
            .segment("B")
            .segment("C")
            .build()
            .unwrap();
        
        assert_eq!(control.segments().len(), 3);
        assert_eq!(control.selected_index(), 0);
    }

    #[test]
    fn test_segmented_control_set_selected() {
        let mut control = SegmentedControl::new(vec!["A".to_string(), "B".to_string()]).unwrap();
        control.set_selected_index(1).unwrap();
        assert_eq!(control.selected_index(), 1);
    }

    #[test]
    fn test_segmented_control_empty() {
        let result = SegmentedControl::new(vec![]);
        assert!(result.is_err());
    }

    // Stepper Tests
    #[test]
    fn test_stepper_creation() {
        let stepper = Stepper::new(0, 10).unwrap();
        assert_eq!(stepper.value(), 0);
    }

    #[test]
    fn test_stepper_increment() {
        let mut stepper = Stepper::new(0, 10).unwrap();
        stepper.increment().unwrap();
        assert_eq!(stepper.value(), 1);
    }

    #[test]
    fn test_stepper_decrement() {
        let mut stepper = Stepper::new(0, 10).unwrap();
        stepper.increment().unwrap();
        stepper.decrement().unwrap();
        assert_eq!(stepper.value(), 0);
    }

    #[test]
    fn test_stepper_builder() {
        let stepper = StepperBuilder::new()
            .min(5)
            .max(15)
            .value(10)
            .build()
            .unwrap();
        
        assert_eq!(stepper.value(), 10);
    }

    // Switch Tests
    #[test]
    fn test_switch_creation() {
        let switch = Switch::new("WiFi").unwrap();
        assert_eq!(switch.label(), "WiFi");
        assert!(!switch.is_enabled());
    }

    #[test]
    fn test_switch_builder() {
        let switch = SwitchBuilder::new()
            .label("Bluetooth")
            .enabled(true)
            .build()
            .unwrap();
        
        assert_eq!(switch.label(), "Bluetooth");
        assert!(switch.is_enabled());
    }

    #[test]
    fn test_switch_set_enabled() {
        let mut switch = Switch::new("Test").unwrap();
        switch.set_enabled(true).unwrap();
        assert!(switch.is_enabled());
    }

    #[test]
    fn test_switch_toggle() {
        let mut switch = Switch::new("Toggle").unwrap();
        assert!(!switch.is_enabled());
        switch.set_enabled(true).unwrap();
        assert!(switch.is_enabled());
        switch.set_enabled(false).unwrap();
        assert!(!switch.is_enabled());
    }
}
