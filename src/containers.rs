//! Container views for organizing content in macOS GUI applications
//!
//! Includes ScrollView, TabView, SplitView, and GroupBox containers.

use crate::error::Result;

// ============================================================================
// SCROLL VIEW
// ============================================================================

/// A scroll view container for scrollable content
pub struct ScrollView {
    width: f64,
    height: f64,
    scrollable_width: f64,
    scrollable_height: f64,
}

impl ScrollView {
    /// Create a new scroll view builder
    pub fn builder() -> ScrollViewBuilder {
        ScrollViewBuilder::new()
    }

    /// Create a new scroll view
    pub fn new(width: f64, height: f64) -> Result<Self> {
        Ok(ScrollView {
            width,
            height,
            scrollable_width: width,
            scrollable_height: height,
        })
    }

    /// Get the scroll view dimensions
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    /// Get the scrollable content dimensions
    pub fn scrollable_size(&self) -> (f64, f64) {
        (self.scrollable_width, self.scrollable_height)
    }
}

/// Builder for ScrollView
pub struct ScrollViewBuilder {
    width: f64,
    height: f64,
    scrollable_width: f64,
    scrollable_height: f64,
}

impl ScrollViewBuilder {
    /// Create a new scroll view builder
    pub fn new() -> Self {
        Self {
            width: 400.0,
            height: 300.0,
            scrollable_width: 400.0,
            scrollable_height: 600.0,
        }
    }

    /// Set the scroll view size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the scrollable content size
    pub fn content_size(mut self, width: f64, height: f64) -> Self {
        self.scrollable_width = width;
        self.scrollable_height = height;
        self
    }

    /// Build the scroll view
    pub fn build(self) -> Result<ScrollView> {
        Ok(ScrollView {
            width: self.width,
            height: self.height,
            scrollable_width: self.scrollable_width,
            scrollable_height: self.scrollable_height,
        })
    }
}

impl Default for ScrollViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TAB VIEW
// ============================================================================

/// A tab view container for tabbed interfaces
pub struct TabView {
    tabs: Vec<String>,
    selected_tab: usize,
}

impl TabView {
    /// Create a new tab view builder
    pub fn builder() -> TabViewBuilder {
        TabViewBuilder::new()
    }

    /// Create a new tab view
    pub fn new(tabs: Vec<String>) -> Result<Self> {
        if tabs.is_empty() {
            return Err(crate::error::CocoanutError::InvalidParameter(
                "Tabs cannot be empty".to_string()
            ));
        }
        Ok(TabView {
            tabs,
            selected_tab: 0,
        })
    }

    /// Get the tabs
    pub fn tabs(&self) -> &[String] {
        &self.tabs
    }

    /// Get the selected tab index
    pub fn selected_tab(&self) -> usize {
        self.selected_tab
    }

    /// Set the selected tab
    pub fn set_selected_tab(&mut self, index: usize) -> Result<()> {
        if index < self.tabs.len() {
            self.selected_tab = index;
            Ok(())
        } else {
            Err(crate::error::CocoanutError::InvalidParameter(
                format!("Tab index {} out of bounds", index)
            ))
        }
    }
}

/// Builder for TabView
pub struct TabViewBuilder {
    tabs: Vec<String>,
    selected_tab: usize,
}

impl TabViewBuilder {
    /// Create a new tab view builder
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            selected_tab: 0,
        }
    }

    /// Add a tab
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.tabs.push(label.into());
        self
    }

    /// Set the selected tab
    pub fn selected(mut self, index: usize) -> Self {
        self.selected_tab = index;
        self
    }

    /// Build the tab view
    pub fn build(self) -> Result<TabView> {
        TabView::new(self.tabs)
    }
}

impl Default for TabViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SPLIT VIEW
// ============================================================================

/// A split view container for resizable panes
pub struct SplitView {
    orientation: SplitOrientation,
    divider_position: f64,
}

/// Split view orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitOrientation {
    /// Vertical split (left/right panes)
    Vertical,
    /// Horizontal split (top/bottom panes)
    Horizontal,
}

impl SplitView {
    /// Create a new split view builder
    pub fn builder() -> SplitViewBuilder {
        SplitViewBuilder::new()
    }

    /// Create a new split view
    pub fn new(orientation: SplitOrientation) -> Result<Self> {
        Ok(SplitView {
            orientation,
            divider_position: 0.5,
        })
    }

    /// Get the orientation
    pub fn orientation(&self) -> SplitOrientation {
        self.orientation
    }

    /// Get the divider position (0.0 to 1.0)
    pub fn divider_position(&self) -> f64 {
        self.divider_position
    }

