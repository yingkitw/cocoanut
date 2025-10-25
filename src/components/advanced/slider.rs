//! Slider control for macOS GUI applications
//!
//! Provides numeric range selection with builder pattern support.

use crate::core::error::Result;

/// A slider control for numeric range selection
pub struct Slider {
    min_value: f64,
    max_value: f64,
    current_value: f64,
}

impl Slider {
    /// Create a new slider builder
    pub fn builder() -> SliderBuilder {
        SliderBuilder::new()
    }

    /// Create a new slider with range
    pub fn new(min: f64, max: f64) -> Result<Self> {
        Ok(Slider {
            min_value: min,
            max_value: max,
            current_value: min,
        })
    }

    /// Get the minimum value
    pub fn min_value(&self) -> f64 {
        self.min_value
    }

    /// Get the maximum value
    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    /// Get the current value
    pub fn current_value(&self) -> f64 {
        self.current_value
    }

    /// Set the current value
    pub fn set_value(&mut self, value: f64) -> Result<()> {
        if value >= self.min_value && value <= self.max_value {
            self.current_value = value;
            Ok(())
        } else {
            Err(crate::core::error::CocoanutError::InvalidParameter(
                format!("Value {} out of range [{}, {}]", value, self.min_value, self.max_value)
            ))
        }
    }
}

/// Builder for Slider controls
pub struct SliderBuilder {
    min_value: f64,
    max_value: f64,
    current_value: f64,
}

impl SliderBuilder {
    /// Create a new slider builder
    pub fn new() -> Self {
        Self {
            min_value: 0.0,
            max_value: 100.0,
            current_value: 0.0,
        }
    }

    /// Set the minimum value
    pub fn min(mut self, min: f64) -> Self {
        self.min_value = min;
        self
    }

    /// Set the maximum value
    pub fn max(mut self, max: f64) -> Self {
        self.max_value = max;
        self
    }

    /// Set the current value
    pub fn value(mut self, value: f64) -> Self {
        self.current_value = value;
        self
    }

    /// Build the slider
    pub fn build(self) -> Result<Slider> {
        Ok(Slider {
            min_value: self.min_value,
            max_value: self.max_value,
            current_value: self.current_value,
        })
    }
}

impl Default for SliderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_creation() {
        let slider = Slider::new(0.0, 100.0).unwrap();
        assert_eq!(slider.min_value(), 0.0);
        assert_eq!(slider.max_value(), 100.0);
        assert_eq!(slider.current_value(), 0.0);
    }

    #[test]
    fn test_slider_builder() {
        let slider = SliderBuilder::new()
            .min(0.0)
            .max(100.0)
            .value(50.0)
            .build()
            .unwrap();
        
        assert_eq!(slider.min_value(), 0.0);
        assert_eq!(slider.max_value(), 100.0);
        assert_eq!(slider.current_value(), 50.0);
    }

    #[test]
    fn test_slider_set_value() {
        let mut slider = Slider::new(0.0, 100.0).unwrap();
        slider.set_value(75.0).unwrap();
        assert_eq!(slider.current_value(), 75.0);
    }

    #[test]
    fn test_slider_set_value_out_of_range() {
        let mut slider = Slider::new(0.0, 100.0).unwrap();
        let result = slider.set_value(150.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_slider_builder_fluent() {
        let slider = SliderBuilder::new()
            .min(10.0)
            .max(90.0)
            .value(50.0)
            .build()
            .unwrap();
        
        assert_eq!(slider.min_value(), 10.0);
        assert_eq!(slider.max_value(), 90.0);
        assert_eq!(slider.current_value(), 50.0);
    }
}
