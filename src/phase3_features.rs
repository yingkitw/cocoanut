//! macOS-specific features and integration
//!
//! Includes GridView, Touch Bar, Accessibility, Dark Mode, Drag & Drop, and Advanced Styling.

use crate::error::Result;

// ============================================================================
// GRID VIEW
// ============================================================================

/// A grid view for flexible layout
pub struct GridView {
    columns: usize,
    rows: usize,
    spacing: f64,
}

impl GridView {
    /// Create a new grid view builder
    pub fn builder() -> GridViewBuilder {
        GridViewBuilder::new()
    }

    /// Create a new grid view
    pub fn new(columns: usize, rows: usize) -> Result<Self> {
        if columns == 0 || rows == 0 {
            return Err(crate::error::CocoanutError::InvalidParameter(
                "Grid dimensions must be greater than 0".to_string()
            ));
        }
        Ok(GridView {
            columns,
            rows,
            spacing: 8.0,
        })
    }

    /// Get the number of columns
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the spacing
    pub fn spacing(&self) -> f64 {
        self.spacing
    }

    /// Set the spacing
    pub fn set_spacing(&mut self, spacing: f64) -> Result<()> {
        self.spacing = spacing;
        Ok(())
    }
}

/// Builder for GridView
pub struct GridViewBuilder {
    columns: usize,
    rows: usize,
    spacing: f64,
}

impl GridViewBuilder {
    /// Create a new grid view builder
    pub fn new() -> Self {
        Self {
            columns: 1,
            rows: 1,
            spacing: 8.0,
        }
    }

    /// Set the number of columns
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

    /// Set the number of rows
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// Set the spacing
    pub fn spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }

    /// Build the grid view
    pub fn build(self) -> Result<GridView> {
        GridView::new(self.columns, self.rows)
    }
}

impl Default for GridViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TOUCH BAR
// ============================================================================

/// A Touch Bar item
#[derive(Debug, Clone)]
pub struct TouchBarItem {
    identifier: String,
    label: String,
}

impl TouchBarItem {
    /// Create a new touch bar item
    pub fn new(identifier: impl Into<String>, label: impl Into<String>) -> Self {
        TouchBarItem {
            identifier: identifier.into(),
            label: label.into(),
        }
    }

    /// Get the identifier
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Get the label
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Touch Bar support for macOS applications
pub struct TouchBar {
    items: Vec<TouchBarItem>,
}

impl TouchBar {
    /// Create a new touch bar builder
    pub fn builder() -> TouchBarBuilder {
        TouchBarBuilder::new()
    }

    /// Create a new touch bar
    pub fn new() -> Result<Self> {
        Ok(TouchBar {
            items: Vec::new(),
        })
    }

    /// Add an item
    pub fn add_item(&mut self, item: TouchBarItem) {
        self.items.push(item);
    }

    /// Get items
    pub fn items(&self) -> &[TouchBarItem] {
        &self.items
    }
}

/// Builder for TouchBar
pub struct TouchBarBuilder {
    items: Vec<TouchBarItem>,
}

impl TouchBarBuilder {
    /// Create a new touch bar builder
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Add an item
    pub fn item(mut self, item: TouchBarItem) -> Self {
        self.items.push(item);
        self
    }

    /// Build the touch bar
    pub fn build(self) -> Result<TouchBar> {
        let mut bar = TouchBar::new()?;
        for item in self.items {
            bar.add_item(item);
        }
        Ok(bar)
    }
}

impl Default for TouchBarBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ACCESSIBILITY
// ============================================================================

/// Accessibility features for components
pub struct AccessibilityOptions {
    label: String,
    help_text: Option<String>,
    enabled: bool,
}

impl AccessibilityOptions {
    /// Create a new accessibility options builder
    pub fn builder() -> AccessibilityBuilder {
        AccessibilityBuilder::new()
    }

    /// Create new accessibility options
    pub fn new(label: impl Into<String>) -> Result<Self> {
        Ok(AccessibilityOptions {
            label: label.into(),
            help_text: None,
            enabled: true,
        })
    }

