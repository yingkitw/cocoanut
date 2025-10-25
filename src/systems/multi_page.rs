//! Phase 5: Multi-Page Navigation
//! 
//! Implements multi-page app support with navigation.

use crate::core::error::Result;
use std::collections::HashMap;

/// Page - represents a single page in multi-page app
pub struct Page {
    name: String,
    title: String,
    icon: Option<String>,
    visible: bool,
}

impl Page {
    /// Create a new page
    pub fn new(name: impl Into<String>, title: impl Into<String>) -> Self {
        Page {
            name: name.into(),
            title: title.into(),
            icon: None,
            visible: true,
        }
    }

    /// Set page icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Get page name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get page title
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Get icon
    pub fn get_icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

/// Navigation - multi-page app navigation
pub struct Navigation {
    pages: HashMap<String, Page>,
    current_page: Option<String>,
    history: Vec<String>,
}

impl Navigation {
    /// Create a new navigation
    pub fn new() -> Self {
        Navigation {
            pages: HashMap::new(),
            current_page: None,
            history: Vec::new(),
        }
    }

    /// Add a page
    pub fn add_page(&mut self, page: Page) -> Result<()> {
        let name = page.get_name().to_string();
        self.pages.insert(name, page);
        Ok(())
    }

    /// Navigate to page
    pub fn navigate_to(&mut self, page_name: &str) -> Result<()> {
        if !self.pages.contains_key(page_name) {
            return Err("Page not found".into());
        }

        if let Some(current) = &self.current_page {
            self.history.push(current.clone());
        }

        self.current_page = Some(page_name.to_string());
        Ok(())
    }

    /// Go back to previous page
    pub fn go_back(&mut self) -> Result<()> {
        if let Some(previous) = self.history.pop() {
            self.current_page = Some(previous);
            Ok(())
        } else {
            Err("No previous page".into())
        }
    }

    /// Get current page
    pub fn get_current_page(&self) -> Option<&str> {
        self.current_page.as_deref()
    }

    /// Get page by name
    pub fn get_page(&self, name: &str) -> Option<&Page> {
        self.pages.get(name)
    }

    /// Get all pages
    pub fn get_pages(&self) -> Vec<&Page> {
        self.pages.values().collect()
    }

    /// Get visible pages
    pub fn get_visible_pages(&self) -> Vec<&Page> {
        self.pages.values().filter(|p| p.is_visible()).collect()
    }

    /// Get page count
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get history
    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for Navigation {
    fn default() -> Self {
        Self::new()
    }
}

/// Sidebar navigation - sidebar-based page navigation
pub struct SidebarNav {
    pages: Vec<String>,
    current_page: Option<String>,
    collapsed: bool,
}

impl SidebarNav {
    /// Create a new sidebar navigation
    pub fn new() -> Self {
        SidebarNav {
            pages: Vec::new(),
            current_page: None,
            collapsed: false,
        }
    }

    /// Add page to sidebar
    pub fn add_page(&mut self, page_name: impl Into<String>) {
        self.pages.push(page_name.into());
    }

    /// Select page
    pub fn select_page(&mut self, page_name: &str) -> Result<()> {
        if !self.pages.contains(&page_name.to_string()) {
            return Err("Page not found in sidebar".into());
        }
        self.current_page = Some(page_name.to_string());
        Ok(())
    }

    /// Get current page
    pub fn get_current_page(&self) -> Option<&str> {
        self.current_page.as_deref()
    }

    /// Get all pages
    pub fn get_pages(&self) -> &[String] {
        &self.pages
    }

    /// Toggle collapse
    pub fn toggle_collapse(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Check if collapsed
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// Get page count
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}

impl Default for SidebarNav {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = Page::new("home", "Home").icon("🏠");
        assert_eq!(page.get_name(), "home");
        assert_eq!(page.get_title(), "Home");
        assert_eq!(page.get_icon(), Some("🏠"));
    }

    #[test]
    fn test_navigation() {
        let mut nav = Navigation::new();
        let page1 = Page::new("home", "Home");
        let page2 = Page::new("about", "About");

        nav.add_page(page1).unwrap();
        nav.add_page(page2).unwrap();

        nav.navigate_to("home").unwrap();
        assert_eq!(nav.get_current_page(), Some("home"));

        nav.navigate_to("about").unwrap();
        assert_eq!(nav.get_current_page(), Some("about"));
    }

    #[test]
    fn test_navigation_history() {
        let mut nav = Navigation::new();
        nav.add_page(Page::new("home", "Home")).unwrap();
        nav.add_page(Page::new("about", "About")).unwrap();

        nav.navigate_to("home").unwrap();
        nav.navigate_to("about").unwrap();

        assert_eq!(nav.get_history().len(), 1);

        nav.go_back().unwrap();
        assert_eq!(nav.get_current_page(), Some("home"));
    }

