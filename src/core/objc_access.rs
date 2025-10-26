//! ObjcAccess trait - Unified interface for accessing backing Objective-C objects

use objc::runtime::Object;

/// Trait for types that wrap Objective-C objects and need to provide access to them.
pub trait ObjcAccess {
    /// Provides mutable access to the backing Objective-C object.
    fn with_backing_obj_mut<F: Fn(*mut Object)>(&self, handler: F);

    /// Provides immutable access to the backing Objective-C object.
    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R;
}
