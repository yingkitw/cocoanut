//! Simplified drawing primitives tests - only testing actually implemented methods

use cocoanut::*;
use cocoanut::drawing::{Color, Point, Size, Rect};

#[test]
fn test_color_creation() {
    let color = Color::new(1.0, 0.5, 0.0, 1.0);
    assert!(color.is_ok());
    
    let color = color.unwrap();
    assert_eq!(color.red, 1.0);
    assert_eq!(color.green, 0.5);
    assert_eq!(color.blue, 0.0);
    assert_eq!(color.alpha, 1.0);
}

#[test]
fn test_color_validation() {
    // Test valid colors
    assert!(Color::new(0.0, 0.0, 0.0, 0.0).is_ok());
    assert!(Color::new(1.0, 1.0, 1.0, 1.0).is_ok());
    assert!(Color::new(0.5, 0.5, 0.5, 0.5).is_ok());
    
    // Test invalid colors (out of range)
    assert!(Color::new(-0.1, 0.0, 0.0, 0.0).is_err());
    assert!(Color::new(1.1, 0.0, 0.0, 0.0).is_err());
    assert!(Color::new(0.0, -0.1, 0.0, 0.0).is_err());
    assert!(Color::new(0.0, 1.1, 0.0, 0.0).is_err());
    assert!(Color::new(0.0, 0.0, -0.1, 0.0).is_err());
    assert!(Color::new(0.0, 0.0, 1.1, 0.0).is_err());
    assert!(Color::new(0.0, 0.0, 0.0, -0.1).is_err());
    assert!(Color::new(0.0, 0.0, 0.0, 1.1).is_err());
}

#[test]
fn test_color_predefined() {
    // Test predefined colors
    let red = Color::red();
    assert_eq!(red.red, 1.0);
    assert_eq!(red.green, 0.0);
    assert_eq!(red.blue, 0.0);
    assert_eq!(red.alpha, 1.0);
    
    let green = Color::green();
    assert_eq!(green.red, 0.0);
    assert_eq!(green.green, 1.0);
    assert_eq!(green.blue, 0.0);
    assert_eq!(green.alpha, 1.0);
    
    let blue = Color::blue();
    assert_eq!(blue.red, 0.0);
    assert_eq!(blue.green, 0.0);
    assert_eq!(blue.blue, 1.0);
    assert_eq!(blue.alpha, 1.0);
    
    let white = Color::white();
    assert_eq!(white.red, 1.0);
    assert_eq!(white.green, 1.0);
    assert_eq!(white.blue, 1.0);
    assert_eq!(white.alpha, 1.0);
    
    let black = Color::black();
    assert_eq!(black.red, 0.0);
    assert_eq!(black.green, 0.0);
    assert_eq!(black.blue, 0.0);
    assert_eq!(black.alpha, 1.0);
}

#[test]
fn test_color_rgb() {
    let color = Color::rgb(0.5, 0.7, 0.3);
    assert!(color.is_ok());
    
    let color = color.unwrap();
    assert_eq!(color.red, 0.5);
    assert_eq!(color.green, 0.7);
    assert_eq!(color.blue, 0.3);
    assert_eq!(color.alpha, 1.0);
}

#[test]
fn test_color_gray() {
    let color = Color::gray(0.5);
    assert!(color.is_ok());
    
    let color = color.unwrap();
    assert_eq!(color.red, 0.5);
    assert_eq!(color.green, 0.5);
    assert_eq!(color.blue, 0.5);
    assert_eq!(color.alpha, 1.0);
}

#[test]
fn test_point_creation() {
    let point = Point::new(100.0, 200.0);
    assert_eq!(point.x, 100.0);
    assert_eq!(point.y, 200.0);
}

#[test]
fn test_size_creation() {
    let size = Size::new(300.0, 400.0);
    assert_eq!(size.width, 300.0);
    assert_eq!(size.height, 400.0);
}

#[test]
fn test_rect_creation() {
    let origin = Point::new(10.0, 20.0);
    let size = Size::new(300.0, 400.0);
    let rect = Rect::new(origin, size);
    
    assert_eq!(rect.origin.x, 10.0);
    assert_eq!(rect.origin.y, 20.0);
    assert_eq!(rect.size.width, 300.0);
    assert_eq!(rect.size.height, 400.0);
}

#[test]
fn test_rect_from_xywh() {
    let rect = Rect::from_xywh(10.0, 20.0, 300.0, 400.0);
    
    assert_eq!(rect.origin.x, 10.0);
    assert_eq!(rect.origin.y, 20.0);
    assert_eq!(rect.size.width, 300.0);
    assert_eq!(rect.size.height, 400.0);
}

#[test]
fn test_rect_properties() {
    let rect = Rect::new(Point::new(10.0, 20.0), Size::new(100.0, 200.0));
    
    // Test properties
    assert_eq!(rect.min_x(), 10.0);
    assert_eq!(rect.min_y(), 20.0);
}

#[test]
fn test_drawing_performance() {
    // Test performance with many color creations
    let mut colors = Vec::new();
    
    for i in 0..1000 {
        let color = Color::new(
            (i as f64) / 1000.0,
            ((i * 2) as f64) / 2000.0,
            ((i * 3) as f64) / 3000.0,
            1.0
        ).unwrap();
        colors.push(color);
    }
    
    assert_eq!(colors.len(), 1000);
    // Should complete without panicking
}

#[test]
fn test_drawing_memory_management() {
    // Test memory management
    let mut points = Vec::new();
    let mut sizes = Vec::new();
    let mut rects = Vec::new();
    
    for i in 0..100 {
        let point = Point::new(i as f64, (i * 2) as f64);
        let size = Size::new((i * 3) as f64, (i * 4) as f64);
        let rect = Rect::new(point, size);
        
        points.push(point);
        sizes.push(size);
        rects.push(rect);
    }
    
    assert_eq!(points.len(), 100);
    assert_eq!(sizes.len(), 100);
    assert_eq!(rects.len(), 100);
    // Should be dropped without panicking
}
