//! Integration tests for Cocoanut

use cocoanut::*;
use cocoanut::streaming;
use cocoanut::drawing::{Color, Point, Size, Rect};

#[test]
fn test_error_from_string() {
    let err: CocoanutError = "test error".into();
    assert!(format!("{}", err).contains("test error"));
}

#[test]
fn test_color_creation() {
    let color = Color::new(1.0, 0.5, 0.0, 1.0).unwrap();
    assert_eq!(color.red, 1.0);
    assert_eq!(color.green, 0.5);
    assert_eq!(color.blue, 0.0);
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
fn test_ui_event_creation() {
    let event = streaming::UIEvent::ButtonClick { id: "test".to_string() };
    if let streaming::UIEvent::ButtonClick { id } = event {
        assert_eq!(id, "test");
    } else {
        panic!("Wrong event type");
    }
}

#[test]
fn test_error_types() {
    // Test different error types
    let err1 = CocoanutError::ApplicationInitFailed("test".to_string());
    assert!(format!("{}", err1).contains("test"));
    
    let err2 = CocoanutError::WindowCreationFailed("window".to_string());
    assert!(format!("{}", err2).contains("window"));
}
