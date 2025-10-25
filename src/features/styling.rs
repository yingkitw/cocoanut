//! Styling system with Carbon Design System integration
//!
//! Provides consistent styling and theming for UI components following
//! IBM's Carbon Design System guidelines.

use crate::drawing::Color;

/// Carbon Design System color palette
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarbonColor {
    /// Interactive - Primary action color
    Interactive,
    /// Interactive - Hover state
    InteractiveHover,
    /// Interactive - Active state
    InteractiveActive,
    /// UI - Background
    UIBackground,
    /// UI - Light background
    UILightBackground,
    /// UI - Dark background
    UIDarkBackground,
    /// Text - Primary
    TextPrimary,
    /// Text - Secondary
    TextSecondary,
    /// Text - Tertiary
    TextTertiary,
    /// Text - Disabled
    TextDisabled,
    /// Support - Success
    SupportSuccess,
    /// Support - Warning
    SupportWarning,
    /// Support - Error
    SupportError,
    /// Support - Info
    SupportInfo,
}

impl CarbonColor {
    /// Get the RGB values for this color
    pub fn rgb(&self) -> (f64, f64, f64) {
        match self {
            // Interactive colors
            Self::Interactive => (0.0, 102.0 / 255.0, 204.0 / 255.0),           // #0066CC
            Self::InteractiveHover => (0.0, 82.0 / 255.0, 163.0 / 255.0),       // #0052A3
            Self::InteractiveActive => (0.0, 61.0 / 255.0, 122.0 / 255.0),      // #003D7A
            
            // UI colors
            Self::UIBackground => (1.0, 1.0, 1.0),                              // #FFFFFF
            Self::UILightBackground => (242.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0), // #F2F2F2
            Self::UIDarkBackground => (21.0 / 255.0, 21.0 / 255.0, 21.0 / 255.0), // #151515
            
            // Text colors
            Self::TextPrimary => (21.0 / 255.0, 21.0 / 255.0, 21.0 / 255.0),    // #151515
            Self::TextSecondary => (110.0 / 255.0, 110.0 / 255.0, 110.0 / 255.0), // #6E6E6E
            Self::TextTertiary => (161.0 / 255.0, 161.0 / 255.0, 161.0 / 255.0), // #A1A1A1
            Self::TextDisabled => (198.0 / 255.0, 198.0 / 255.0, 198.0 / 255.0), // #C6C6C6
            
            // Support colors
            Self::SupportSuccess => (36.0 / 255.0, 161.0 / 255.0, 72.0 / 255.0), // #24A148
            Self::SupportWarning => (244.0 / 255.0, 171.0 / 255.0, 0.0),        // #F4AB00
            Self::SupportError => (218.0 / 255.0, 33.0 / 255.0, 35.0 / 255.0),  // #DA2123
            Self::SupportInfo => (0.0, 113.0 / 255.0, 197.0 / 255.0),           // #0071C5
        }
    }
}

/// Typography scale following Carbon Design System
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographyScale {
    /// Display - 32pt, bold
    Display,
    /// Heading 1 - 28pt, semibold
    Heading1,
    /// Heading 2 - 24pt, semibold
    Heading2,
    /// Heading 3 - 20pt, semibold
    Heading3,
    /// Body - 16pt, regular
    Body,
    /// Label - 14pt, regular
    Label,
    /// Caption - 12pt, regular
    Caption,
}

impl TypographyScale {
    /// Get the font size in points
    pub fn font_size(&self) -> f64 {
        match self {
            Self::Display => 32.0,
            Self::Heading1 => 28.0,
            Self::Heading2 => 24.0,
            Self::Heading3 => 20.0,
            Self::Body => 16.0,
            Self::Label => 14.0,
            Self::Caption => 12.0,
        }
    }

    /// Get the font weight (0.0 = light, 0.5 = regular, 1.0 = bold)
    pub fn font_weight(&self) -> f64 {
        match self {
            Self::Display => 1.0,      // Bold
            Self::Heading1 => 0.75,    // Semibold
            Self::Heading2 => 0.75,    // Semibold
            Self::Heading3 => 0.75,    // Semibold
            Self::Body => 0.5,         // Regular
            Self::Label => 0.5,        // Regular
            Self::Caption => 0.5,      // Regular
        }
    }

    /// Get the line height as a multiple of font size
    pub fn line_height_multiplier(&self) -> f64 {
        match self {
            Self::Display => 1.3,
            Self::Heading1 => 1.3,
            Self::Heading2 => 1.3,
            Self::Heading3 => 1.3,
            Self::Body => 1.5,
            Self::Label => 1.4,
            Self::Caption => 1.4,
        }
    }
}

