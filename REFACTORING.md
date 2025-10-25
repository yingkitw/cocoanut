# Cocoanut Refactoring Guide

## Overview

This document describes the refactoring effort to make Cocoanut more modular, trait-based, test-friendly, and capability-facing.

## Key Principles

### 1. **Trait-Based Design**
- Components implement capability traits (`Drawable`, `Textual`, `Positionable`)
- Enables composition and flexible component combinations
- Makes testing easier through trait mocking

### 2. **Capability-Facing**
- Components expose capabilities, not implementation details
- Users think in terms of "what can this do?" not "what type is this?"
- Traits define capabilities clearly

### 3. **Concise Code**
- Macros reduce boilerplate (e.g., `ns_string!`)
- Shared base structures (`ControlBase`)
- DRY principle throughout

### 4. **Test-Friendly**
- Traits enable easy mocking
- `test-mock` feature for testing without macOS
- Builder patterns for test setup

### 5. **Modular Organization**
- Clear separation of concerns
- Foundation traits in `traits.rs`
- Specific implementations in feature modules

## Module Organization

```
src/
├── error.rs              # Error types
├── utils.rs              # Utilities
├── traits.rs             # Foundation traits (NEW)
├── window.rs             # Window management
├── controls.rs           # Original controls (legacy)
├── controls_v2.rs        # Refactored controls (NEW)
├── menu.rs               # Menu system
├── layout.rs             # Layout system
├── styling.rs            # Styling system
├── events.rs             # Event handling
├── builder.rs            # Builder patterns
└── simple_app.rs         # High-level API
```

## Trait System

### Core Traits

#### `Drawable`
```rust
pub trait Drawable {
    fn as_view(&self) -> *mut Object;
    fn set_visible(&self, visible: bool) -> Result<()>;
    fn is_visible(&self) -> bool;
}
```
- All GUI components implement this
- Enables adding components to windows
- Provides visibility control

#### `Textual`
```rust
pub trait Textual {
    fn text(&self) -> &str;
    fn set_text(&mut self, text: &str) -> Result<()>;
}
```
- For components with text content
- Button, Label, TextField implement this
- Enables text manipulation

#### `Positionable`
```rust
pub trait Positionable {
    fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()>;
    fn frame(&self) -> (f64, f64, f64, f64);
}
```
- For components with position/size
- Enables layout system integration
- Provides frame management

#### `Clickable`
```rust
pub trait Clickable {
    fn on_click<F>(&mut self, handler: F) -> Result<()>
    where F: Fn() + 'static;
}
```
- For interactive components
- Enables event binding
- Type-safe callbacks

#### `Container`
```rust
pub trait Container: Drawable {
    fn add_subview(&self, subview: *mut Object) -> Result<()>;
    fn remove_subview(&self, subview: *mut Object) -> Result<()>;
    fn subviews(&self) -> Vec<*mut Object>;
}
```
- For container views
- Enables composition
- Manages child views

## Code Patterns

### Macro for NSString Creation
```rust
macro_rules! ns_string {
    ($text:expr) => {{
        let cstr = CString::new($text)?;
        let ns_string_class = objc::class!(NSString);
        let ns_str: *mut Object = msg_send![ns_string_class, stringWithUTF8String: cstr.as_ptr()];
        ns_str
    }};
}
```
- Reduces boilerplate significantly
- Centralizes NSString creation
- Consistent error handling

### Base Structure Pattern
```rust
struct ControlBase {
    ns_view: *mut Object,
    id: String,
}
```
- Shared functionality for all controls
- Implements common traits
- Reduces code duplication

### Builder Pattern
```rust
pub struct ButtonBuilder {
    title: String,
    width: Option<f64>,
    height: Option<f64>,
}

impl ButtonBuilder {
    pub fn title(mut self, title: &str) -> Self { ... }
    pub fn size(mut self, width: f64, height: f64) -> Self { ... }
    pub fn build(self) -> Result<Button> { ... }
}
```
- Fluent API
- Type-safe configuration
- Easy to test

## Migration Guide

### From controls.rs to controls_v2.rs

**Old Way:**
```rust
let button = Button::new("Click")?;
button.set_title("New")?;
button.set_size(100.0, 50.0)?;
window.add_subview(button.ns_button())?;
```

**New Way:**
```rust
let button = Button::builder()
    .title("New")
    .size(100.0, 50.0)
    .build()?;
window.add_subview(button.as_view())?;
```

### Using Traits

**Old Way:**
```rust
// No trait support, type-specific methods
let title = button.title();
```

**New Way:**
```rust
// Trait-based, generic over capabilities
fn set_text<T: Textual>(component: &mut T, text: &str) -> Result<()> {
    component.set_text(text)
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_button_builder() {
    let builder = ButtonBuilder::default()
        .title("Test")
        .size(100.0, 50.0);
    assert_eq!(builder.title, "Test");
}
```

### Trait Tests
```rust
#[test]
fn test_drawable_trait_exists() {
    fn assert_drawable<T: Drawable>() {}
    // Compile-time verification
}
```

### Mock Testing
```rust
#[cfg(feature = "test-mock")]
// Components work without macOS
let button = Button::new("Test")?;
```

## Benefits

### For Users
- Cleaner, more intuitive API
- Less boilerplate code
- Better error messages
- Easier to learn

### For Developers
- Easier to add new components
- Trait-based composition
- Better code organization
- Easier to test

### For Maintainers
- Reduced code duplication
- Clear separation of concerns
- Easier to refactor
- Better documentation

## Next Steps

1. **Migrate controls.rs** - Gradually replace with controls_v2.rs
2. **Add more traits** - Stateful, Validatable, etc.
3. **Implement trait for all components** - Button, Label, TextField, etc.
4. **Create trait documentation** - Usage examples and patterns
5. **Add comprehensive tests** - Unit, integration, and property tests

## Performance Considerations

- **Zero-cost abstractions** - Traits compile to same code as direct calls
- **No runtime overhead** - All trait methods are inlined
- **Memory efficient** - ControlBase reduces duplication
- **Fast compilation** - Modular structure enables incremental builds

## Backward Compatibility

- `controls.rs` remains unchanged
- `controls_v2.rs` is new, opt-in
- Gradual migration path
- No breaking changes to public API
