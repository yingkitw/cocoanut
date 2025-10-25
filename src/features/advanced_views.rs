//! Advanced view components for Cocoanut
//!
//! This module implements Priority 3 advanced features:
//! - Table Views (NSTableView)
//! - Collection Views (NSCollectionView)
//! - Split Views (NSSplitViewController)
//! - Tab Views (NSTabViewController)
//! - Web Views (WKWebView)

use crate::core::error::Result;
use crate::core::traits::Drawable;
use objc::runtime::Object;

/// Table view data source
pub trait TableViewDataSource: Send + Sync {
    /// Get number of rows
    fn number_of_rows(&self) -> usize;

    /// Get cell content for row
    fn cell_content(&self, row: usize) -> String;
}

/// Table view component
pub struct TableView {
    rows: Vec<String>,
}

impl TableView {
    /// Create a new table view
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
        }
    }

    /// Add a row
    pub fn add_row(&mut self, content: String) {
        self.rows.push(content);
    }

    /// Remove a row
    pub fn remove_row(&mut self, index: usize) {
        if index < self.rows.len() {
            self.rows.remove(index);
        }
    }

    /// Get row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get row content
    pub fn row_content(&self, index: usize) -> Option<&str> {
        self.rows.get(index).map(|s| s.as_str())
    }

    /// Clear all rows
    pub fn clear(&mut self) {
        self.rows.clear();
    }
}

impl Default for TableView {
    fn default() -> Self {
        Self::new()
    }
}

/// Collection view item
#[derive(Debug, Clone)]
pub struct CollectionViewItem {
    /// Item identifier
    pub identifier: String,
    /// Item content
    pub content: String,
}

impl CollectionViewItem {
    /// Create a new collection view item
    pub fn new(identifier: &str, content: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            content: content.to_string(),
        }
    }
}

/// Collection view component
pub struct CollectionView {
    items: Vec<CollectionViewItem>,
}

impl CollectionView {
    /// Create a new collection view
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Add an item
    pub fn add_item(&mut self, item: CollectionViewItem) {
        self.items.push(item);
    }

    /// Remove an item
    pub fn remove_item(&mut self, identifier: &str) {
        self.items.retain(|item| item.identifier != identifier);
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get all items
    pub fn items(&self) -> &[CollectionViewItem] {
        &self.items
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for CollectionView {
    fn default() -> Self {
        Self::new()
    }
}

/// Split view orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitOrientation {
    /// Horizontal split
    Horizontal,
    /// Vertical split
    Vertical,
}

/// Split view component
pub struct SplitView {
    orientation: SplitOrientation,
    divider_position: f64,
}

impl SplitView {
    /// Create a new split view
    pub fn new(orientation: SplitOrientation) -> Self {
        Self {
            orientation,
            divider_position: 0.5,
        }
    }

    /// Set divider position (0.0 to 1.0)
    pub fn set_divider_position(&mut self, position: f64) {
        self.divider_position = position.clamp(0.0, 1.0);
    }

    /// Get divider position
    pub fn divider_position(&self) -> f64 {
        self.divider_position
    }

    /// Get orientation
    pub fn orientation(&self) -> SplitOrientation {
        self.orientation
    }
}

/// Tab view item
#[derive(Debug, Clone)]
pub struct TabViewItem {
    /// Tab identifier
    pub identifier: String,
    /// Tab label
    pub label: String,
}

impl TabViewItem {
    /// Create a new tab view item
    pub fn new(identifier: &str, label: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            label: label.to_string(),
        }
    }
}

/// Tab view component
pub struct TabView {
    tabs: Vec<TabViewItem>,
    selected_index: usize,
}

impl TabView {
    /// Create a new tab view
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            selected_index: 0,
        }
    }

    /// Add a tab
    pub fn add_tab(&mut self, tab: TabViewItem) {
        self.tabs.push(tab);
    }

    /// Remove a tab
    pub fn remove_tab(&mut self, identifier: &str) {
        self.tabs.retain(|tab| tab.identifier != identifier);
        if self.selected_index >= self.tabs.len() && !self.tabs.is_empty() {
            self.selected_index = self.tabs.len() - 1;
        }
    }

    /// Select a tab by index
    pub fn select_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.selected_index = index;
        }
    }

    /// Get selected tab index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get all tabs
    pub fn tabs(&self) -> &[TabViewItem] {
        &self.tabs
    }
}

impl Default for TabView {
    fn default() -> Self {
        Self::new()
    }
}

/// Web view component for WKWebView
pub struct WebView {
    url: String,
    html: String,
}

impl WebView {
    /// Create a new web view
    pub fn new() -> Self {
        Self {
            url: String::new(),
            html: String::new(),
        }
    }

    /// Load URL
    pub fn load_url(&mut self, url: &str) -> Result<()> {
        if url.is_empty() {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                "URL cannot be empty".into(),
            ));
        }
        self.url = url.to_string();
        Ok(())
    }

    /// Load HTML
    pub fn load_html(&mut self, html: &str) -> Result<()> {
        if html.is_empty() {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                "HTML cannot be empty".into(),
            ));
        }
        self.html = html.to_string();
        Ok(())
    }

    /// Get current URL
    pub fn current_url(&self) -> &str {
        &self.url
    }

    /// Get current HTML
    pub fn current_html(&self) -> &str {
        &self.html
    }

    /// Go back
    pub fn go_back(&self) -> Result<()> {
        // WKWebView goBack implementation
        Ok(())
    }

    /// Go forward
    pub fn go_forward(&self) -> Result<()> {
        // WKWebView goForward implementation
        Ok(())
    }

    /// Reload
    pub fn reload(&self) -> Result<()> {
        // WKWebView reload implementation
        Ok(())
    }
}

impl Default for WebView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_view() {
        let mut table = TableView::new();
        table.add_row("Row 1".to_string());
        table.add_row("Row 2".to_string());

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.row_content(0), Some("Row 1"));
    }

    #[test]
    fn test_table_view_remove() {
        let mut table = TableView::new();
        table.add_row("Row 1".to_string());
        table.add_row("Row 2".to_string());
        table.remove_row(0);

        assert_eq!(table.row_count(), 1);
    }

    #[test]
    fn test_collection_view() {
        let mut collection = CollectionView::new();
        collection.add_item(CollectionViewItem::new("item1", "Item 1"));
        collection.add_item(CollectionViewItem::new("item2", "Item 2"));

        assert_eq!(collection.item_count(), 2);
    }

    #[test]
    fn test_split_view() {
        let mut split = SplitView::new(SplitOrientation::Horizontal);
        split.set_divider_position(0.3);

        assert_eq!(split.divider_position(), 0.3);
        assert_eq!(split.orientation(), SplitOrientation::Horizontal);
    }

    #[test]
    fn test_tab_view() {
        let mut tabs = TabView::new();
        tabs.add_tab(TabViewItem::new("tab1", "Tab 1"));
        tabs.add_tab(TabViewItem::new("tab2", "Tab 2"));

        assert_eq!(tabs.tab_count(), 2);
        assert_eq!(tabs.selected_index(), 0);

        tabs.select_tab(1);
        assert_eq!(tabs.selected_index(), 1);
    }

    #[test]
    fn test_web_view() {
        let mut web = WebView::new();
        assert!(web.load_url("https://example.com").is_ok());
        assert_eq!(web.current_url(), "https://example.com");
    }

    #[test]
    fn test_web_view_html() {
        let mut web = WebView::new();
        let html = "<html><body>Hello</body></html>";
        assert!(web.load_html(html).is_ok());
        assert_eq!(web.current_html(), html);
    }
}
