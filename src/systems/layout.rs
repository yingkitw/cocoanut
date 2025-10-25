//! Layout system for simplified UI composition
//!
//! Provides simple layout containers (VStack, HStack) for organizing UI components
//! without manual positioning, inspired by SwiftUI.

use crate::core::error::Result;

/// Represents the spacing between items in a layout
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    /// Space in points
    pub value: f64,
}

impl Spacing {
    /// Create spacing with a specific value
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    /// Standard spacing (8 points)
    pub fn standard() -> Self {
        Self { value: 8.0 }
    }

    /// Compact spacing (4 points)
    pub fn compact() -> Self {
        Self { value: 4.0 }
    }

    /// Relaxed spacing (16 points)
    pub fn relaxed() -> Self {
        Self { value: 16.0 }
    }
}

/// Alignment for layout items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Align to the leading edge
    Leading,
    /// Center alignment
    Center,
    /// Align to the trailing edge
    Trailing,
}

/// Vertical stack layout container
#[derive(Debug)]
pub struct VStack {
    /// Spacing between items
    spacing: Spacing,
    /// Alignment of items
    alignment: Alignment,
    /// Width of the stack
    width: Option<f64>,
    /// Height of the stack
    height: Option<f64>,
}

impl VStack {
    /// Create a new vertical stack
    pub fn new() -> Self {
        Self {
            spacing: Spacing::standard(),
            alignment: Alignment::Center,
            width: None,
            height: None,
        }
    }

    /// Set the spacing between items
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the alignment of items
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the width of the stack
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the stack
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set both width and height
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Get the spacing
    pub fn get_spacing(&self) -> Spacing {
        self.spacing
    }

    /// Get the alignment
    pub fn get_alignment(&self) -> Alignment {
        self.alignment
    }

    /// Get the width
    pub fn get_width(&self) -> Option<f64> {
        self.width
    }

    /// Get the height
    pub fn get_height(&self) -> Option<f64> {
        self.height
    }
}

impl Default for VStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Horizontal stack layout container
#[derive(Debug)]
pub struct HStack {
    /// Spacing between items
    spacing: Spacing,
    /// Alignment of items
    alignment: Alignment,
    /// Width of the stack
    width: Option<f64>,
    /// Height of the stack
    height: Option<f64>,
}

impl HStack {
    /// Create a new horizontal stack
    pub fn new() -> Self {
        Self {
            spacing: Spacing::standard(),
            alignment: Alignment::Center,
            width: None,
            height: None,
        }
    }

    /// Set the spacing between items
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the alignment of items
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the width of the stack
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the stack
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set both width and height
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Get the spacing
    pub fn get_spacing(&self) -> Spacing {
        self.spacing
    }

    /// Get the alignment
    pub fn get_alignment(&self) -> Alignment {
        self.alignment
    }

    /// Get the width
    pub fn get_width(&self) -> Option<f64> {
        self.width
    }

    /// Get the height
    pub fn get_height(&self) -> Option<f64> {
        self.height
    }
}

impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Spacer for flexible spacing in layouts
#[derive(Debug, Clone, Copy)]
pub struct Spacer {
    /// Minimum space in points
    min_length: Option<f64>,
}

impl Spacer {
    /// Create a new spacer with flexible spacing
    pub fn new() -> Self {
        Self { min_length: None }
    }

    /// Create a spacer with a minimum length
    pub fn with_min_length(min_length: f64) -> Self {
        Self {
            min_length: Some(min_length),
        }
    }

    /// Get the minimum length
    pub fn min_length(&self) -> Option<f64> {
        self.min_length
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vstack_builder() {
        let stack = VStack::new()
            .spacing(Spacing::relaxed())
            .alignment(Alignment::Leading)
            .size(400.0, 600.0);

        assert_eq!(stack.get_spacing().value, 16.0);
        assert_eq!(stack.get_alignment(), Alignment::Leading);
        assert_eq!(stack.get_width(), Some(400.0));
        assert_eq!(stack.get_height(), Some(600.0));
    }

    #[test]
    fn test_hstack_builder() {
        let stack = HStack::new()
            .spacing(Spacing::compact())
            .alignment(Alignment::Center)
            .width(800.0);

        assert_eq!(stack.get_spacing().value, 4.0);
        assert_eq!(stack.get_alignment(), Alignment::Center);
        assert_eq!(stack.get_width(), Some(800.0));
        assert_eq!(stack.get_height(), None);
    }

    #[test]
    fn test_spacing_presets() {
        assert_eq!(Spacing::standard().value, 8.0);
        assert_eq!(Spacing::compact().value, 4.0);
        assert_eq!(Spacing::relaxed().value, 16.0);
    }

    #[test]
    fn test_spacer() {
        let spacer1 = Spacer::new();
        assert_eq!(spacer1.min_length(), None);

        let spacer2 = Spacer::with_min_length(20.0);
        assert_eq!(spacer2.min_length(), Some(20.0));
    }
}