    /// Get the label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the help text
    pub fn help_text(&self) -> Option<&str> {
        self.help_text.as_deref()
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Builder for AccessibilityOptions
pub struct AccessibilityBuilder {
    label: String,
    help_text: Option<String>,
    enabled: bool,
}

impl AccessibilityBuilder {
    /// Create a new accessibility builder
    pub fn new() -> Self {
        Self {
            label: String::new(),
            help_text: None,
            enabled: true,
        }
    }

    /// Set the label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set the help text
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Build the accessibility options
    pub fn build(self) -> Result<AccessibilityOptions> {
        Ok(AccessibilityOptions {
            label: self.label,
            help_text: self.help_text,
            enabled: self.enabled,
        })
    }
}

impl Default for AccessibilityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DARK MODE
// ============================================================================

/// Dark mode appearance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    /// Light appearance
    Light,
    /// Dark appearance
    Dark,
    /// System default
    Auto,
}

/// Dark mode support
pub struct DarkModeManager {
    appearance: Appearance,
}

impl DarkModeManager {
    /// Create a new dark mode manager
    pub fn new(appearance: Appearance) -> Result<Self> {
        Ok(DarkModeManager { appearance })
    }

    /// Get the current appearance
    pub fn appearance(&self) -> Appearance {
        self.appearance
    }

    /// Set the appearance
    pub fn set_appearance(&mut self, appearance: Appearance) -> Result<()> {
        self.appearance = appearance;
        Ok(())
    }

    /// Check if dark mode is enabled
    pub fn is_dark(&self) -> bool {
        self.appearance == Appearance::Dark
    }
}

// ============================================================================
// DRAG & DROP
// ============================================================================

/// Drag and drop support
pub struct DragDropManager {
    enabled: bool,
    allowed_types: Vec<String>,
}

impl DragDropManager {
    /// Create a new drag and drop manager
    pub fn new() -> Result<Self> {
        Ok(DragDropManager {
            enabled: false,
            allowed_types: Vec::new(),
        })
    }

    /// Enable drag and drop
    pub fn enable(&mut self) -> Result<()> {
        self.enabled = true;
        Ok(())
    }

    /// Disable drag and drop
    pub fn disable(&mut self) -> Result<()> {
        self.enabled = false;
        Ok(())
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Add an allowed type
    pub fn add_allowed_type(&mut self, type_name: impl Into<String>) {
        self.allowed_types.push(type_name.into());
    }

    /// Get allowed types
    pub fn allowed_types(&self) -> &[String] {
        &self.allowed_types
    }
}

// ============================================================================
// ADVANCED STYLING
// ============================================================================

/// Advanced styling options
pub struct AdvancedStyling {
    corner_radius: f64,
    shadow_enabled: bool,
    shadow_opacity: f64,
    border_width: f64,
}

impl AdvancedStyling {
    /// Create a new advanced styling builder
    pub fn builder() -> AdvancedStylingBuilder {
        AdvancedStylingBuilder::new()
    }

    /// Create new advanced styling
    pub fn new() -> Result<Self> {
        Ok(AdvancedStyling {
            corner_radius: 0.0,
            shadow_enabled: false,
            shadow_opacity: 0.5,
            border_width: 0.0,
        })
    }

    /// Get corner radius
    pub fn corner_radius(&self) -> f64 {
        self.corner_radius
    }

    /// Check if shadow is enabled
    pub fn shadow_enabled(&self) -> bool {
        self.shadow_enabled
    }

    /// Get shadow opacity
    pub fn shadow_opacity(&self) -> f64 {
        self.shadow_opacity
    }

    /// Get border width
    pub fn border_width(&self) -> f64 {
        self.border_width
    }
}

/// Builder for AdvancedStyling
pub struct AdvancedStylingBuilder {
    corner_radius: f64,
    shadow_enabled: bool,
    shadow_opacity: f64,
    border_width: f64,
}

impl AdvancedStylingBuilder {
    /// Create a new advanced styling builder
    pub fn new() -> Self {
        Self {
            corner_radius: 0.0,
            shadow_enabled: false,
            shadow_opacity: 0.5,
            border_width: 0.0,
        }
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f64) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Enable shadow
    pub fn shadow(mut self, enabled: bool) -> Self {
        self.shadow_enabled = enabled;
        self
    }

    /// Set shadow opacity
    pub fn shadow_opacity(mut self, opacity: f64) -> Self {
        self.shadow_opacity = opacity;
        self
    }

    /// Set border width
    pub fn border_width(mut self, width: f64) -> Self {
        self.border_width = width;
        self
    }

    /// Build the advanced styling
    pub fn build(self) -> Result<AdvancedStyling> {
        Ok(AdvancedStyling {
            corner_radius: self.corner_radius,
            shadow_enabled: self.shadow_enabled,
            shadow_opacity: self.shadow_opacity,
            border_width: self.border_width,
        })
    }
}

impl Default for AdvancedStylingBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // GridView Tests
    #[test]
    fn test_grid_view_creation() {
        let grid = GridView::new(3, 4).unwrap();
        assert_eq!(grid.columns(), 3);
        assert_eq!(grid.rows(), 4);
    }

