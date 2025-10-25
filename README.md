# 🥥 Cocoanut

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust Edition](https://img.shields.io/badge/rust%20edition-2024-orange.svg)](https://blog.rust-lang.org/2024/12/19/rust-2024.html)
[![Crates.io](https://img.shields.io/crates/v/cocoanut.svg)](https://crates.io/crates/cocoanut)

A Rust wrapper for Cocoa to develop macOS-specific GUI applications with **idiomatic, simple APIs**.

## Why Cocoanut?

### The Problem
Raw `objc` and `cocoa` crates require:
- Manual pointer management
- Verbose method calls
- Manual layout calculations
- No built-in styling system
- Steep learning curve

### The Solution
Cocoanut provides:
- ✅ **Builder Patterns** - Fluent, chainable API
- ✅ **Layout System** - Declarative VStack/HStack
- ✅ **Design System** - Carbon Design System built-in
- ✅ **Type Safety** - No raw pointers in public API
- ✅ **Simplicity** - Learn macOS GUI development easily

## Simplicity in Action

### Before (objc/cocoa)
```rust
let button = Button::new("Click")?;
button.set_title("Updated")?;
button.set_size(100.0, 50.0)?;
button.set_enabled(true)?;
window.add_subview(button, 10.0, 10.0, 100.0, 50.0)?;
```

### After (Cocoanut)
```rust
let button = Button::builder()
    .title("Updated")
    .size(100.0, 50.0)
    .enabled(true)
    .build()?;

let vstack = VStack::new()
    .spacing(Spacing::standard())
    .alignment(Alignment::Center);
```

## Comparison Table

| Crate | Level | Safety | Learning Curve | Performance | Best For |
|-------|-------|--------|----------------|-------------|----------|
| `objc` | Very Low | Low | High | Highest | System programming |
| `objc2` | Low | Medium | High | High | Performance-critical |
| `cocoa` | Medium | Medium | Medium | High | Legacy code |
| `cacao` | High | High | Low | Medium | Cross-platform apps |
| **`cocoanut`** | **Low-Medium** | **High** | **Low** | **High** | **Learning & prototyping** |

**Recommendation**: Start with Cocoanut to learn macOS GUI development in Rust!

## Quick Start

```bash
cargo add cocoanut
```

```rust
use cocoanut::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a window
    let window = Window::builder()
        .title("My App")
        .size(600.0, 400.0)
        .center()
        .build()?;
    
    // Run the app - components display automatically!
    app("MyApp")
        .with_window(window)
        .run()?;
    
    Ok(())
}
```

**That's it!** The app will display a window with Button, Label, and TextField components automatically.

Run it:
```bash
cargo run
```

## Features

- ✅ **Builder Patterns** - Fluent, chainable API
- ✅ **Layout System** - VStack/HStack for composition
- ✅ **Carbon Design System** - Professional styling
- ✅ **Window Management** - Native macOS windows
- ✅ **Menu System** - Application menus
- ✅ **Controls** - Button, Label, TextField
- ✅ **Event Handling** - User interactions
- ✅ **Drawing** - Core Graphics integration
- ✅ **Type Safety** - No raw pointers
- ✅ **Memory Safe** - Rust ownership system

## Examples

Run examples to see Cocoanut in action:

```bash
# Minimal app with components
cargo run --example minimal_app

# Basic window
cargo run --example basic_window

# Menu application
cargo run --example menu_app

# Comprehensive component demo
cargo run --example comprehensive_app
```

All examples display real macOS GUI windows with interactive components. Press `Cmd+Q` to quit.

See [docs/EXAMPLES_AND_TESTS.md](docs/EXAMPLES_AND_TESTS.md) for detailed examples.

## Requirements

- macOS 10.15 or later
- Rust 1.70 or later
- Xcode command line tools

## Building

```bash
cargo build
```

## Running Examples

```bash
cargo run --example basic_window
cargo run --example menu_app
```

## Testing

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Changelog

### 0.2.0 (Current)
- ✅ **GUI Components Display** - Button, Label, TextField now show in windows
- ✅ **SimpleApp API** - Minimal boilerplate app creation
- ✅ **Window Builder** - Fluent window configuration
- ✅ **Component Positioning** - Automatic layout in windows
- ✅ **Real macOS GUI** - No mocking, full NSApplication support

## 📚 Documentation

Complete documentation is available in [DOCUMENTATION.md](DOCUMENTATION.md). Key guides:

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture and design
- **[SRC_ORGANIZATION.md](SRC_ORGANIZATION.md)** - Source code structure
- **[MACOS_FEATURES.md](MACOS_FEATURES.md)** - Native macOS features
- **[SIMPLE_APP_API.md](SIMPLE_APP_API.md)** - High-level API guide
- **[EXAMPLES_UPDATED.md](EXAMPLES_UPDATED.md)** - Available examples

## 📋 Changelog

### 0.2.0
- ✅ **Trait-Based Architecture** - Drawable, Textual, Positionable traits
- ✅ **macOS Features** - Native Feel, Accessibility, Dark Mode, Touch Bar, Continuity
- ✅ **Priority Features** - 15 features across 3 priority levels
- ✅ **Source Organization** - Hierarchical module structure
- ✅ **Component Positioning** - Automatic layout in windows
- ✅ **Real macOS GUI** - No mocking, full NSApplication support

### 0.1.0
- Initial release
- Basic window management
- Menu system
- UI controls (Button, Label, TextField)
- Event handling
- Drawing utilities
- Comprehensive test suite
