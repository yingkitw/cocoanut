//! Component types and configuration

use std::fmt;

/// Component types that can be added to a window
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// NSButton control
    Button,
    /// NSTextField used as label (non-editable)
    Label,
    /// NSTextField used as input field (editable)
    TextField,
    /// Checkbox control
    Checkbox,
    /// Radio button control
    Radio,
    /// Slider control
    Slider,
    /// Dropdown/Popup button
    Dropdown,
    /// TextArea (multi-line text)
    TextArea,
    /// ScrollView container
    ScrollView,
    /// TabView container
    TabView,
    /// SplitView container
    SplitView,
    /// GroupBox container
    GroupBox,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Button => write!(f, "Button"),
            Kind::Label => write!(f, "Label"),
            Kind::TextField => write!(f, "TextField"),
            Kind::Checkbox => write!(f, "Checkbox"),
            Kind::Radio => write!(f, "Radio"),
            Kind::Slider => write!(f, "Slider"),
            Kind::Dropdown => write!(f, "Dropdown"),
            Kind::TextArea => write!(f, "TextArea"),
            Kind::ScrollView => write!(f, "ScrollView"),
            Kind::TabView => write!(f, "TabView"),
            Kind::SplitView => write!(f, "SplitView"),
            Kind::GroupBox => write!(f, "GroupBox"),
        }
    }
}

/// Configurable component with customizable properties
#[derive(Debug, Clone)]
pub struct Comp {
    /// Component type
    pub kind: Kind,
    /// Component title/text
    pub text: String,
    /// Component width
    pub width: f64,
    /// Component height
    pub height: f64,
}

impl Comp {
    /// Create a new component configuration
    pub fn new(kind: Kind) -> Self {
        let (text, width, height) = match kind {
            Kind::Button => ("Click Me!".to_string(), 100.0, 40.0),
            Kind::Label => ("Label".to_string(), 300.0, 30.0),
            Kind::TextField => ("Input".to_string(), 300.0, 30.0),
            Kind::Checkbox => ("Option".to_string(), 200.0, 25.0),
            Kind::Radio => ("Choice".to_string(), 150.0, 25.0),
            Kind::Slider => ("Slider".to_string(), 250.0, 25.0),
            Kind::Dropdown => ("Select".to_string(), 200.0, 30.0),
            Kind::TextArea => ("Text".to_string(), 400.0, 100.0),
            Kind::ScrollView => ("ScrollView".to_string(), 350.0, 200.0),
            Kind::TabView => ("TabView".to_string(), 350.0, 200.0),
            Kind::SplitView => ("SplitView".to_string(), 350.0, 200.0),
            Kind::GroupBox => ("GroupBox".to_string(), 350.0, 200.0),
        };
        Self { kind, text, width, height }
    }

    /// Set component text
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Set component size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}