    #[test]
    fn test_grid_view_builder() {
        let grid = GridViewBuilder::new()
            .columns(2)
            .rows(2)
            .build()
            .unwrap();
        
        assert_eq!(grid.columns(), 2);
        assert_eq!(grid.rows(), 2);
    }

    #[test]
    fn test_grid_view_invalid() {
        let result = GridView::new(0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_grid_view_set_spacing() {
        let mut grid = GridView::new(2, 2).unwrap();
        grid.set_spacing(15.0).unwrap();
        assert_eq!(grid.spacing(), 15.0);
    }

    // TouchBar Tests
    #[test]
    fn test_touch_bar_item_creation() {
        let item = TouchBarItem::new("id1", "Label");
        assert_eq!(item.identifier(), "id1");
        assert_eq!(item.label(), "Label");
    }

    #[test]
    fn test_touch_bar_creation() {
        let bar = TouchBar::new().unwrap();
        assert_eq!(bar.items().len(), 0);
    }

    #[test]
    fn test_touch_bar_add_item() {
        let mut bar = TouchBar::new().unwrap();
        let item = TouchBarItem::new("id", "Label");
        bar.add_item(item);
        assert_eq!(bar.items().len(), 1);
    }

    #[test]
    fn test_touch_bar_builder() {
        let bar = TouchBarBuilder::new()
            .item(TouchBarItem::new("id1", "Item 1"))
            .item(TouchBarItem::new("id2", "Item 2"))
            .build()
            .unwrap();
        
        assert_eq!(bar.items().len(), 2);
    }

    // Accessibility Tests
    #[test]
    fn test_accessibility_creation() {
        let acc = AccessibilityOptions::new("Button").unwrap();
        assert_eq!(acc.label(), "Button");
        assert!(acc.is_enabled());
    }

    #[test]
    fn test_accessibility_builder() {
        let acc = AccessibilityBuilder::new()
            .label("Click me")
            .help_text("This is a button")
            .build()
            .unwrap();
        
        assert_eq!(acc.label(), "Click me");
        assert!(acc.help_text().is_some());
    }

    // DarkMode Tests
    #[test]
    fn test_dark_mode_light() {
        let dm = DarkModeManager::new(Appearance::Light).unwrap();
        assert!(!dm.is_dark());
    }

    #[test]
    fn test_dark_mode_dark() {
        let dm = DarkModeManager::new(Appearance::Dark).unwrap();
        assert!(dm.is_dark());
    }

    #[test]
    fn test_dark_mode_set_appearance() {
        let mut dm = DarkModeManager::new(Appearance::Light).unwrap();
        dm.set_appearance(Appearance::Dark).unwrap();
        assert!(dm.is_dark());
    }

    // DragDrop Tests
    #[test]
    fn test_drag_drop_creation() {
        let dd = DragDropManager::new().unwrap();
        assert!(!dd.is_enabled());
    }

    #[test]
    fn test_drag_drop_enable() {
        let mut dd = DragDropManager::new().unwrap();
        dd.enable().unwrap();
        assert!(dd.is_enabled());
    }

    #[test]
    fn test_drag_drop_add_type() {
        let mut dd = DragDropManager::new().unwrap();
        dd.add_allowed_type("text");
        dd.add_allowed_type("image");
        assert_eq!(dd.allowed_types().len(), 2);
    }

    // AdvancedStyling Tests
    #[test]
    fn test_advanced_styling_creation() {
        let style = AdvancedStyling::new().unwrap();
        assert_eq!(style.corner_radius(), 0.0);
        assert!(!style.shadow_enabled());
    }

    #[test]
    fn test_advanced_styling_builder() {
        let style = AdvancedStylingBuilder::new()
            .corner_radius(8.0)
            .shadow(true)
            .shadow_opacity(0.7)
            .border_width(1.0)
            .build()
            .unwrap();
        
        assert_eq!(style.corner_radius(), 8.0);
        assert!(style.shadow_enabled());
        assert_eq!(style.shadow_opacity(), 0.7);
        assert_eq!(style.border_width(), 1.0);
    }

    #[test]
    fn test_advanced_styling_default() {
        let style = AdvancedStylingBuilder::default().build().unwrap();
        assert_eq!(style.corner_radius(), 0.0);
    }
}
