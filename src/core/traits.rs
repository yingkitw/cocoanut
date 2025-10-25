//! Foundation traits for capability-facing design
//!
//! This module defines core traits that enable flexible, composable GUI components.

use crate::core::error::Result;
use objc::runtime::Object;

/// Trait for components that can be displayed in a window
pub trait Drawable {
    /// Get the underlying NSView pointer
    fn as_view(&self) -> *mut Object;

    /// Set visibility
    fn set_visible(&self, visible: bool) -> Result<()>;

    /// Get visibility state
    fn is_visible(&self) -> bool;
}

/// Trait for components with text content
pub trait Textual {
    /// Get the text content
    fn text(&self) -> &str;

    /// Set the text content
    fn set_text(&mut self, text: &str) -> Result<()>;
}

/// Trait for components with frame/position
pub trait Positionable {
    /// Set the frame (position and size)
    fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()>;

    /// Get the frame
    fn frame(&self) -> (f64, f64, f64, f64);
}

/// Trait for clickable components
pub trait Clickable {
    /// Set click handler
    fn on_click<F>(&mut self, handler: F) -> Result<()>
    where
        F: Fn() + 'static;
}

/// Trait for components with state
pub trait Stateful {
    /// Get the current state
    fn state(&self) -> String;

    /// Set the state
    fn set_state(&mut self, state: &str) -> Result<()>;
}

/// Trait for components that can be built
pub trait Builder: Sized {
    /// The type this builder produces
    type Output;

    /// Build the component
    fn build(self) -> Result<Self::Output>;
}

/// Trait for components that can be added to containers
pub trait Containable: Drawable {
    /// Get the component's identifier
    fn id(&self) -> &str;

    /// Set the component's identifier
    fn set_id(&mut self, id: &str);
}

/// Trait for container views
pub trait Container: Drawable {
    /// Add a subview
    fn add_subview(&self, subview: *mut Object) -> Result<()>;

    /// Remove a subview
    fn remove_subview(&self, subview: *mut Object) -> Result<()>;

    /// Get all subviews
    fn subviews(&self) -> Vec<*mut Object>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawable_trait_exists() {
        // Verify trait is properly defined
        fn assert_drawable<T: Drawable>() {}
        // This compiles if Drawable trait is valid
    }

    #[test]
    fn test_textual_trait_exists() {
        fn assert_textual<T: Textual>() {}
    }

    #[test]
    fn test_positionable_trait_exists() {
        fn assert_positionable<T: Positionable>() {}
    }
}
