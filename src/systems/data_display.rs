//! Phase 1: Data Display Elements
//! 
//! Implements data visualization components for tables, metrics, and basic data display.

use crate::core::error::Result;
use std::collections::HashMap;

/// Static table display element
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    use_container_width: bool,
}

impl Table {
    /// Create a new table with headers
    pub fn new(headers: Vec<String>) -> Self {
        Table {
            headers,
            rows: Vec::new(),
            use_container_width: false,
        }
    }

    /// Add a row to the table
    pub fn add_row(&mut self, row: Vec<String>) -> Result<()> {
        if row.len() != self.headers.len() {
            return Err("Row length must match header count".into());
        }
        self.rows.push(row);
        Ok(())
    }

    /// Set whether to use container width
    pub fn use_container_width(mut self, use_width: bool) -> Self {
        self.use_container_width = use_width;
        self
    }

    /// Get table headers
    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    /// Get table rows
    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    /// Get row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Check if using container width
    pub fn uses_container_width(&self) -> bool {
        self.use_container_width
    }
}

/// Interactive DataFrame display element
pub struct DataFrame {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    editable: bool,
    use_container_width: bool,
    height: Option<f64>,
}

impl DataFrame {
    /// Create a new DataFrame display
    pub fn new(headers: Vec<String>) -> Self {
        DataFrame {
            headers,
            rows: Vec::new(),
            editable: false,
            use_container_width: false,
            height: None,
        }
    }

    /// Add a row to the DataFrame
    pub fn add_row(&mut self, row: Vec<String>) -> Result<()> {
        if row.len() != self.headers.len() {
            return Err("Row length must match header count".into());
        }
        self.rows.push(row);
        Ok(())
    }

    /// Set whether the DataFrame is editable
    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Set container width usage
    pub fn use_container_width(mut self, use_width: bool) -> Self {
        self.use_container_width = use_width;
        self
    }

    /// Set the height of the DataFrame
    pub fn with_height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Get headers
    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    /// Get rows
    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    /// Check if editable
    pub fn is_editable(&self) -> bool {
        self.editable
    }

    /// Get height
    pub fn get_height(&self) -> Option<f64> {
        self.height
    }
}

/// Editable DataFrame widget
pub struct DataEditor {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    num_rows: usize,
    use_container_width: bool,
    height: Option<f64>,
}

impl DataEditor {
    /// Create a new data editor
    pub fn new(headers: Vec<String>) -> Self {
        DataEditor {
            headers,
            rows: Vec::new(),
            num_rows: 10,
            use_container_width: false,
            height: None,
        }
    }

    /// Set number of rows to display
    pub fn with_num_rows(mut self, num: usize) -> Self {
        self.num_rows = num;
        self
    }

    /// Add a row
    pub fn add_row(&mut self, row: Vec<String>) -> Result<()> {
        if row.len() != self.headers.len() {
            return Err("Row length must match header count".into());
        }
        self.rows.push(row);
        Ok(())
    }

    /// Set container width usage
    pub fn use_container_width(mut self, use_width: bool) -> Self {
        self.use_container_width = use_width;
        self
    }

    /// Set height
    pub fn with_height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Get headers
    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    /// Get rows
    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    /// Get number of rows to display
    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }
}

/// KPI metric display with delta
pub struct Metric {
    label: String,
    value: String,
    delta: Option<String>,
    delta_color: Option<String>,
}

impl Metric {
    /// Create a new metric
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Metric {
            label: label.into(),
            value: value.into(),
            delta: None,
            delta_color: None,
        }
    }

    /// Set the delta (change) value
    pub fn with_delta(mut self, delta: impl Into<String>) -> Self {
        self.delta = Some(delta.into());
        self
    }

    /// Set the delta color (e.g., "green", "red", "gray")
    pub fn with_delta_color(mut self, color: impl Into<String>) -> Self {
        self.delta_color = Some(color.into());
        self
    }

    /// Get the label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the delta
    pub fn get_delta(&self) -> Option<&str> {
        self.delta.as_deref()
    }

    /// Get the delta color
    pub fn get_delta_color(&self) -> Option<&str> {
        self.delta_color.as_deref()
    }
}

/// Column layout for metrics
pub struct MetricColumn {
    metrics: Vec<Metric>,
}

impl MetricColumn {
    /// Create a new metric column
    pub fn new() -> Self {
        MetricColumn {
            metrics: Vec::new(),
        }
    }

    /// Add a metric to the column
    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    /// Get all metrics
    pub fn metrics(&self) -> &[Metric] {
        &self.metrics
    }

    /// Get metric count
    pub fn metric_count(&self) -> usize {
        self.metrics.len()
    }
}

impl Default for MetricColumn {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new(vec!["Name".to_string(), "Age".to_string()]);
        assert_eq!(table.headers().len(), 2);
        assert_eq!(table.row_count(), 0);
    }

    #[test]
    fn test_table_add_row() {
        let mut table = Table::new(vec!["Name".to_string(), "Age".to_string()]);
        let result = table.add_row(vec!["Alice".to_string(), "30".to_string()]);
        assert!(result.is_ok());
        assert_eq!(table.row_count(), 1);
    }

    #[test]
    fn test_table_row_mismatch() {
        let mut table = Table::new(vec!["Name".to_string(), "Age".to_string()]);
        let result = table.add_row(vec!["Alice".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_dataframe_editable() {
        let df = DataFrame::new(vec!["Col1".to_string()]).editable(true);
        assert!(df.is_editable());
    }

    #[test]
    fn test_dataframe_height() {
        let df = DataFrame::new(vec!["Col1".to_string()]).with_height(300.0);
        assert_eq!(df.get_height(), Some(300.0));
    }

    #[test]
    fn test_metric_with_delta() {
        let metric = Metric::new("Revenue", "$1000")
            .with_delta("+10%")
            .with_delta_color("green");
        assert_eq!(metric.get_delta(), Some("+10%"));
        assert_eq!(metric.get_delta_color(), Some("green"));
    }

    #[test]
    fn test_metric_column() {
        let mut col = MetricColumn::new();
        col.add_metric(Metric::new("Metric1", "100"));
        col.add_metric(Metric::new("Metric2", "200"));
        assert_eq!(col.metric_count(), 2);
    }

    #[test]
    fn test_data_editor() {
        let editor = DataEditor::new(vec!["Name".to_string()]).with_num_rows(20);
        assert_eq!(editor.get_num_rows(), 20);
    }
}
