//! Phase 3: Layout Containers
//! 
//! Implements advanced layout containers for macOS GUI.
//! These are structural elements that organize other components.

use crate::core::error::Result;

/// Columns layout - side-by-side columns
pub struct Columns {
    num_columns: usize,
    gap: f64,
    weights: Vec<f64>,
}

impl Columns {
    /// Create a new columns layout
    pub fn new(num_columns: usize) -> Result<Self> {
        if num_columns == 0 {
            return Err("Must have at least 1 column".into());
        }
        Ok(Columns {
            num_columns,
            gap: 10.0,
            weights: vec![1.0; num_columns],
        })
    }

    /// Set gap between columns
    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    /// Set column weights (for proportional sizing)
    pub fn weights(mut self, weights: Vec<f64>) -> Result<Self> {
        if weights.len() != self.num_columns {
            return Err("Weight count must match column count".into());
        }
        self.weights = weights;
        Ok(self)
    }

    /// Get number of columns
    pub fn get_num_columns(&self) -> usize {
        self.num_columns
    }

    /// Get gap
    pub fn get_gap(&self) -> f64 {
        self.gap
    }

    /// Get weights
    pub fn get_weights(&self) -> &[f64] {
        &self.weights
    }
}

/// Tabs layout - tabbed interface
pub struct Tabs {
    tab_names: Vec<String>,
    active_tab: usize,
    lazy_load: bool,
}

impl Tabs {
    /// Create a new tabs layout
    pub fn new(tab_names: Vec<String>) -> Result<Self> {
        if tab_names.is_empty() {
            return Err("Must have at least 1 tab".into());
        }
        Ok(Tabs {
            tab_names,
            active_tab: 0,
            lazy_load: false,
        })
    }

    /// Set active tab by index
    pub fn active_tab(mut self, index: usize) -> Result<Self> {
        if index >= self.tab_names.len() {
            return Err("Tab index out of range".into());
        }
        self.active_tab = index;
        Ok(self)
    }

    /// Enable lazy loading of tab content
    pub fn lazy_load(mut self, enable: bool) -> Self {
        self.lazy_load = enable;
        self
    }

    /// Get tab names
    pub fn get_tab_names(&self) -> &[String] {
        &self.tab_names
    }

    /// Get active tab index
    pub fn get_active_tab(&self) -> usize {
        self.active_tab
    }

    /// Check if lazy loading enabled
    pub fn is_lazy_load(&self) -> bool {
        self.lazy_load
    }
}

/// Expander - collapsible section
pub struct Expander {
    label: String,
    expanded: bool,
    icon: Option<String>,
}

impl Expander {
    /// Create a new expander
    pub fn new(label: impl Into<String>) -> Self {
        Expander {
            label: label.into(),
            expanded: false,
            icon: None,
        }
    }

    /// Set expanded state
    pub fn expanded(mut self, state: bool) -> Self {
        self.expanded = state;
        self
    }

    /// Set custom icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Get label
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Check if expanded
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Get icon
    pub fn get_icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
}

/// Container - generic container for grouping elements
pub struct Container {
    border: bool,
    padding: f64,
    background_color: Option<String>,
}

impl Container {
    /// Create a new container
    pub fn new() -> Self {
        Container {
            border: false,
            padding: 10.0,
            background_color: None,
        }
    }

    /// Enable border
    pub fn border(mut self, enable: bool) -> Self {
        self.border = enable;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }

    /// Set background color
    pub fn background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    /// Check if has border
    pub fn has_border(&self) -> bool {
        self.border
    }

    /// Get padding
    pub fn get_padding(&self) -> f64 {
        self.padding
    }

    /// Get background color
    pub fn get_background_color(&self) -> Option<&str> {
        self.background_color.as_deref()
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

/// Form - form submission container
pub struct Form {
    name: String,
    submit_button_label: String,
    clear_on_submit: bool,
}

impl Form {
    /// Create a new form
    pub fn new(name: impl Into<String>) -> Self {
        Form {
            name: name.into(),
            submit_button_label: "Submit".to_string(),
            clear_on_submit: false,
        }
    }

    /// Set submit button label
    pub fn submit_button(mut self, label: impl Into<String>) -> Self {
        self.submit_button_label = label.into();
        self
    }

    /// Clear form on submit
    pub fn clear_on_submit(mut self, clear: bool) -> Self {
        self.clear_on_submit = clear;
        self
    }

    /// Get form name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get submit button label
    pub fn get_submit_button(&self) -> &str {
        &self.submit_button_label
    }

    /// Check if clears on submit
    pub fn clears_on_submit(&self) -> bool {
        self.clear_on_submit
    }
}

/// Sidebar - sidebar container
pub struct Sidebar {
    width: f64,
    collapsible: bool,
    collapsed: bool,
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        Sidebar {
            width: 250.0,
            collapsible: true,
            collapsed: false,
        }
    }

    /// Set sidebar width
    pub fn width(mut self, w: f64) -> Self {
        self.width = w;
        self
    }

    /// Set collapsible
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set collapsed state
    pub fn collapsed(mut self, state: bool) -> Self {
        self.collapsed = state;
        self
    }

    /// Get width
    pub fn get_width(&self) -> f64 {
        self.width
    }

    /// Check if collapsible
    pub fn is_collapsible(&self) -> bool {
        self.collapsible
    }

    /// Check if collapsed
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

/// Empty - placeholder container
pub struct Empty {
    height: f64,
}

impl Empty {
    /// Create a new empty container
    pub fn new() -> Self {
        Empty { height: 0.0 }
    }

    /// Set height
    pub fn height(mut self, h: f64) -> Self {
        self.height = h;
        self
    }

    /// Get height
    pub fn get_height(&self) -> f64 {
        self.height
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns_creation() {
        let cols = Columns::new(3).unwrap();
        assert_eq!(cols.get_num_columns(), 3);
    }

    #[test]
    fn test_columns_zero() {
        let result = Columns::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_columns_weights() {
        let cols = Columns::new(3)
            .unwrap()
            .weights(vec![1.0, 2.0, 1.0])
            .unwrap();
        assert_eq!(cols.get_weights().len(), 3);
    }

    #[test]
    fn test_tabs_creation() {
        let tabs = Tabs::new(vec!["Tab1".to_string(), "Tab2".to_string()]).unwrap();
        assert_eq!(tabs.get_tab_names().len(), 2);
    }

    #[test]
    fn test_tabs_empty() {
        let result = Tabs::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_expander() {
        let exp = Expander::new("Expand me").expanded(true).icon("▼");
        assert!(exp.is_expanded());
        assert_eq!(exp.get_icon(), Some("▼"));
    }

    #[test]
    fn test_container() {
        let cont = Container::new()
            .border(true)
            .padding(15.0)
            .background_color("#FFFFFF");
        assert!(cont.has_border());
        assert_eq!(cont.get_padding(), 15.0);
    }

    #[test]
    fn test_form() {
        let form = Form::new("contact").submit_button("Send").clear_on_submit(true);
        assert_eq!(form.get_name(), "contact");
        assert!(form.clears_on_submit());
    }

    #[test]
    fn test_sidebar() {
        let sidebar = Sidebar::new().width(300.0).collapsible(true);
        assert_eq!(sidebar.get_width(), 300.0);
        assert!(sidebar.is_collapsible());
    }

    #[test]
    fn test_empty() {
        let empty = Empty::new().height(20.0);
        assert_eq!(empty.get_height(), 20.0);
    }
}