/// Spacing scale following Carbon Design System
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingScale {
    /// 2pt
    Compact,
    /// 4pt
    Tight,
    /// 8pt
    Standard,
    /// 12pt
    Relaxed,
    /// 16pt
    Loose,
    /// 24pt
    Spacious,
    /// 32pt
    ExtraSpacious,
}

impl SpacingScale {
    /// Get the spacing value in points
    pub fn value(&self) -> f64 {
        match self {
            Self::Compact => 2.0,
            Self::Tight => 4.0,
            Self::Standard => 8.0,
            Self::Relaxed => 12.0,
            Self::Loose => 16.0,
            Self::Spacious => 24.0,
            Self::ExtraSpacious => 32.0,
        }
    }
}

/// Corner radius scale following Carbon Design System
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerRadiusScale {
    /// 0pt - Sharp corners
    Sharp,
    /// 2pt - Subtle rounding
    Subtle,
    /// 4pt - Standard rounding
    Standard,
    /// 8pt - Pronounced rounding
    Pronounced,
}

impl CornerRadiusScale {
    /// Get the corner radius value in points
    pub fn value(&self) -> f64 {
        match self {
            Self::Sharp => 0.0,
            Self::Subtle => 2.0,
            Self::Standard => 4.0,
            Self::Pronounced => 8.0,
        }
    }
}

/// Component style configuration
#[derive(Debug, Clone)]
pub struct ComponentStyle {
    /// Background color
    pub background: CarbonColor,
    /// Text color
    pub text: CarbonColor,
    /// Border color
    pub border: CarbonColor,
    /// Typography scale
    pub typography: TypographyScale,
    /// Corner radius
    pub corner_radius: CornerRadiusScale,
    /// Padding
    pub padding: SpacingScale,
}

impl ComponentStyle {
    /// Create a default button style
    pub fn button() -> Self {
        Self {
            background: CarbonColor::Interactive,
            text: CarbonColor::UIBackground,
            border: CarbonColor::Interactive,
            typography: TypographyScale::Label,
            corner_radius: CornerRadiusScale::Standard,
            padding: SpacingScale::Standard,
        }
    }

    /// Create a default label style
    pub fn label() -> Self {
        Self {
            background: CarbonColor::UIBackground,
            text: CarbonColor::TextPrimary,
            border: CarbonColor::UILightBackground,
            typography: TypographyScale::Body,
            corner_radius: CornerRadiusScale::Sharp,
            padding: SpacingScale::Tight,
        }
    }

    /// Create a default text field style
    pub fn text_field() -> Self {
        Self {
            background: CarbonColor::UILightBackground,
            text: CarbonColor::TextPrimary,
            border: CarbonColor::UILightBackground,
            typography: TypographyScale::Body,
            corner_radius: CornerRadiusScale::Subtle,
            padding: SpacingScale::Standard,
        }
    }

    /// Set the background color
    pub fn with_background(mut self, background: CarbonColor) -> Self {
        self.background = background;
        self
    }

    /// Set the text color
    pub fn with_text(mut self, text: CarbonColor) -> Self {
        self.text = text;
        self
    }

    /// Set the typography scale
    pub fn with_typography(mut self, typography: TypographyScale) -> Self {
        self.typography = typography;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carbon_colors() {
        let (r, g, b) = CarbonColor::Interactive.rgb();
        assert!(r >= 0.0 && r <= 1.0);
        assert!(g >= 0.0 && g <= 1.0);
        assert!(b >= 0.0 && b <= 1.0);
    }

    #[test]
    fn test_typography_scale() {
        assert_eq!(TypographyScale::Display.font_size(), 32.0);
        assert_eq!(TypographyScale::Body.font_size(), 16.0);
        assert_eq!(TypographyScale::Caption.font_size(), 12.0);
    }

    #[test]
    fn test_spacing_scale() {
        assert_eq!(SpacingScale::Compact.value(), 2.0);
        assert_eq!(SpacingScale::Standard.value(), 8.0);
        assert_eq!(SpacingScale::Spacious.value(), 24.0);
    }

    #[test]
    fn test_corner_radius_scale() {
        assert_eq!(CornerRadiusScale::Sharp.value(), 0.0);
        assert_eq!(CornerRadiusScale::Standard.value(), 4.0);
    }

    #[test]
    fn test_component_styles() {
        let button_style = ComponentStyle::button();
        assert_eq!(button_style.background, CarbonColor::Interactive);

        let label_style = ComponentStyle::label();
        assert_eq!(label_style.text, CarbonColor::TextPrimary);

        let text_field_style = ComponentStyle::text_field();
        assert_eq!(text_field_style.background, CarbonColor::UILightBackground);
    }
}
