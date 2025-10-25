//! Drawing utilities for macOS GUI applications

use crate::core::error::{CocoanutError, Result};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

/// A color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

/// A 2D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A 2D size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// A 2D rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Color {
    /// Create a new color with RGBA values
    /// 
    /// # Arguments
    /// 
    /// * `red` - Red component (0.0 to 1.0)
    /// * `green` - Green component (0.0 to 1.0)
    /// * `blue` - Blue component (0.0 to 1.0)
    /// * `alpha` - Alpha component (0.0 to 1.0)
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Color>` if the values are valid
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&red) || !(0.0..=1.0).contains(&green) || 
           !(0.0..=1.0).contains(&blue) || !(0.0..=1.0).contains(&alpha) {
            return Err(CocoanutError::InvalidParameter(
                "Color components must be between 0.0 and 1.0".to_string()
            ));
        }
        
        Ok(Color { red, green, blue, alpha })
    }
    
    /// Create a color from RGB values (alpha defaults to 1.0)
    pub fn rgb(red: f64, green: f64, blue: f64) -> Result<Self> {
        Self::new(red, green, blue, 1.0)
    }
    
    /// Create a grayscale color
    pub fn gray(value: f64) -> Result<Self> {
        Self::rgb(value, value, value)
    }
    
    /// Common colors
    pub fn black() -> Self {
        Self { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }
    
    pub fn white() -> Self {
        Self { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
    }
    
    pub fn red() -> Self {
        Self { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }
    
    pub fn green() -> Self {
        Self { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }
    }
    
    pub fn blue() -> Self {
        Self { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }
    }
    
    /// Convert to NSColor
    pub(crate) fn to_ns_color(&self) -> *mut Object {
        unsafe {
            let color_class = objc::class!(NSColor);
            let ns_color: *mut Object = msg_send![
                color_class,
                colorWithRed:self.red green:self.green blue:self.blue alpha:self.alpha
            ];
            ns_color
        }
    }
}

impl Point {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    /// Convert to NSPoint
    pub(crate) fn to_ns_point(&self) -> *mut Object {
        unsafe {
            let point_class = objc::class!(NSPoint);
            let ns_point: *mut Object = objc::msg_send![point_class, new];
            let _: () = msg_send![ns_point, setX: self.x];
            let _: () = msg_send![ns_point, setY: self.y];
            ns_point
        }
    }
}

impl Size {
    /// Create a new size
    pub fn new(width: f64, height: f64) -> Self {
        Size { width, height }
    }
    
    /// Convert to NSSize
    pub(crate) fn to_ns_size(&self) -> *mut Object {
        unsafe {
            let size_class = objc::class!(NSSize);
            let ns_size: *mut Object = objc::msg_send![size_class, new];
            let _: () = msg_send![ns_size, setWidth: self.width];
            let _: () = msg_send![ns_size, setHeight: self.height];
            ns_size
        }
    }
}

impl Rect {
    /// Create a new rectangle
    pub fn new(origin: Point, size: Size) -> Self {
        Rect { origin, size }
    }
    
    /// Create a rectangle from x, y, width, height
    pub fn from_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }
    
    /// Get the minimum X coordinate
    pub fn min_x(&self) -> f64 {
        self.origin.x
    }
    
    /// Get the minimum Y coordinate
    pub fn min_y(&self) -> f64 {
        self.origin.y
    }
    
    /// Get the maximum X coordinate
    pub fn max_x(&self) -> f64 {
        self.origin.x + self.size.width
    }
    
    /// Get the maximum Y coordinate
    pub fn max_y(&self) -> f64 {
        self.origin.y + self.size.height
    }
    
    /// Check if a point is inside this rectangle
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min_x() && point.x <= self.max_x() &&
        point.y >= self.min_y() && point.y <= self.max_y()
    }
    
    /// Convert to NSRect
    pub(crate) fn to_ns_rect(&self) -> *mut Object {
        unsafe {
            let rect_class = objc::class!(NSRect);
            let ns_rect: *mut Object = objc::msg_send![rect_class, new];
            let _: () = msg_send![ns_rect, setOrigin: self.origin.to_ns_point()];
            let _: () = msg_send![ns_rect, setSize: self.size.to_ns_size()];
            ns_rect
        }
    }
}

/// Drawing context for custom drawing operations
pub struct DrawingContext {
    ns_graphics_context: *mut Object,
}

impl DrawingContext {
    /// Create a new drawing context
    pub fn new() -> Result<Self> {
        unsafe {
            let context_class = objc::class!(NSGraphicsContext);
            let ns_graphics_context: *mut Object = msg_send![
                context_class,
                currentContext
            ];
            
            if ns_graphics_context.is_null() {
                return Err(CocoanutError::DrawingError(
                    "No current graphics context".to_string()
                ));
            }
            
            Ok(DrawingContext {
                ns_graphics_context,
            })
        }
    }
    
    /// Set the current fill color
    pub fn set_fill_color(&self, color: Color) -> Result<()> {
        unsafe {
            let ns_color = color.to_ns_color();
            let _: () = msg_send![ns_color, setFill];
            Ok(())
        }
    }
    
    /// Set the current stroke color
    pub fn set_stroke_color(&self, color: Color) -> Result<()> {
        unsafe {
            let ns_color = color.to_ns_color();
            let _: () = msg_send![ns_color, setStroke];
            Ok(())
        }
    }
    
    /// Fill a rectangle
    pub fn fill_rect(&self, rect: Rect) -> Result<()> {
        unsafe {
            let ns_rect = rect.to_ns_rect();
            let _: () = msg_send![ns_rect, fill];
            Ok(())
        }
    }
    
    /// Stroke a rectangle
    pub fn stroke_rect(&self, rect: Rect) -> Result<()> {
        unsafe {
            let ns_rect = rect.to_ns_rect();
            let _: () = msg_send![ns_rect, stroke];
            Ok(())
        }
    }
}

impl Default for DrawingContext {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // This is a fallback - in practice, you should handle this error properly
            unsafe { std::mem::zeroed() }
        })
    }
}
