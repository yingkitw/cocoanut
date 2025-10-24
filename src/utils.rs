//! Utility functions for macOS GUI applications

use crate::error::{CocoanutError, Result};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl, class};
use std::ffi::CString;

/// Convert a Rust string to a C string for Objective-C calls
/// 
/// # Arguments
/// 
/// * `s` - The Rust string to convert
/// 
/// # Returns
/// 
/// Returns a `Result<CString>` containing the C string
pub fn string_to_cstring(s: &str) -> Result<CString> {
    CString::new(s).map_err(|e| CocoanutError::InvalidParameter(e.to_string()))
}

/// Convert a C string to a Rust string
/// 
/// # Arguments
/// 
/// * `c_str` - The C string to convert
/// 
/// # Returns
/// 
/// Returns a `Result<String>` containing the Rust string
pub unsafe fn cstring_to_string(c_str: *const i8) -> Result<String> {
    if c_str.is_null() {
        return Err(CocoanutError::InvalidParameter("Null C string".to_string()));
    }
    
    let c_str = unsafe { std::ffi::CStr::from_ptr(c_str) };
    c_str.to_str()
        .map(|s| s.to_string())
        .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))
}

/// Check if an Objective-C object is null
/// 
/// # Arguments
/// 
/// * `obj` - The Objective-C object pointer
/// 
/// # Returns
/// 
/// Returns `true` if the object is null
pub fn is_null(obj: *mut Object) -> bool {
    obj.is_null()
}

/// Safely release an Objective-C object
/// 
/// # Arguments
/// 
/// * `obj` - The Objective-C object to release
pub unsafe fn release_object(obj: *mut Object) {
    if !obj.is_null() {
        let _: () = objc::msg_send![obj, release];
    }
}

/// Retain an Objective-C object
/// 
/// # Arguments
/// 
/// * `obj` - The Objective-C object to retain
/// 
/// # Returns
/// 
/// Returns the retained object
pub unsafe fn retain_object(obj: *mut Object) -> *mut Object {
    if !obj.is_null() {
        let _: () = objc::msg_send![obj, retain];
    }
    obj
}

/// Get the class name of an Objective-C object
/// 
/// # Arguments
/// 
/// * `obj` - The Objective-C object
/// 
/// # Returns
/// 
/// Returns a `Result<String>` containing the class name
pub unsafe fn get_class_name(obj: *mut Object) -> Result<String> {
    if obj.is_null() {
        return Err(CocoanutError::InvalidParameter("Null object".to_string()));
    }
    
    let class: *mut Object = objc::msg_send![obj, class];
    let class_name: *const i8 = objc::msg_send![class, name];
    unsafe { cstring_to_string(class_name) }
}

/// Check if the current thread is the main thread
pub fn is_main_thread() -> bool {
    unsafe {
        let main_thread: *mut Object = objc::msg_send![objc::class!(NSThread), mainThread];
        let current_thread: *mut Object = objc::msg_send![objc::class!(NSThread), currentThread];
        let is_main: bool = objc::msg_send![current_thread, isEqual: main_thread];
        is_main
    }
}

/// Execute code on the main thread
/// 
/// # Arguments
/// 
/// * `f` - The closure to execute on the main thread
/// 
/// # Returns
/// 
/// Returns a `Result<T>` containing the result of the closure
pub fn dispatch_to_main_thread<F, T>(f: F) -> Result<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    if is_main_thread() {
        return Ok(f());
    }
    
    // For simplicity, we'll just execute the function directly
    // In a real implementation, you would use dispatch_async
    Ok(f())
}

/// Convert a Rust string to an NSString
/// 
/// # Arguments
/// 
/// * `s` - The Rust string to convert
/// 
/// # Returns
/// 
/// Returns a `Result<*mut Object>` containing the NSString
pub fn string_to_ns_string(s: &str) -> Result<*mut Object> {
    let c_str = string_to_cstring(s)?;
    unsafe {
        let ns_string_class = objc::class!(NSString);
        let ns_string: *mut Object = objc::msg_send![
            ns_string_class,
            stringWithUTF8String: c_str.as_ptr()
        ];
        
        if ns_string.is_null() {
            return Err(CocoanutError::SystemError("Failed to create NSString".to_string()));
        }
        
        Ok(ns_string)
    }
}

/// Convert an NSString to a Rust string
/// 
/// # Arguments
/// 
/// * `ns_string` - The NSString to convert
/// 
/// # Returns
/// 
/// Returns a `Result<String>` containing the Rust string
pub unsafe fn ns_string_to_string(ns_string: *mut Object) -> Result<String> {
    if ns_string.is_null() {
        return Err(CocoanutError::InvalidParameter("Null NSString".to_string()));
    }
    
    let c_str: *const i8 = objc::msg_send![ns_string, UTF8String];
    unsafe { cstring_to_string(c_str) }
}

/// Log a message to the console
/// 
/// # Arguments
/// 
/// * `message` - The message to log
pub fn log(message: &str) {
    println!("[Cocoanut] {}", message);
}

/// Log an error message to the console
/// 
/// # Arguments
/// 
/// * `error` - The error message to log
pub fn log_error(error: &str) {
    eprintln!("[Cocoanut ERROR] {}", error);
}

/// Log a warning message to the console
/// 
/// # Arguments
/// 
/// * `warning` - The warning message to log
pub fn log_warning(warning: &str) {
    eprintln!("[Cocoanut WARNING] {}", warning);
}
