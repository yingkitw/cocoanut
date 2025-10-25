//! Zero-cost abstractions with minimal runtime overhead
//! 
//! This module provides abstractions that compile to efficient code
//! with minimal runtime overhead, following Rust's zero-cost abstraction principle.

use crate::core::error::{CocoanutError, Result};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Zero-cost wrapper for Objective-C objects
/// 
/// This wrapper provides type safety without runtime overhead
/// by using compile-time type information.
#[repr(transparent)]
pub struct ZeroCostObject<T> {
    ptr: *mut objc::runtime::Object,
    _phantom: PhantomData<T>,
}

impl<T> ZeroCostObject<T> {
    /// Create a new zero-cost object wrapper
    /// 
    /// # Safety
    /// 
    /// The pointer must be valid and point to an object of type T
    pub unsafe fn new(ptr: *mut objc::runtime::Object) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }
    
    /// Get the raw pointer
    pub fn as_ptr(&self) -> *mut objc::runtime::Object {
        self.ptr
    }
    
    /// Check if the object is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl<T> Clone for ZeroCostObject<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for ZeroCostObject<T> {}

unsafe impl<T> Send for ZeroCostObject<T> {}
unsafe impl<T> Sync for ZeroCostObject<T> {}

/// Zero-cost string wrapper
/// 
/// This wrapper provides efficient string operations without
/// unnecessary allocations or copies.
pub struct ZeroCostString {
    ptr: *const i8,
    len: usize,
}

impl ZeroCostString {
    /// Create a new zero-cost string from a C string
    /// 
    /// # Safety
    /// 
    /// The pointer must be valid and point to a null-terminated string
    pub unsafe fn from_c_str(ptr: *const i8) -> Self {
        let len = unsafe { libc::strlen(ptr) };
        Self { ptr, len }
    }
    
    /// Get the string as a Rust string slice
    pub fn as_str(&self) -> Result<&str> {
        if self.ptr.is_null() {
            return Err(CocoanutError::InvalidParameter("Null string pointer".to_string()));
        }
        
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr as *const u8, self.len);
            std::str::from_utf8(slice)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))
        }
    }
    
    /// Get the string length
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if the string is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Clone for ZeroCostString {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: self.len,
        }
    }
}

impl Copy for ZeroCostString {}

unsafe impl Send for ZeroCostString {}
unsafe impl Sync for ZeroCostString {}

/// Zero-cost array wrapper
/// 
/// This wrapper provides efficient array operations without
/// unnecessary allocations or copies.
pub struct ZeroCostArray<T> {
    ptr: *const T,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T> ZeroCostArray<T> {
    /// Create a new zero-cost array wrapper
    /// 
    /// # Safety
    /// 
    /// The pointer must be valid and point to an array of length `len`
    pub unsafe fn new(ptr: *const T, len: usize) -> Self {
        Self {
            ptr,
            len,
            _phantom: PhantomData,
        }
    }
    
    /// Get the array as a slice
    pub fn as_slice(&self) -> &[T] {
        if self.ptr.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }
    
    /// Get the array length
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Get an element by index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
}

impl<T> Clone for ZeroCostArray<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: self.len,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for ZeroCostArray<T> {}

unsafe impl<T> Send for ZeroCostArray<T> {}
unsafe impl<T> Sync for ZeroCostArray<T> {}

/// Zero-cost point structure
/// 
/// This structure represents a point with zero-cost operations
/// and can be safely transmuted to/from C structures.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZeroCostPoint {
    pub x: f64,
    pub y: f64,
}

impl ZeroCostPoint {
    /// Create a new zero-cost point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    /// Get the x coordinate
    pub fn x(&self) -> f64 {
        self.x
    }
    
    /// Get the y coordinate
    pub fn y(&self) -> f64 {
        self.y
    }
    
    /// Set the x coordinate
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    /// Set the y coordinate
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}

/// Zero-cost size structure
/// 
/// This structure represents a size with zero-cost operations
/// and can be safely transmuted to/from C structures.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZeroCostSize {
    pub width: f64,
    pub height: f64,
}

impl ZeroCostSize {
    /// Create a new zero-cost size
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    
    /// Get the width
    pub fn width(&self) -> f64 {
        self.width
    }
    
    /// Get the height
    pub fn height(&self) -> f64 {
        self.height
    }
    
    /// Set the width
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }
    
    /// Set the height
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }
}

/// Zero-cost rectangle structure
/// 
/// This structure represents a rectangle with zero-cost operations
/// and can be safely transmuted to/from C structures.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZeroCostRect {
    pub origin: ZeroCostPoint,
    pub size: ZeroCostSize,
}

impl ZeroCostRect {
    /// Create a new zero-cost rectangle
    pub fn new(origin: ZeroCostPoint, size: ZeroCostSize) -> Self {
        Self { origin, size }
    }
    
    /// Create a new zero-cost rectangle from coordinates
    pub fn from_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            origin: ZeroCostPoint::new(x, y),
            size: ZeroCostSize::new(width, height),
        }
    }
    
    /// Get the origin
    pub fn origin(&self) -> ZeroCostPoint {
        self.origin
    }
    
    /// Get the size
    pub fn size(&self) -> ZeroCostSize {
        self.size
    }
    
    /// Get the x coordinate
    pub fn x(&self) -> f64 {
        self.origin.x
    }
    
    /// Get the y coordinate
    pub fn y(&self) -> f64 {
        self.origin.y
    }
    
    /// Get the width
    pub fn width(&self) -> f64 {
        self.size.width
    }
    
    /// Get the height
    pub fn height(&self) -> f64 {
        self.size.height
    }
}

