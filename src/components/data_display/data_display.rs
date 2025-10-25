//! Data display components for macOS GUI applications
//!
//! Includes TableView, OutlineView, and CollectionView for displaying data.

use crate::core::error::Result;

// ============================================================================
// TABLE VIEW
// ============================================================================

/// A table view for displaying tabular data
pub struct TableView {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl TableView {
    /// Create a new table view builder
    pub fn builder() -> TableViewBuilder {
        TableViewBuilder::new()
    }

    /// Create a new table view with columns
    pub fn new(columns: Vec<String>) -> Result<Self> {
        if columns.is_empty() {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                "Columns cannot be empty".to_string()
            ));
        }
        Ok(TableView {
            columns,
            rows: Vec::new(),
        })
    }

    /// Get the columns
    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    /// Get the rows
    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    /// Add a row
    pub fn add_row(&mut self, row: Vec<String>) -> Result<()> {
        if row.len() != self.columns.len() {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                format!("Row length {} doesn't match column count {}", row.len(), self.columns.len())
            ));
        }
        self.rows.push(row);
        Ok(())
    }

    /// Get row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// Builder for TableView
pub struct TableViewBuilder {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl TableViewBuilder {
    /// Create a new table view builder
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Add a column
    pub fn column(mut self, name: impl Into<String>) -> Self {
        self.columns.push(name.into());
        self
    }

    /// Add a row
    pub fn row(mut self, row: Vec<String>) -> Self {
        self.rows.push(row);
        self
    }

    /// Build the table view
    pub fn build(self) -> Result<TableView> {
        TableView::new(self.columns)
    }
}

impl Default for TableViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// OUTLINE VIEW
// ============================================================================

/// An outline view for displaying hierarchical data
pub struct OutlineView {
    root_items: Vec<OutlineItem>,
}

/// An item in an outline view
#[derive(Debug, Clone)]
pub struct OutlineItem {
    title: String,
    children: Vec<OutlineItem>,
}

impl OutlineItem {
    /// Create a new outline item
    pub fn new(title: impl Into<String>) -> Self {
        OutlineItem {
            title: title.into(),
            children: Vec::new(),
        }
    }

    /// Add a child item
    pub fn add_child(&mut self, child: OutlineItem) {
        self.children.push(child);
    }

    /// Get the title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the children
    pub fn children(&self) -> &[OutlineItem] {
        &self.children
    }
}

impl OutlineView {
    /// Create a new outline view builder
    pub fn builder() -> OutlineViewBuilder {
        OutlineViewBuilder::new()
    }

    /// Create a new outline view
    pub fn new() -> Result<Self> {
        Ok(OutlineView {
            root_items: Vec::new(),
        })
    }

    /// Add a root item
    pub fn add_item(&mut self, item: OutlineItem) {
        self.root_items.push(item);
    }

    /// Get root items
    pub fn items(&self) -> &[OutlineItem] {
        &self.root_items
    }
}

/// Builder for OutlineView
pub struct OutlineViewBuilder {
    root_items: Vec<OutlineItem>,
}

impl OutlineViewBuilder {
    /// Create a new outline view builder
    pub fn new() -> Self {
        Self {
            root_items: Vec::new(),
        }
    }

    /// Add a root item
    pub fn item(mut self, item: OutlineItem) -> Self {
        self.root_items.push(item);
        self
    }

    /// Build the outline view
    pub fn build(self) -> Result<OutlineView> {
        let mut view = OutlineView::new()?;
        for item in self.root_items {
            view.add_item(item);
        }
        Ok(view)
    }
}

impl Default for OutlineViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// COLLECTION VIEW
// ============================================================================

/// A collection view for displaying items in a grid layout
pub struct CollectionView {
    items: Vec<String>,
    columns: usize,
}

impl CollectionView {
    /// Create a new collection view builder
    pub fn builder() -> CollectionViewBuilder {
        CollectionViewBuilder::new()
    }

    /// Create a new collection view
    pub fn new(columns: usize) -> Result<Self> {
        if columns == 0 {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                "Columns must be greater than 0".to_string()
            ));
        }
        Ok(CollectionView {
            items: Vec::new(),
            columns,
        })
    }

    /// Add an item
    pub fn add_item(&mut self, item: impl Into<String>) {
        self.items.push(item.into());
    }

    /// Get items
    pub fn items(&self) -> &[String] {
        &self.items
    }

    /// Get column count
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

/// Builder for CollectionView
pub struct CollectionViewBuilder {
    items: Vec<String>,
    columns: usize,
}

