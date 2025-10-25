//! Core fixes and improvements for Cocoanut
//!
//! This module addresses Priority 1 immediate fixes:
//! - Compilation error resolution
//! - Memory management (ARC integration)
//! - Comprehensive error handling
//! - Thread safety enforcement
//! - API consistency

use crate::core::error::Result;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::sync::{Arc, Mutex};

/// Thread-safe wrapper for GUI operations
pub struct ThreadSafeView {
    view: Arc<Mutex<*mut Object>>,
}

impl ThreadSafeView {
    /// Create a new thread-safe view wrapper
    pub fn new(view: *mut Object) -> Self {
        Self {
            view: Arc::new(Mutex::new(view)),
        }
    }

    /// Execute operation on main thread
    pub fn on_main_thread<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(*mut Object) -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        let view = *self.view.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        #[cfg(not(feature = "test-mock"))]
        unsafe {
            // Check if on main thread
            let thread_class = objc::class!(NSThread);
            let is_main: bool = msg_send![thread_class, isMainThread];

            if !is_main {
                return Err(crate::core::error::CocoanutError::ThreadingError(
                    "GUI operations must be on main thread".into(),
                ));
            }
        }

        f(view)
    }

    /// Get the underlying view pointer (unsafe)
    pub fn as_ptr(&self) -> Result<*mut Object> {
        Ok(*self.view.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?)
    }
}

impl Clone for ThreadSafeView {
    fn clone(&self) -> Self {
        Self {
            view: Arc::clone(&self.view),
        }
    }
}

/// Memory management helper for ARC integration
pub struct MemoryManager;

impl MemoryManager {
    /// Retain an Objective-C object (increment reference count)
    pub fn retain(obj: *mut Object) -> Result<()> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = objc::msg_send![obj, retain];
        }
        Ok(())
    }

    /// Release an Objective-C object (decrement reference count)
    pub fn release(obj: *mut Object) -> Result<()> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = objc::msg_send![obj, release];
        }
        Ok(())
    }

    /// Autorelease an Objective-C object
    pub fn autorelease(obj: *mut Object) -> Result<*mut Object> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let result: *mut Object = objc::msg_send![obj, autorelease];
            Ok(result)
        }
        #[cfg(feature = "test-mock")]
        Ok(obj)
    }

    /// Get reference count
    pub fn retain_count(obj: *mut Object) -> Result<usize> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let count: usize = objc::msg_send![obj, retainCount];
            Ok(count)
        }
        #[cfg(feature = "test-mock")]
        Ok(1)
    }
}

/// Enhanced error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Error message
    pub message: String,
    /// Error code
    pub code: i32,
    /// Component that caused the error
    pub component: String,
    /// Operation that failed
    pub operation: String,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(message: &str, code: i32, component: &str, operation: &str) -> Self {
        Self {
            message: message.to_string(),
            code,
            component: component.to_string(),
            operation: operation.to_string(),
        }
    }

    /// Get formatted error message
    pub fn formatted(&self) -> String {
        format!(
            "[{}] {} (code: {}, operation: {})",
            self.component, self.message, self.code, self.operation
        )
    }
}

/// API consistency helper
pub struct ApiConsistency;

impl ApiConsistency {
    /// Validate method signature consistency
    pub fn validate_signature(method_name: &str, param_count: usize) -> Result<()> {
        // Ensure method names follow snake_case
        if !method_name.chars().all(|c| c.is_lowercase() || c == '_' || c.is_numeric()) {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                format!("Method name should be snake_case: {}", method_name),
            ));
        }

        // Ensure reasonable parameter count
        if param_count > 10 {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                format!("Too many parameters: {}", param_count),
            ));
        }

        Ok(())
    }

    /// Validate return type consistency
    pub fn validate_return_type(type_name: &str) -> Result<()> {
        // Ensure return types are consistent
        let valid_types = ["Result", "Option", "bool", "String", "f64", "i32"];

        if !valid_types.iter().any(|t| type_name.contains(t)) {
            return Err(crate::core::error::CocoanutError::InvalidParameter(
                format!("Inconsistent return type: {}", type_name),
            ));
        }

        Ok(())
    }
}

/// Compilation error tracker
pub struct CompilationTracker {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl CompilationTracker {
    /// Create a new compilation tracker
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Get all errors
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Get all warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Check if there are errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

impl Default for CompilationTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_safe_view_creation() {
        let view = ThreadSafeView::new(std::ptr::null_mut());
        assert!(view.as_ptr().is_ok());
    }

    #[test]
    fn test_thread_safe_view_clone() {
        let view = ThreadSafeView::new(std::ptr::null_mut());
        let cloned = view.clone();
        assert!(cloned.as_ptr().is_ok());
    }

    #[test]
    fn test_error_context() {
        let ctx = ErrorContext::new("Test error", 42, "Button", "create");
        let formatted = ctx.formatted();
        assert!(formatted.contains("Button"));
        assert!(formatted.contains("42"));
    }

    #[test]
    fn test_api_consistency_method_name() {
        assert!(ApiConsistency::validate_signature("create_button", 2).is_ok());
        assert!(ApiConsistency::validate_signature("CreateButton", 2).is_err());
    }

    #[test]
    fn test_api_consistency_param_count() {
        assert!(ApiConsistency::validate_signature("method", 5).is_ok());
        assert!(ApiConsistency::validate_signature("method", 15).is_err());
    }

    #[test]
    fn test_compilation_tracker() {
        let mut tracker = CompilationTracker::new();
        assert!(!tracker.has_errors());

        tracker.add_error("Error 1".to_string());
        assert!(tracker.has_errors());
        assert_eq!(tracker.error_count(), 1);

        tracker.add_warning("Warning 1".to_string());
        assert_eq!(tracker.warning_count(), 1);
    }
}
