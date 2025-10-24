//! Comprehensive window management tests

use cocoanut::*;
use cocoanut::window::Window;
use cocoanut::drawing::{Point, Size, Rect};

#[test]
fn test_window_creation() {
    let window = Window::new("Test Window", 800.0, 600.0);
    assert!(window.is_ok());
    
    let window = window.unwrap();
    assert_eq!(window.title(), "Test Window");
}

#[test]
fn test_window_dimensions() {
    let window = Window::new("Dimension Test", 1024.0, 768.0).unwrap();
    
    // Test initial size
    let (width, height) = window.size();
    assert_eq!(width, 1024.0);
    assert_eq!(height, 768.0);
}

#[test]
fn test_window_position() {
    let window = Window::new("Position Test", 400.0, 300.0).unwrap();
    
    // Test initial position (Window doesn't have position method yet)
    // This test would need to be implemented when position() method is added
}

#[test]
fn test_window_visibility() {
    let window = Window::new("Visibility Test", 400.0, 300.0).unwrap();
    
    // In mock mode, visibility is always false
    // In real mode, initially should be hidden
    assert!(!window.is_visible());
    
    // Show window - in mock mode this doesn't change visibility
    let _ = window.show();
    
    // In mock mode, visibility remains false
    // In real mode, it would be true after show()
    #[cfg(feature = "test-mock")]
    assert!(!window.is_visible());
    
    #[cfg(not(feature = "test-mock"))]
    assert!(window.is_visible());
    
    // Hide window - in mock mode this doesn't change visibility
    let _ = window.hide();
    assert!(!window.is_visible());
}

#[test]
fn test_window_title_change() {
    let mut window = Window::new("Original Title", 400.0, 300.0).unwrap();
    
    assert_eq!(window.title(), "Original Title");
    
    window.set_title("New Title");
    assert_eq!(window.title(), "New Title");
}

#[test]
fn test_window_resize() {
    let mut window = Window::new("Resize Test", 400.0, 300.0).unwrap();
    
    let (original_width, original_height) = window.size();
    assert_eq!(original_width, 400.0);
    assert_eq!(original_height, 300.0);
    
    // Resize window
    window.set_size(800.0, 600.0);
    let (new_width, new_height) = window.size();
    assert_eq!(new_width, 800.0);
    assert_eq!(new_height, 600.0);
}

// Note: These tests are commented out because the corresponding methods
// are not yet implemented in the Window struct. They can be uncommented
// when those methods are added to the Window implementation.