/// Zero-cost color structure
/// 
/// This structure represents a color with zero-cost operations
/// and can be safely transmuted to/from C structures.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZeroCostColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl ZeroCostColor {
    /// Create a new zero-cost color
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { red, green, blue, alpha }
    }
    
    /// Create a new zero-cost color from RGB values
    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::new(red, green, blue, 1.0)
    }
    
    /// Create a new zero-cost color from RGBA values
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self::new(red, green, blue, alpha)
    }
    
    /// Get the red component
    pub fn red(&self) -> f32 {
        self.red
    }
    
    /// Get the green component
    pub fn green(&self) -> f32 {
        self.green
    }
    
    /// Get the blue component
    pub fn blue(&self) -> f32 {
        self.blue
    }
    
    /// Get the alpha component
    pub fn alpha(&self) -> f32 {
        self.alpha
    }
}

/// Zero-cost iterator over an array
/// 
/// This iterator provides zero-cost iteration over arrays
/// without unnecessary allocations or copies.
pub struct ZeroCostIter<T> {
    ptr: *const T,
    len: usize,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<T> ZeroCostIter<T> {
    /// Create a new zero-cost iterator
    /// 
    /// # Safety
    /// 
    /// The pointer must be valid and point to an array of length `len`
    pub unsafe fn new(ptr: *const T, len: usize) -> Self {
        Self {
            ptr,
            len,
            index: 0,
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static> Iterator for ZeroCostIter<T> {
    type Item = &'static T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &*self.ptr.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.index;
        (remaining, Some(remaining))
    }
}

impl<T: 'static> ExactSizeIterator for ZeroCostIter<T> {}

/// Zero-cost string iterator
/// 
/// This iterator provides zero-cost iteration over strings
/// without unnecessary allocations or copies.
pub struct ZeroCostStringIter {
    ptr: *const i8,
    len: usize,
    index: usize,
}

impl ZeroCostStringIter {
    /// Create a new zero-cost string iterator
    /// 
    /// # Safety
    /// 
    /// The pointer must be valid and point to a null-terminated string
    pub unsafe fn new(ptr: *const i8) -> Self {
        let len = unsafe { libc::strlen(ptr) };
        Self { ptr, len, index: 0 }
    }
}

impl Iterator for ZeroCostStringIter {
    type Item = char;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let byte = unsafe { *self.ptr.add(self.index) } as u8;
            self.index += 1;
            Some(byte as char)
        } else {
            None
        }
    }
}

/// Zero-cost memory operations
/// 
/// This module provides zero-cost memory operations for
/// efficient data manipulation.
pub mod memory {
    use super::*;
    
    /// Copy memory with zero-cost abstraction
    pub unsafe fn copy<T: Copy>(src: *const T, dst: *mut T, count: usize) {
        unsafe { std::ptr::copy(src, dst, count); }
    }
    
    /// Move memory with zero-cost abstraction
    pub unsafe fn move_mem<T>(src: *mut T, dst: *mut T, count: usize) {
        unsafe {
            std::ptr::copy(src, dst, count);
            std::ptr::write_bytes(src, 0, count);
        }
    }
    
    /// Zero memory with zero-cost abstraction
    pub unsafe fn zero<T>(ptr: *mut T, count: usize) {
        unsafe { std::ptr::write_bytes(ptr, 0, count); }
    }
    
    /// Compare memory with zero-cost abstraction
    pub unsafe fn compare<T: PartialEq>(a: *const T, b: *const T, count: usize) -> bool {
        unsafe {
            std::slice::from_raw_parts(a, count) == std::slice::from_raw_parts(b, count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zero_cost_point() {
        let point = ZeroCostPoint::new(10.0, 20.0);
        assert_eq!(point.x(), 10.0);
        assert_eq!(point.y(), 20.0);
    }
    
    #[test]
    fn test_zero_cost_size() {
        let size = ZeroCostSize::new(100.0, 200.0);
        assert_eq!(size.width(), 100.0);
        assert_eq!(size.height(), 200.0);
    }
    
    #[test]
    fn test_zero_cost_rect() {
        let rect = ZeroCostRect::from_xywh(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.x(), 10.0);
        assert_eq!(rect.y(), 20.0);
        assert_eq!(rect.width(), 100.0);
        assert_eq!(rect.height(), 200.0);
    }
    
    #[test]
    fn test_zero_cost_color() {
        let color = ZeroCostColor::rgb(1.0, 0.5, 0.0);
        assert_eq!(color.red(), 1.0);
        assert_eq!(color.green(), 0.5);
        assert_eq!(color.blue(), 0.0);
        assert_eq!(color.alpha(), 1.0);
    }
    
    #[test]
    fn test_zero_cost_array() {
        let data = [1, 2, 3, 4, 5];
        let array = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
        
        assert_eq!(array.len(), 5);
        assert_eq!(array.get(0), Some(&1));
        assert_eq!(array.get(4), Some(&5));
        assert_eq!(array.get(5), None);
    }
    
    #[test]
    fn test_zero_cost_iter() {
        let data = [1, 2, 3, 4, 5];
        let iter = unsafe { ZeroCostIter::new(data.as_ptr(), data.len()) };
        
        let collected: Vec<_> = iter.collect();
        assert_eq!(collected, vec![&1, &2, &3, &4, &5]);
    }
}
