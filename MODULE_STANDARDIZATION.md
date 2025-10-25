# Module Standardization Guide

## Overview

This document describes the standardized module structure for Cocoanut to ensure consistency and maintainability across the codebase.

---

## Standard Module Structure

Every module should follow this structure:

```rust
//! # Module Name
//! 
//! Brief description of what this module provides.
//! 
//! ## Features
//! 
//! - Feature 1
//! - Feature 2
//! 
//! ## Example
//! 
//! ```rust
//! // Example code
//! ```

// 1. Imports (organized by source)
use crate::core::error::Result;
use crate::utils::helpers::*;
use std::collections::HashMap;

// 2. Re-exports
pub use self::submodule::*;

// 3. Type definitions
pub struct MyType {
    field1: Type1,
    field2: Type2,
}

// 4. Implementations
impl MyType {
    /// Creates a new instance
    pub fn new() -> Self { /* ... */ }
}

// 5. Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() { /* ... */ }
}
```

---

## Import Organization

Organize imports in this order:

```rust
// 1. Crate imports (internal)
use crate::core::error::Result;
use crate::utils::helpers::*;

// 2. External crate imports
use objc::runtime::Object;
use serde::{Deserialize, Serialize};

// 3. Standard library imports
use std::collections::HashMap;
use std::ffi::CString;
```

---

## Helper Functions

The `utils/helpers.rs` module provides common patterns to reduce duplication:

### Null Pointer Checks
```rust
use cocoanut::utils::helpers::check_not_null;

let ptr: *mut Object = /* ... */;
check_not_null(ptr, "NSButton")?;
```

### String Conversion
```rust
use cocoanut::utils::helpers::string_to_cstring_with_context;

let c_str = string_to_cstring_with_context("hello", "button title")?;
```

### Validation
```rust
use cocoanut::utils::helpers::{validate, validate_all};

validate(condition, "error message")?;

validate_all(vec![
    (condition1, "error 1"),
    (condition2, "error 2"),
])?;
```

### Safe Dereferencing
```rust
use cocoanut::utils::helpers::{safe_deref, safe_deref_mut};

let ref_value = unsafe { safe_deref(ptr, "MyType")? };
let mut_ref = unsafe { safe_deref_mut(ptr, "MyType")? };
```

---

## Documentation Standards

### Module Documentation
```rust
//! # Module Name
//! 
//! One-line description.
//! 
//! Longer description explaining the module's purpose and usage.
//! 
//! ## Features
//! 
//! - Feature 1: Description
//! - Feature 2: Description
//! 
//! ## Example
//! 
//! ```rust
//! use cocoanut::module_name::*;
//! 
//! // Example code
//! ```
```

### Type Documentation
```rust
/// Brief description (one line)
/// 
/// Longer description explaining what this type is and why you might use it.
/// 
/// # Fields
/// 
/// * `field1` - Description of field1
/// * `field2` - Description of field2
pub struct MyType {
    pub field1: Type1,
    pub field2: Type2,
}
```

### Method Documentation
```rust
/// Brief description (one line)
/// 
/// Longer description.
/// 
/// # Arguments
/// 
/// * `param1` - Description
/// * `param2` - Description
/// 
/// # Returns
/// 
/// Description of return value
/// 
/// # Errors
/// 
/// Description of possible errors
/// 
/// # Example
/// 
/// ```rust
/// let result = function_name(arg1, arg2)?;
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // Implementation
}
```

---

## Error Handling

### Error Messages

All error messages should be descriptive and actionable:

```rust
// Bad
Err(CocoanutError::InvalidParameter("Invalid"))

// Good
Err(CocoanutError::InvalidParameter(
    "Button title cannot be empty. Provide a non-empty string.".to_string()
))
```

### Using Helpers

```rust
use cocoanut::utils::helpers::check_not_null;

// Instead of:
if ptr.is_null() {
    return Err(CocoanutError::InvalidParameter("Null pointer".to_string()));
}

// Use:
check_not_null(ptr, "NSButton")?;
```

---

## Testing Standards

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = setup();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_error_case() {
        let result = function_under_test(invalid_input);
        assert!(result.is_err());
    }
}
```

### Test Naming
- `test_basic_functionality` - Happy path
- `test_error_case` - Error handling
- `test_edge_case_*` - Boundary conditions
- `test_*_with_null` - Null pointer handling

---

## Naming Conventions

- **Types:** PascalCase (`Button`, `TextFieldBuilder`)
- **Functions:** snake_case (`create_button`, `set_title`)
- **Constants:** SCREAMING_SNAKE_CASE (`MAX_WIDTH`, `DEFAULT_HEIGHT`)
- **Variables:** snake_case (`button_width`, `is_enabled`)
- **Modules:** snake_case (`components`, `core_fixes`)

---

## Public API Surface

### Prelude Module

The `prelude` module exports the most commonly used types and functions:

```rust
pub mod prelude {
    // Core types
    pub use crate::core::error::Result;
    
    // Helpers
    pub use crate::utils::helpers::*;
    
    // Components
    pub use crate::components::*;
    
    // High-level API
    pub use crate::simple_app::*;
}
```

### Module Exports

Each module should re-export its public items:

```rust
// In module/mod.rs
pub mod submodule1;
pub mod submodule2;

pub use submodule1::*;
pub use submodule2::*;
```

---

## Code Review Checklist

- [ ] Module has documentation
- [ ] All public items are documented
- [ ] Code follows naming conventions
- [ ] Error messages are descriptive
- [ ] Helper functions are used where applicable
- [ ] Tests are comprehensive
- [ ] No unused imports
- [ ] Code is formatted with `cargo fmt`
- [ ] No clippy warnings

---

## Examples

### Well-Structured Module

```rust
//! Button component for macOS applications
//! 
//! Provides a simple button control with click event support.

use crate::core::error::Result;
use crate::utils::helpers::*;
use objc::runtime::Object;

/// A clickable button component
/// 
/// # Example
/// 
/// ```rust
/// let button = Button::new("Click me")?;
/// button.set_size(100.0, 40.0)?;
/// ```
pub struct Button {
    ns_button: *mut Object,
    id: String,
}

impl Button {
    /// Creates a new button with the specified title
    pub fn new(title: &str) -> Result<Self> {
        check_not_null(title as *const _ as *mut _, "title")?;
        
        let c_str = string_to_cstring_with_context(title, "button title")?;
        
        // Create button...
        Ok(Self {
            ns_button: std::ptr::null_mut(),
            id: "button".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_button_creation() {
        let button = Button::new("Test");
        assert!(button.is_ok());
    }
}
```

---

## Benefits

✅ **Consistency** - All modules follow the same structure  
✅ **Maintainability** - Easy to understand and modify  
✅ **Discoverability** - Clear organization and naming  
✅ **Quality** - Comprehensive documentation and tests  
✅ **Reusability** - Helper functions reduce duplication  

---

**Last Updated:** October 25, 2025  
**Version:** 1.0  
**Status:** Active
