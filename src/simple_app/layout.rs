//! Layout configuration and management

/// Layout configuration for component positioning
#[derive(Debug, Clone)]
pub struct Layout {
    /// Top padding from window edge
    pub top_padding: f64,
    /// Horizontal margin (left/right)
    pub horizontal_margin: f64,
    /// Gap between components
    pub gap: f64,
}

impl Layout {
    /// Create default layout (balanced spacing)
    pub fn default() -> Self {
        Self {
            top_padding: 40.0,
            horizontal_margin: 20.0,
            gap: 12.0,
        }
    }

    /// Create compact layout (tight spacing)
    pub fn compact() -> Self {
        Self {
            top_padding: 20.0,
            horizontal_margin: 10.0,
            gap: 8.0,
        }
    }

    /// Create spacious layout (generous spacing)
    pub fn spacious() -> Self {
        Self {
            top_padding: 60.0,
            horizontal_margin: 40.0,
            gap: 20.0,
        }
    }

    /// Set top padding
    pub fn top_padding(mut self, padding: f64) -> Self {
        self.top_padding = padding;
        self
    }

    /// Set horizontal margin
    pub fn horizontal_margin(mut self, margin: f64) -> Self {
        self.horizontal_margin = margin;
        self
    }

    /// Set component gap
    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::default()
    }
}
