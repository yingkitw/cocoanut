# 🥥 Cocoanut Examples - Updated to SimpleApp API

All examples have been updated to use the new **SimpleApp** builder API for simplified macOS GUI development.

## Examples Overview

### 1. **minimal_app** - Simplest Example
```bash
cargo run --example minimal_app
```

**What it shows:**
- SimpleApp basic usage
- Window configuration (title, size, centered)
- Fluent API pattern

**Code:**
```rust
app("Minimal App")
    .title("My Cocoanut App")
    .size(600.0, 400.0)
    .centered(true)
    .run()?;
```

---

### 2. **basic_window** - Window Creation
```bash
cargo run --example basic_window
```

**What it shows:**
- Creating a simple window
- Window sizing
- Centering on screen

**Code:**
```rust
app("Basic Window")
    .title("Hello, Cocoanut!")
    .size(800.0, 600.0)
    .centered(true)
    .run()?;
```

---

### 3. **menu_app** - Menu System
```bash
cargo run --example menu_app
```

**What it shows:**
- Creating File menu with items
- Creating Edit menu
- Menu item separators
- SimpleApp integration

**Code:**
```rust
let file_menu = Menu::new("File")?;
let new_item = MenuItem::new("New", Some("newDocument:"))?;
file_menu.add_item(new_item)?;

app("Menu App")
    .title("Menu Application Example")
    .size(600.0, 400.0)
    .centered(true)
    .run()?;
```

---

### 4. **window_builder_example** - Builder Pattern
```bash
cargo run --example window_builder_example
```

**What it shows:**
- SimpleApp fluent API
- Window builder pattern
- Configuration options
- Before/after API comparison

**Code:**
```rust
// SimpleApp API
app("MyApp")
    .title("Title")
    .size(800.0, 600.0)
    .centered(true)
    .run()?;

// Window builder
Window::builder()
    .title("Custom Window")
    .size(1024.0, 768.0)
    .center()
    .resizable(true)
    .build()?;
```

---

### 5. **comprehensive_app** - Full Feature Demo
```bash
cargo run --example comprehensive_app
```

**What it shows:**
- SimpleApp setup
- All basic controls (Button, Label, TextField, Checkbox, etc.)
- Event binding (on_click, on_change)
- Layout system (VStack, HStack)
- Container views (TabView, ScrollView, GroupBox)
- Data display (TableView, CollectionView)
- macOS features (DarkMode, Accessibility, Styling)
- Design system colors and typography

**Features demonstrated:**
- 26+ components
- Event handlers
- Layout system
- Styling system
- macOS integration

---

### 6. **modern_features_simple** - Modern Rust Patterns
```bash
cargo run --example modern_features_simple
```

**What it shows:**
- Async/await support
- Streaming APIs
- Zero-cost abstractions
- macOS integration features
- Design system
- SimpleApp usage

---

### 7. **comprehensive_components** - Component Showcase
```bash
cargo run --example comprehensive_components
```

**What it shows:**
- All available components
- Component builders
- Component properties
- Integration with SimpleApp

---

### 8. **event_binding_example** - Event Handling
```bash
cargo run --example event_binding_example
```

**What it shows:**
- Button click events
- Text field change events
- State management with Arc<Mutex<T>>
- Event handler patterns

---

### 9. **simplified_api** - API Simplification
```bash
cargo run --example simplified_api
```

**What it shows:**
- Before/after API comparison
- Builder pattern benefits
- Layout system usage
- Styling system usage

---

## Running All Examples

```bash
# Build all examples
cargo build --examples

# Run specific example
cargo run --example minimal_app

# Run comprehensive demo
cargo run --example comprehensive_app

# List all examples
cargo run --example --help
```

## Key Improvements

| Aspect | Before | After |
|--------|--------|-------|
| **Boilerplate** | 80+ lines | 5 lines |
| **API** | Low-level objc | High-level SimpleApp |
| **Readability** | Poor | Excellent |
| **Type Safety** | Manual | Automatic |
| **Learning Curve** | Steep | Gentle |

## SimpleApp API Quick Reference

```rust
// Create app
app("AppName")

// Configure
.title("Window Title")
.size(width, height)
.centered(true)
.with_window(window)

// Run
.run()?
```

## Example Patterns

### Pattern 1: Minimal App
```rust
app("MyApp").run()?;
```

### Pattern 2: Configured App
```rust
app("MyApp")
    .title("My Application")
    .size(800.0, 600.0)
    .centered(true)
    .run()?;
```

### Pattern 3: Custom Window
```rust
let window = Window::builder()
    .title("Custom")
    .size(1024.0, 768.0)
    .build()?;

app("MyApp")
    .with_window(window)
    .run()?;
```

### Pattern 4: With Components
```rust
let button = Button::builder()
    .title("Click Me")
    .on_click(|| println!("Clicked!"))
    .build()?;

let window = Window::builder()
    .title("App")
    .size(600.0, 400.0)
    .build()?;

app("MyApp")
    .with_window(window)
    .run()?;
```

## Running Examples

All examples run as real macOS GUI applications:

```bash
cargo run --example minimal_app
```

This creates a real, native macOS window with full event loop support.

## Next Steps

1. **Explore examples** - Start with `minimal_app`, then `basic_window`
2. **Try components** - Check `comprehensive_components`
3. **Learn events** - Study `event_binding_example`
4. **Build your app** - Use patterns from examples as templates

## Documentation

- `SIMPLE_APP_API.md` - Complete SimpleApp API reference
- `SIMPLIFICATION_STRATEGY.md` - Design decisions and patterns
- `README.md` - Project overview

## Summary

All examples have been updated to use the **SimpleApp API**, demonstrating:
- ✅ 80% reduction in boilerplate code
- ✅ Fluent, readable API
- ✅ Type-safe configuration
- ✅ Real macOS GUI support
- ✅ Easy learning curve

The examples are now **production-ready templates** for building macOS applications with Cocoanut! 🎉
