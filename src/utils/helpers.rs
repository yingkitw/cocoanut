//! Common helper functions and patterns for reducing code duplication
//!
//! This module provides reusable utilities for common operations across the codebase.

use crate::core::error::{CocoanutError, Result};
use objc::runtime::Object;
use std::ffi::CString;

/// Helper for null pointer checks with descriptive error messages
/// 
/// # Arguments
/// 
/// * `ptr` - The pointer to check
/// * `context` - Description of what is being checked (e.g., "NSString", "NSButton")
/// 
/// # Returns
/// 
/// `Ok(())` if pointer is not null, or an error with context if it is
/// 
/// # Example
/// 
/// ```rust,no_run
/// use cocoanut::utils::helpers::check_not_null;
/// 
/// let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
/// check_not_null(ptr, "NSButton")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn check_not_null<T>(ptr: *mut T, context: &str) -> Result<()> {
    if ptr.is_null() {
        return Err(CocoanutError::InvalidParameter(
            format!("Null pointer encountered for {}", context)
        ));
    }
    Ok(())
}

/// Helper for converting Rust strings to C strings with error context
/// 
/// # Arguments
/// 
/// * `s` - The Rust string to convert
/// * `context` - Description of what is being converted
/// 
/// # Returns
/// 
/// A CString or an error with context
pub fn string_to_cstring_with_context(s: &str, context: &str) -> Result<CString> {
    CString::new(s).map_err(|e| CocoanutError::InvalidParameter(
        format!("Failed to convert {} to C string: {} (contains null bytes)", context, e)
    ))
}

/// Helper for safe pointer dereferencing
/// 
/// # Arguments
/// 
/// * `ptr` - The pointer to dereference
/// * `context` - Description of what is being dereferenced
/// 
/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure the pointer is valid and properly aligned.
/// 
/// # Returns
/// 
/// A reference to the dereferenced value or an error
pub unsafe fn safe_deref<'a, T>(ptr: *const T, context: &str) -> Result<&'a T> {
    if ptr.is_null() {
        return Err(CocoanutError::InvalidParameter(
            format!("Cannot dereference null pointer for {}", context)
        ));
    }
    Ok(unsafe { &*ptr })
}

/// Helper for safe mutable pointer dereferencing
/// 
/// # Arguments
/// 
/// * `ptr` - The mutable pointer to dereference
/// * `context` - Description of what is being dereferenced
/// 
/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure the pointer is valid, properly aligned, and not aliased.
/// 
/// # Returns
/// 
/// A mutable reference to the dereferenced value or an error
pub unsafe fn safe_deref_mut<'a, T>(ptr: *mut T, context: &str) -> Result<&'a mut T> {
    if ptr.is_null() {
        return Err(CocoanutError::InvalidParameter(
            format!("Cannot dereference null mutable pointer for {}", context)
        ));
    }
    Ok(unsafe { &mut *ptr })
}

/// Helper for wrapping operations that may fail with context
/// 
/// # Arguments
/// 
/// * `operation` - The operation to perform
/// * `context` - Description of the operation
/// 
/// # Returns
/// 
/// The result of the operation or an error with context
pub fn with_context<F, T>(operation: F, context: &str) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    operation().map_err(|e| {
        CocoanutError::SystemError(
            format!("Operation failed: {} - {}", context, e)
        )
    })
}

/// Helper for validating object state
/// 
/// # Arguments
/// 
/// * `condition` - The condition to check
/// * `error_message` - Message if condition is false
/// 
/// # Returns
/// 
/// `Ok(())` if condition is true, or an error if false
pub fn validate(condition: bool, error_message: &str) -> Result<()> {
    if !condition {
        return Err(CocoanutError::InvalidParameter(error_message.to_string()));
    }
    Ok(())
}

/// Helper for chaining multiple validations
/// 
/// # Arguments
/// 
/// * `validations` - Vector of (condition, error_message) tuples
/// 
/// # Returns
/// 
/// `Ok(())` if all conditions are true, or first error encountered
pub fn validate_all(validations: Vec<(bool, &str)>) -> Result<()> {
    for (condition, message) in validations {
        validate(condition, message)?;
    }
    Ok(())
}

/// Helper for safe object casting
/// 
/// # Arguments
/// 
/// * `obj` - The object to cast
/// * `expected_type` - The expected type name
/// 
/// # Returns
/// 
/// The object if casting is valid, or an error
pub fn safe_cast<T>(obj: *mut Object, expected_type: &str) -> Result<*mut T> {
    check_not_null(obj, expected_type)?;
    Ok(obj as *mut T)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_check_not_null_valid() {
        let ptr = &mut 42 as *mut i32;
        assert!(check_not_null(ptr, "test").is_ok());
    }
    
    #[test]
    fn test_check_not_null_invalid() {
        let ptr: *mut i32 = std::ptr::null_mut();
        assert!(check_not_null(ptr, "test").is_err());
    }
    
    #[test]
    fn test_string_to_cstring_valid() {
        let result = string_to_cstring_with_context("hello", "test");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_true() {
        assert!(validate(true, "test").is_ok());
    }
    
    #[test]
    fn test_validate_false() {
        assert!(validate(false, "test").is_err());
    }
    
    #[test]
    fn test_validate_all_pass() {
        let validations = vec![
            (true, "first"),
            (true, "second"),
            (true, "third"),
        ];
        assert!(validate_all(validations).is_ok());
    }
    
    #[test]
    fn test_validate_all_fail() {
        let validations = vec![
            (true, "first"),
            (false, "second"),
            (true, "third"),
        ];
        assert!(validate_all(validations).is_err());
    }
}