impl CollectionViewBuilder {
    /// Create a new collection view builder
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            columns: 3,
        }
    }

    /// Set the number of columns
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

    /// Add an item
    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Build the collection view
    pub fn build(self) -> Result<CollectionView> {
        let mut view = CollectionView::new(self.columns)?;
        for item in self.items {
            view.add_item(item);
        }
        Ok(view)
    }
}

impl Default for CollectionViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TableView Tests
    #[test]
    fn test_table_view_creation() {
        let table = TableView::new(vec!["Name".to_string(), "Age".to_string()]).unwrap();
        assert_eq!(table.columns().len(), 2);
        assert_eq!(table.row_count(), 0);
    }

    #[test]
    fn test_table_view_add_row() {
        let mut table = TableView::new(vec!["Name".to_string(), "Age".to_string()]).unwrap();
        table.add_row(vec!["Alice".to_string(), "30".to_string()]).unwrap();
        assert_eq!(table.row_count(), 1);
    }

    #[test]
    fn test_table_view_builder() {
        let table = TableViewBuilder::new()
            .column("Name")
            .column("Email")
            .build()
            .unwrap();
        
        assert_eq!(table.columns().len(), 2);
    }

    #[test]
    fn test_table_view_invalid_row() {
        let mut table = TableView::new(vec!["A".to_string(), "B".to_string()]).unwrap();
        let result = table.add_row(vec!["X".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_table_view_empty_columns() {
        let result = TableView::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_table_view_multiple_rows() {
        let mut table = TableView::new(vec!["A".to_string(), "B".to_string()]).unwrap();
        table.add_row(vec!["1".to_string(), "2".to_string()]).unwrap();
        table.add_row(vec!["3".to_string(), "4".to_string()]).unwrap();
        table.add_row(vec!["5".to_string(), "6".to_string()]).unwrap();
        assert_eq!(table.row_count(), 3);
    }

    #[test]
    fn test_table_view_get_rows() {
        let mut table = TableView::new(vec!["A".to_string()]).unwrap();
        table.add_row(vec!["Value".to_string()]).unwrap();
        assert_eq!(table.rows().len(), 1);
    }

    // OutlineView Tests
    #[test]
    fn test_outline_item_creation() {
        let item = OutlineItem::new("Root");
        assert_eq!(item.title(), "Root");
        assert_eq!(item.children().len(), 0);
    }

    #[test]
    fn test_outline_item_add_child() {
        let mut item = OutlineItem::new("Parent");
        let child = OutlineItem::new("Child");
        item.add_child(child);
        assert_eq!(item.children().len(), 1);
    }

    #[test]
    fn test_outline_view_creation() {
        let view = OutlineView::new().unwrap();
        assert_eq!(view.items().len(), 0);
    }

    #[test]
    fn test_outline_view_add_item() {
        let mut view = OutlineView::new().unwrap();
        let item = OutlineItem::new("Item");
        view.add_item(item);
        assert_eq!(view.items().len(), 1);
    }

    #[test]
    fn test_outline_view_builder() {
        let item1 = OutlineItem::new("Item 1");
        let item2 = OutlineItem::new("Item 2");
        
        let view = OutlineViewBuilder::new()
            .item(item1)
            .item(item2)
            .build()
            .unwrap();
        
        assert_eq!(view.items().len(), 2);
    }

    // CollectionView Tests
    #[test]
    fn test_collection_view_creation() {
        let view = CollectionView::new(3).unwrap();
        assert_eq!(view.columns(), 3);
        assert_eq!(view.item_count(), 0);
    }

    #[test]
    fn test_collection_view_add_item() {
        let mut view = CollectionView::new(2).unwrap();
        view.add_item("Item 1");
        view.add_item("Item 2");
        assert_eq!(view.item_count(), 2);
    }

    #[test]
    fn test_collection_view_builder() {
        let view = CollectionViewBuilder::new()
            .columns(4)
            .item("A")
            .item("B")
            .item("C")
            .build()
            .unwrap();
        
        assert_eq!(view.columns(), 4);
        assert_eq!(view.item_count(), 3);
    }

    #[test]
    fn test_collection_view_invalid_columns() {
        let result = CollectionView::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_collection_view_get_items() {
        let mut view = CollectionView::new(2).unwrap();
        view.add_item("Item");
        assert_eq!(view.items().len(), 1);
    }

    #[test]
    fn test_collection_view_multiple_items() {
        let mut view = CollectionView::new(3).unwrap();
        for i in 0..10 {
            view.add_item(format!("Item {}", i));
        }
        assert_eq!(view.item_count(), 10);
    }
}