    /// Set the divider position
    pub fn set_divider_position(&mut self, position: f64) -> Result<()> {
        if position >= 0.0 && position <= 1.0 {
            self.divider_position = position;
            Ok(())
        } else {
            Err(crate::error::CocoanutError::InvalidParameter(
                "Divider position must be between 0.0 and 1.0".to_string()
            ))
        }
    }
}

/// Builder for SplitView
pub struct SplitViewBuilder {
    orientation: SplitOrientation,
    divider_position: f64,
}

impl SplitViewBuilder {
    /// Create a new split view builder
    pub fn new() -> Self {
        Self {
            orientation: SplitOrientation::Vertical,
            divider_position: 0.5,
        }
    }

    /// Set the orientation
    pub fn orientation(mut self, orientation: SplitOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the divider position
    pub fn divider_position(mut self, position: f64) -> Self {
        self.divider_position = position;
        self
    }

    /// Build the split view
    pub fn build(self) -> Result<SplitView> {
        Ok(SplitView {
            orientation: self.orientation,
            divider_position: self.divider_position,
        })
    }
}

impl Default for SplitViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// GROUP BOX
// ============================================================================

/// A group box container for grouping related controls
pub struct GroupBox {
    title: String,
}

impl GroupBox {
    /// Create a new group box builder
    pub fn builder() -> GroupBoxBuilder {
        GroupBoxBuilder::new()
    }

    /// Create a new group box
    pub fn new(title: &str) -> Result<Self> {
        Ok(GroupBox {
            title: title.to_string(),
        })
    }

    /// Get the group box title
    pub fn title(&self) -> &str {
        &self.title
    }
}

/// Builder for GroupBox
pub struct GroupBoxBuilder {
    title: String,
}

impl GroupBoxBuilder {
    /// Create a new group box builder
    pub fn new() -> Self {
        Self {
            title: String::new(),
        }
    }

    /// Set the group box title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Build the group box
    pub fn build(self) -> Result<GroupBox> {
        Ok(GroupBox {
            title: self.title,
        })
    }
}

impl Default for GroupBoxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ScrollView Tests
    #[test]
    fn test_scroll_view_creation() {
        let scroll = ScrollView::new(400.0, 300.0).unwrap();
        assert_eq!(scroll.size(), (400.0, 300.0));
    }

    #[test]
    fn test_scroll_view_builder() {
        let scroll = ScrollViewBuilder::new()
            .size(500.0, 400.0)
            .content_size(500.0, 800.0)
            .build()
            .unwrap();
        
        assert_eq!(scroll.size(), (500.0, 400.0));
        assert_eq!(scroll.scrollable_size(), (500.0, 800.0));
    }

    // TabView Tests
    #[test]
    fn test_tab_view_creation() {
        let tabs = TabView::new(vec!["Tab 1".to_string(), "Tab 2".to_string()]).unwrap();
        assert_eq!(tabs.tabs().len(), 2);
        assert_eq!(tabs.selected_tab(), 0);
    }

    #[test]
    fn test_tab_view_builder() {
        let tabs = TabViewBuilder::new()
            .tab("General")
            .tab("Advanced")
            .tab("About")
            .build()
            .unwrap();
        
        assert_eq!(tabs.tabs().len(), 3);
        assert_eq!(tabs.selected_tab(), 0);
    }

    #[test]
    fn test_tab_view_set_selected() {
        let mut tabs = TabView::new(vec!["A".to_string(), "B".to_string()]).unwrap();
        tabs.set_selected_tab(1).unwrap();
        assert_eq!(tabs.selected_tab(), 1);
    }

    // SplitView Tests
    #[test]
    fn test_split_view_creation() {
        let split = SplitView::new(SplitOrientation::Vertical).unwrap();
        assert_eq!(split.orientation(), SplitOrientation::Vertical);
    }

    #[test]
    fn test_split_view_builder() {
        let split = SplitViewBuilder::new()
            .orientation(SplitOrientation::Horizontal)
            .divider_position(0.3)
            .build()
            .unwrap();
        
        assert_eq!(split.orientation(), SplitOrientation::Horizontal);
        assert_eq!(split.divider_position(), 0.3);
    }

    #[test]
    fn test_split_view_set_divider() {
        let mut split = SplitView::new(SplitOrientation::Vertical).unwrap();
        split.set_divider_position(0.7).unwrap();
        assert_eq!(split.divider_position(), 0.7);
    }

    // GroupBox Tests
    #[test]
    fn test_group_box_creation() {
        let group = GroupBox::new("Settings").unwrap();
        assert_eq!(group.title(), "Settings");
    }

    #[test]
    fn test_group_box_builder() {
        let group = GroupBoxBuilder::new()
            .title("Preferences")
            .build()
            .unwrap();
        
        assert_eq!(group.title(), "Preferences");
    }
}
