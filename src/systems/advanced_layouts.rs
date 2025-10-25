//! Phase 3: Advanced Layouts
//! 
//! Implements advanced layout systems for macOS GUI.
//! These provide flexible and powerful layout options.

use crate::core::error::Result;

/// Row layout - horizontal arrangement
pub struct Row {
    gap: f64,
    vertical_align: VerticalAlignment,
    wrap: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Align to top
    Top,
    /// Center vertically
    Center,
    /// Align to bottom
    Bottom,
    /// Stretch to fill
    Stretch,
}

impl Row {
    /// Create a new row layout
    pub fn new() -> Self {
        Row {
            gap: 10.0,
            vertical_align: VerticalAlignment::Center,
            wrap: false,
        }
    }

    /// Set gap between items
    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    /// Set vertical alignment
    pub fn vertical_align(mut self, align: VerticalAlignment) -> Self {
        self.vertical_align = align;
        self
    }

    /// Enable wrapping
    pub fn wrap(mut self, enable: bool) -> Self {
        self.wrap = enable;
        self
    }

    /// Get gap
    pub fn get_gap(&self) -> f64 {
        self.gap
    }

    /// Get vertical alignment
    pub fn get_vertical_align(&self) -> VerticalAlignment {
        self.vertical_align
    }

    /// Check if wrapping enabled
    pub fn wraps(&self) -> bool {
        self.wrap
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

/// Column layout - vertical arrangement
pub struct Column {
    gap: f64,
    horizontal_align: HorizontalAlignment,
    wrap: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    /// Align to left
    Left,
    /// Center horizontally
    Center,
    /// Align to right
    Right,
    /// Stretch to fill
    Stretch,
}

impl Column {
    /// Create a new column layout
    pub fn new() -> Self {
        Column {
            gap: 10.0,
            horizontal_align: HorizontalAlignment::Stretch,
            wrap: false,
        }
    }

    /// Set gap between items
    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    /// Set horizontal alignment
    pub fn horizontal_align(mut self, align: HorizontalAlignment) -> Self {
        self.horizontal_align = align;
        self
    }

    /// Enable wrapping
    pub fn wrap(mut self, enable: bool) -> Self {
        self.wrap = enable;
        self
    }

    /// Get gap
    pub fn get_gap(&self) -> f64 {
        self.gap
    }

    /// Get horizontal alignment
    pub fn get_horizontal_align(&self) -> HorizontalAlignment {
        self.horizontal_align
    }

    /// Check if wrapping enabled
    pub fn wraps(&self) -> bool {
        self.wrap
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

/// Grid layout - 2D grid arrangement
pub struct Grid {
    columns: usize,
    rows: usize,
    gap: f64,
    column_gap: Option<f64>,
    row_gap: Option<f64>,
}

impl Grid {
    /// Create a new grid layout
    pub fn new(columns: usize, rows: usize) -> Result<Self> {
        if columns == 0 || rows == 0 {
            return Err("Grid must have at least 1 column and 1 row".into());
        }
        Ok(Grid {
            columns,
            rows,
            gap: 10.0,
            column_gap: None,
            row_gap: None,
        })
    }

    /// Set uniform gap
    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    /// Set column gap
    pub fn column_gap(mut self, gap: f64) -> Self {
        self.column_gap = Some(gap);
        self
    }

    /// Set row gap
    pub fn row_gap(mut self, gap: f64) -> Self {
        self.row_gap = Some(gap);
        self
    }

    /// Get number of columns
    pub fn get_columns(&self) -> usize {
        self.columns
    }

    /// Get number of rows
    pub fn get_rows(&self) -> usize {
        self.rows
    }

    /// Get gap
    pub fn get_gap(&self) -> f64 {
        self.gap
    }

    /// Get column gap
    pub fn get_column_gap(&self) -> f64 {
        self.column_gap.unwrap_or(self.gap)
    }

    /// Get row gap
    pub fn get_row_gap(&self) -> f64 {
        self.row_gap.unwrap_or(self.gap)
    }
}

/// FlexSpacer - flexible spacing element
pub struct FlexSpacer {
    flex: f64,
    min_size: Option<f64>,
    max_size: Option<f64>,
}

impl FlexSpacer {
    /// Create a new flex spacer
    pub fn new() -> Self {
        FlexSpacer {
            flex: 1.0,
            min_size: None,
            max_size: None,
        }
    }

    /// Set flex factor
    pub fn flex(mut self, factor: f64) -> Self {
        self.flex = factor;
        self
    }

    /// Set minimum size
    pub fn min_size(mut self, size: f64) -> Self {
        self.min_size = Some(size);
        self
    }

    /// Set maximum size
    pub fn max_size(mut self, size: f64) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Get flex factor
    pub fn get_flex(&self) -> f64 {
        self.flex
    }

    /// Get minimum size
    pub fn get_min_size(&self) -> Option<f64> {
        self.min_size
    }

    /// Get maximum size
    pub fn get_max_size(&self) -> Option<f64> {
        self.max_size
    }
}

impl Default for FlexSpacer {
    fn default() -> Self {
        Self::new()
    }
}

/// Divider - visual separator
pub struct Divider {
    orientation: Orientation,
    color: Option<String>,
    thickness: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Horizontal divider
    Horizontal,
    /// Vertical divider
    Vertical,
}

impl Divider {
    /// Create a new divider
    pub fn new(orientation: Orientation) -> Self {
        Divider {
            orientation,
            color: None,
            thickness: 1.0,
        }
    }

    /// Set color
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set thickness
    pub fn thickness(mut self, t: f64) -> Self {
        self.thickness = t;
        self
    }

    /// Get orientation
    pub fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    /// Get color
    pub fn get_color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Get thickness
    pub fn get_thickness(&self) -> f64 {
        self.thickness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_creation() {
        let row = Row::new().gap(15.0).vertical_align(VerticalAlignment::Top);
        assert_eq!(row.get_gap(), 15.0);
        assert_eq!(row.get_vertical_align(), VerticalAlignment::Top);
    }

    #[test]
    fn test_column_creation() {
        let col = Column::new()
            .gap(20.0)
            .horizontal_align(HorizontalAlignment::Center);
        assert_eq!(col.get_gap(), 20.0);
        assert_eq!(col.get_horizontal_align(), HorizontalAlignment::Center);
    }

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(3, 3).unwrap();
        assert_eq!(grid.get_columns(), 3);
        assert_eq!(grid.get_rows(), 3);
    }

    #[test]
    fn test_grid_invalid() {
        let result = Grid::new(0, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_grid_gaps() {
        let grid = Grid::new(2, 2)
            .unwrap()
            .column_gap(15.0)
            .row_gap(20.0);
        assert_eq!(grid.get_column_gap(), 15.0);
        assert_eq!(grid.get_row_gap(), 20.0);
    }

    #[test]
    fn test_flex_spacer() {
        let spacer = FlexSpacer::new()
            .flex(2.0)
            .min_size(10.0)
            .max_size(100.0);
        assert_eq!(spacer.get_flex(), 2.0);
        assert_eq!(spacer.get_min_size(), Some(10.0));
    }

    #[test]
    fn test_divider() {
        let divider = Divider::new(Orientation::Horizontal)
            .color("#CCCCCC")
            .thickness(2.0);
        assert_eq!(divider.get_orientation(), Orientation::Horizontal);
        assert_eq!(divider.get_thickness(), 2.0);
    }
}