/*
#[test]
fn test_window_move() {
    let mut window = Window::new("Move Test", 400.0, 300.0).unwrap();
    
    let original_position = window.position();
    
    // Move window
    window.set_position(100.0, 200.0);
    let new_position = window.position();
    assert_eq!(new_position.x, 100.0);
    assert_eq!(new_position.y, 200.0);
}

#[test]
fn test_window_center() {
    let window = Window::new("Center Test", 400.0, 300.0).unwrap();
    
    // Center the window
    window.center();
    
    let position = window.position();
    // Position should be reasonable (not negative)
    assert!(position.x >= 0.0);
    assert!(position.y >= 0.0);
}

#[test]
fn test_window_minimize_restore() {
    let window = Window::new("Minimize Test", 400.0, 300.0).unwrap();
    
    // Initially not minimized
    assert!(!window.is_minimized());
    
    // Minimize window
    window.minimize();
    assert!(window.is_minimized());
    
    // Restore window
    window.restore();
    assert!(!window.is_minimized());
}

#[test]
fn test_window_close() {
    let window = Window::new("Close Test", 400.0, 300.0).unwrap();
    
    // Initially not closed
    assert!(!window.is_closed());
    
    // Close window
    window.close();
    assert!(window.is_closed());
}

#[test]
fn test_window_fullscreen() {
    let window = Window::new("Fullscreen Test", 400.0, 300.0).unwrap();
    
    // Initially not fullscreen
    assert!(!window.is_fullscreen());
    
    // Toggle fullscreen
    window.toggle_fullscreen();
    assert!(window.is_fullscreen());
    
    // Toggle back
    window.toggle_fullscreen();
    assert!(!window.is_fullscreen());
}

#[test]
fn test_window_always_on_top() {
    let mut window = Window::new("Always On Top Test", 400.0, 300.0).unwrap();
    
    // Initially not always on top
    assert!(!window.is_always_on_top());
    
    // Set always on top
    window.set_always_on_top(true);
    assert!(window.is_always_on_top());
    
    // Unset always on top
    window.set_always_on_top(false);
    assert!(!window.is_always_on_top());
}

#[test]
fn test_window_resizable() {
    let mut window = Window::new("Resizable Test", 400.0, 300.0).unwrap();
    
    // Initially resizable
    assert!(window.is_resizable());
    
    // Make non-resizable
    window.set_resizable(false);
    assert!(!window.is_resizable());
    
    // Make resizable again
    window.set_resizable(true);
    assert!(window.is_resizable());
}

#[test]
fn test_window_minimizable() {
    let mut window = Window::new("Minimizable Test", 400.0, 300.0).unwrap();
    
    // Initially minimizable
    assert!(window.is_minimizable());
    
    // Make non-minimizable
    window.set_minimizable(false);
    assert!(!window.is_minimizable());
    
    // Make minimizable again
    window.set_minimizable(true);
    assert!(window.is_minimizable());
}

#[test]
fn test_window_closable() {
    let mut window = Window::new("Closable Test", 400.0, 300.0).unwrap();
    
    // Initially closable
    assert!(window.is_closable());
    
    // Make non-closable
    window.set_closable(false);
    assert!(!window.is_closable());
    
    // Make closable again
    window.set_closable(true);
    assert!(window.is_closable());
}

#[test]
fn test_window_zoomable() {
    let mut window = Window::new("Zoomable Test", 400.0, 300.0).unwrap();
    
    // Initially zoomable
    assert!(window.is_zoomable());
    
    // Make non-zoomable
    window.set_zoomable(false);
    assert!(!window.is_zoomable());
    
    // Make zoomable again
    window.set_zoomable(true);
    assert!(window.is_zoomable());
}
*/

#[test]
fn test_multiple_windows() {
    let window1 = Window::new("Window 1", 400.0, 300.0).unwrap();
    let window2 = Window::new("Window 2", 500.0, 400.0).unwrap();
    
    assert_eq!(window1.title(), "Window 1");
    assert_eq!(window2.title(), "Window 2");
    
    let (width1, _) = window1.size();
    let (width2, _) = window2.size();
    assert_eq!(width1, 400.0);
    assert_eq!(width2, 500.0);
}

// Note: Event handler test commented out as event_handler() method is not yet implemented
/*
#[test]
fn test_window_events() {
    let window = Window::new("Event Test", 400.0, 300.0).unwrap();
    
    // Test that we can get event handlers
    let _event_handler = window.event_handler();
    // Note: Actual event testing would require a running event loop
}
*/

#[test]
fn test_window_creation_with_invalid_dimensions() {
    // Test with zero dimensions
    let result = Window::new("Invalid", 0.0, 0.0);
    // Should handle gracefully (either succeed with minimum size or fail gracefully)
    match result {
        Ok(window) => {
            let (width, height) = window.size();
            // In mock mode, zero dimensions are allowed
            // In real mode, dimensions should be reasonable
            #[cfg(feature = "test-mock")]
            {
                assert_eq!(width, 0.0);
                assert_eq!(height, 0.0);
            }
            #[cfg(not(feature = "test-mock"))]
            {
                assert!(width > 0.0);
                assert!(height > 0.0);
            }
        },
        Err(_) => {
            // If it fails, that's also acceptable
        }
    }
}

#[test]
fn test_window_creation_with_negative_dimensions() {
    // Test with negative dimensions
    let result = Window::new("Negative", -100.0, -200.0);
    // Should handle gracefully
    match result {
        Ok(window) => {
            let (width, height) = window.size();
            // In mock mode, negative dimensions are allowed
            // In real mode, dimensions should be positive
            #[cfg(feature = "test-mock")]
            {
                assert_eq!(width, -100.0);
                assert_eq!(height, -200.0);
            }
            #[cfg(not(feature = "test-mock"))]
            {
                assert!(width > 0.0);
                assert!(height > 0.0);
            }
        },
        Err(_) => {
            // If it fails, that's also acceptable
        }
    }
}
