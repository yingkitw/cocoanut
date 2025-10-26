//! Layout trait - Unified interface for all layout operations
//!
//! This trait provides a standard interface for all components that support
//! layout operations. It's designed to work with objc 0.2.

use crate::core::ObjcAccess;

/// Trait for types that support layout operations.
///
/// Any type implementing `ObjcAccess` can implement this trait to automatically
/// get all layout operations. This eliminates code duplication across components.
pub trait Layout: ObjcAccess {
    /// Adds another Layout-backed control or view as a subview of this view.
    fn add_subview<V: Layout>(&self, view: &V) {
        self.with_backing_obj_mut(|backing_node| {
            view.with_backing_obj_mut(|subview_node| {
                // Implementation: backing_node.addSubview(subview_node)
                // Actual msg_send! call would go here
                let _ = (backing_node, subview_node);
            });
        });
    }

    /// Removes this control or view from its superview.
    fn remove_from_superview(&self) {
        self.with_backing_obj_mut(|_backing_node| {
            // Implementation: backing_node.removeFromSuperview()
            // Actual msg_send! call would go here
        });
    }

    /// Sets whether this view needs to be redrawn before being displayed.
    fn set_needs_display(&self, _needs_display: bool) {
        self.with_backing_obj_mut(|_obj| {
            // Implementation: obj.setNeedsDisplay(needs_display)
            // Actual msg_send! call would go here
        });
    }

    /// Sets whether this view is hidden or not.
    fn set_hidden(&self, _hide: bool) {
        self.with_backing_obj_mut(|_obj| {
            // Implementation: obj.setHidden(hide)
            // Actual msg_send! call would go here
        });
    }

    /// Returns whether this view is hidden or not.
    fn is_hidden(&self) -> bool {
        self.get_from_backing_obj(|_obj| {
            // Implementation: obj.isHidden()
            // Actual msg_send! call would go here
            false
        })
    }

    /// Sets the alpha (opacity) value for this view.
    fn set_alpha(&self, _value: f64) {
        self.with_backing_obj_mut(|_obj| {
            // Implementation: obj.setAlphaValue(value)
            // Actual msg_send! call would go here
        });
    }

    /// Gets the alpha (opacity) value for this view.
    fn get_alpha(&self) -> f64 {
        self.get_from_backing_obj(|_obj| {
            // Implementation: obj.alphaValue()
            // Actual msg_send! call would go here
            1.0
        })
    }
}
