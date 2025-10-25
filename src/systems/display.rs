//! Phase 1: Core Display Elements
//! 
//! Implements Streamlit-inspired display elements for macOS GUI.
//! These are text and data display components that render content without user interaction.

use crate::core::error::Result;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use std::ffi::CString;

/// Universal element renderer - displays any supported type
pub struct Write {
    content: String,
    view: *mut Object,
}

impl Write {
    /// Create a new write element with content
    pub fn new(content: impl Into<String>) -> Result<Self> {
        let content = content.into();
        Ok(Write {
            content,
            view: std::ptr::null_mut(),
        })
    }

    /// Get the content as a string
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Render the element
    pub fn render(&self) -> String {
        self.content.clone()
    }
}

/// Plain text display element
pub struct Text {
    content: String,
    font_size: f64,
}

impl Text {
    /// Create a new text element
    pub fn new(content: impl Into<String>) -> Self {
        Text {
            content: content.into(),
            font_size: 14.0,
        }
    }

    /// Set font size
    pub fn size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    /// Get the text content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get font size
    pub fn font_size(&self) -> f64 {
        self.font_size
    }
}

/// Markdown rendering element
pub struct Markdown {
    content: String,
    unsafe_allow_html: bool,
}

impl Markdown {
    /// Create a new markdown element
    pub fn new(content: impl Into<String>) -> Self {
        Markdown {
            content: content.into(),
            unsafe_allow_html: false,
        }
    }

    /// Allow HTML in markdown
    pub fn unsafe_allow_html(mut self, allow: bool) -> Self {
        self.unsafe_allow_html = allow;
        self
    }

    /// Get the markdown content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Check if HTML is allowed
    pub fn allows_html(&self) -> bool {
        self.unsafe_allow_html
    }
}

/// Page title element
pub struct Title {
    content: String,
}

impl Title {
    /// Create a new title element
    pub fn new(content: impl Into<String>) -> Self {
        Title {
            content: content.into(),
        }
    }

    /// Get the title content
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Section header element
pub struct Header {
    content: String,
    level: u8,
}

impl Header {
    /// Create a new header element (level 1)
    pub fn new(content: impl Into<String>) -> Self {
        Header {
            content: content.into(),
            level: 1,
        }
    }

    /// Set header level (1-6)
    pub fn level(mut self, level: u8) -> Self {
        self.level = level.clamp(1, 6);
        self
    }

    /// Get the header content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the header level
    pub fn level_value(&self) -> u8 {
        self.level
    }
}

/// Subsection header element
pub struct Subheader {
    content: String,
}

impl Subheader {
    /// Create a new subheader element
    pub fn new(content: impl Into<String>) -> Self {
        Subheader {
            content: content.into(),
        }
    }

    /// Get the subheader content
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Small caption text element
pub struct Caption {
    content: String,
}

impl Caption {
    /// Create a new caption element
    pub fn new(content: impl Into<String>) -> Self {
        Caption {
            content: content.into(),
        }
    }

    /// Get the caption content
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Code block with syntax highlighting
pub struct Code {
    content: String,
    language: Option<String>,
    line_numbers: bool,
}

impl Code {
    /// Create a new code block
    pub fn new(content: impl Into<String>) -> Self {
        Code {
            content: content.into(),
            language: None,
            line_numbers: false,
        }
    }

    /// Set the programming language for syntax highlighting
    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    /// Enable line numbers
    pub fn line_numbers(mut self, enable: bool) -> Self {
        self.line_numbers = enable;
        self
    }

    /// Get the code content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the language
    pub fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Check if line numbers are enabled
    pub fn has_line_numbers(&self) -> bool {
        self.line_numbers
    }
}

/// JSON object display element
pub struct Json {
    content: String,
    expanded: bool,
}

impl Json {
    /// Create a new JSON display element
    pub fn new(content: impl Into<String>) -> Self {
        Json {
            content: content.into(),
            expanded: true,
        }
    }

    /// Set whether JSON is expanded by default
    pub fn expanded(mut self, expand: bool) -> Self {
        self.expanded = expand;
        self
    }

    /// Get the JSON content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Check if expanded by default
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }
}

/// Function/class documentation display
pub struct Help {
    content: String,
    title: Option<String>,
}

impl Help {
    /// Create a new help element
    pub fn new(content: impl Into<String>) -> Self {
        Help {
            content: content.into(),
            title: None,
        }
    }

    /// Set the help title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Get the help content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the help title
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_creation() {
        let write = Write::new("Hello, World!").unwrap();
        assert_eq!(write.content(), "Hello, World!");
    }

    #[test]
    fn test_text_with_size() {
        let text = Text::new("Sample").size(18.0);
        assert_eq!(text.font_size(), 18.0);
    }

    #[test]
    fn test_markdown_html() {
        let md = Markdown::new("# Title").unsafe_allow_html(true);
        assert!(md.allows_html());
    }

    #[test]
    fn test_header_level_clamping() {
        let header = Header::new("Title").level(10);
        assert_eq!(header.level_value(), 6);
    }

    #[test]
    fn test_code_language() {
        let code = Code::new("fn main() {}").with_language("rust").line_numbers(true);
        assert_eq!(code.get_language(), Some("rust"));
        assert!(code.has_line_numbers());
    }

    #[test]
    fn test_json_expanded() {
        let json = Json::new("{}").expanded(false);
        assert!(!json.is_expanded());
    }

    #[test]
    fn test_help_with_title() {
        let help = Help::new("Documentation").with_title("API Reference");
        assert_eq!(help.get_title(), Some("API Reference"));
    }
}
