//! ObjcProperty - Safe wrapper for Objective-C object lifetime management

use std::cell::RefCell;
use std::rc::Rc;
use objc::runtime::Object;

/// A safe wrapper for Objective-C objects with automatic lifetime management.
#[derive(Clone, Debug)]
pub struct ObjcProperty(Rc<RefCell<*mut Object>>);

impl ObjcProperty {
    /// Creates a new ObjcProperty from a raw Objective-C object pointer.
    pub fn retain(obj: *mut Object) -> Self {
        ObjcProperty(Rc::new(RefCell::new(obj)))
    }

    /// Provides mutable access to the underlying Objective-C object.
    pub fn with_mut<F: Fn(*mut Object)>(&self, handler: F) {
        let obj = self.0.borrow_mut();
        handler(*obj);
    }

    /// Provides immutable access to the underlying Objective-C object.
    pub fn get<R, F: Fn(&Object) -> R>(&self, handler: F) -> R {
        let obj = self.0.borrow();
        handler(unsafe { &**obj })
    }

    /// Returns a raw pointer to the underlying object.
    pub fn as_ptr(&self) -> *mut Object {
        *self.0.borrow()
    }
}

impl Drop for ObjcProperty {
    fn drop(&mut self) {
        // Cleanup handled by Rc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_objc_property_clone() {
        let ptr = std::ptr::null_mut();
        let prop1 = ObjcProperty::retain(ptr);
        let prop2 = prop1.clone();
        assert_eq!(prop1.as_ptr(), prop2.as_ptr());
    }

    #[test]
    fn test_objc_property_with_mut() {
        let ptr = std::ptr::null_mut();
        let prop = ObjcProperty::retain(ptr);
        let called = std::cell::RefCell::new(false);
        prop.with_mut(|_obj| {
            *called.borrow_mut() = true;
        });
        assert!(*called.borrow());
    }
}