    #[test]
    fn test_navigation_invalid_page() {
        let mut nav = Navigation::new();
        let result = nav.navigate_to("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_sidebar_nav() {
        let mut sidebar = SidebarNav::new();
        sidebar.add_page("home");
        sidebar.add_page("about");
        sidebar.add_page("contact");

        sidebar.select_page("home").unwrap();
        assert_eq!(sidebar.get_current_page(), Some("home"));
        assert_eq!(sidebar.page_count(), 3);
    }

    #[test]
    fn test_sidebar_collapse() {
        let mut sidebar = SidebarNav::new();
        assert!(!sidebar.is_collapsed());

        sidebar.toggle_collapse();
        assert!(sidebar.is_collapsed());

        sidebar.toggle_collapse();
        assert!(!sidebar.is_collapsed());
    }

    #[test]
    fn test_page_visibility() {
        let page = Page::new("home", "Home").visible(false);
        assert!(!page.is_visible());

        let page2 = Page::new("about", "About").visible(true);
        assert!(page2.is_visible());
    }

    #[test]
    fn test_navigation_visible_pages() {
        let mut nav = Navigation::new();
        nav.add_page(Page::new("home", "Home")).unwrap();
        nav.add_page(Page::new("hidden", "Hidden").visible(false)).unwrap();
        nav.add_page(Page::new("about", "About")).unwrap();

        let visible = nav.get_visible_pages();
        assert_eq!(visible.len(), 2);
    }

    #[test]
    fn test_navigation_get_page() {
        let mut nav = Navigation::new();
        let page = Page::new("home", "Home").icon("🏠");
        nav.add_page(page).unwrap();

        let retrieved = nav.get_page("home");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get_icon(), Some("🏠"));
    }

    #[test]
    fn test_navigation_multiple_back() {
        let mut nav = Navigation::new();
        nav.add_page(Page::new("home", "Home")).unwrap();
        nav.add_page(Page::new("about", "About")).unwrap();
        nav.add_page(Page::new("contact", "Contact")).unwrap();

        nav.navigate_to("home").unwrap();
        nav.navigate_to("about").unwrap();
        nav.navigate_to("contact").unwrap();

        assert_eq!(nav.get_history().len(), 2);

        nav.go_back().unwrap();
        assert_eq!(nav.get_current_page(), Some("about"));

        nav.go_back().unwrap();
        assert_eq!(nav.get_current_page(), Some("home"));
    }

    #[test]
    fn test_navigation_clear_history() {
        let mut nav = Navigation::new();
        nav.add_page(Page::new("home", "Home")).unwrap();
        nav.add_page(Page::new("about", "About")).unwrap();

        nav.navigate_to("home").unwrap();
        nav.navigate_to("about").unwrap();

        assert_eq!(nav.get_history().len(), 1);
        nav.clear_history();
        assert_eq!(nav.get_history().len(), 0);
    }

    #[test]
    fn test_sidebar_nav_invalid_page() {
        let mut sidebar = SidebarNav::new();
        sidebar.add_page("home");
        sidebar.add_page("about");

        let result = sidebar.select_page("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_sidebar_nav_multiple_pages() {
        let mut sidebar = SidebarNav::new();
        for i in 1..=5 {
            sidebar.add_page(format!("page{}", i));
        }

        assert_eq!(sidebar.page_count(), 5);

        sidebar.select_page("page3").unwrap();
        assert_eq!(sidebar.get_current_page(), Some("page3"));
    }

    #[test]
    fn test_page_with_icon() {
        let page = Page::new("settings", "Settings").icon("⚙️");
        assert_eq!(page.get_icon(), Some("⚙️"));
    }

    #[test]
    fn test_navigation_get_all_pages() {
        let mut nav = Navigation::new();
        nav.add_page(Page::new("home", "Home")).unwrap();
        nav.add_page(Page::new("about", "About")).unwrap();
        nav.add_page(Page::new("contact", "Contact")).unwrap();

        let all = nav.get_pages();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_navigation_page_count() {
        let mut nav = Navigation::new();
        assert_eq!(nav.page_count(), 0);

        nav.add_page(Page::new("home", "Home")).unwrap();
        assert_eq!(nav.page_count(), 1);

        nav.add_page(Page::new("about", "About")).unwrap();
        assert_eq!(nav.page_count(), 2);
    }

    #[test]
    fn test_sidebar_nav_get_pages() {
        let mut sidebar = SidebarNav::new();
        sidebar.add_page("home");
        sidebar.add_page("about");
        sidebar.add_page("contact");

        let pages = sidebar.get_pages();
        assert_eq!(pages.len(), 3);
        assert_eq!(pages[0], "home");
        assert_eq!(pages[1], "about");
        assert_eq!(pages[2], "contact");
    }

    #[test]
    fn test_navigation_go_back_empty_history() {
        let mut nav = Navigation::new();
        nav.add_page(Page::new("home", "Home")).unwrap();
        nav.navigate_to("home").unwrap();

        let result = nav.go_back();
        assert!(result.is_err());
    }

    #[test]
    fn test_page_default_visibility() {
        let page = Page::new("home", "Home");
        assert!(page.is_visible());
    }
}
