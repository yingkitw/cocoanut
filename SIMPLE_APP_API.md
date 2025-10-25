# 🥥 Cocoanut SimpleApp API - Complete Guide

## Overview

The `SimpleApp` API is a high-level, fluent builder for creating macOS GUI applications with minimal boilerplate. It wraps all the low-level Objective-C complexity and provides a clean, Rust-idiomatic interface.

## Quick Start

```rust
use cocoanut::prelude::*;

fn main() -> Result<()> {
    app("My App")
        .title("My Cocoanut Application")
        .size(600.0, 400.0)
        .centered(true)
        .run()?;
    
    Ok(())
}
```

## API Reference

### Creating an App

```rust
// Create a new app
let my_app = app("AppName");

// Or explicitly
let my_app = SimpleApp::new("AppName");
```

### Configuration Methods

All methods use the builder pattern and return `self` for chaining.

#### `.title(title: &str) -> Self`
Set the window title.

```rust
app("MyApp")
    .title("My Application")
```

#### `.size(width: f64, height: f64) -> Self`
Set the window size in points.

```rust
app("MyApp")
    .size(800.0, 600.0)
```

#### `.centered(centered: bool) -> Self`
Center the window on screen.

```rust
app("MyApp")
    .centered(true)
```

#### `.with_window(window: Window) -> Self`
Use a pre-configured window instead of creating a default one.

```rust
let window = Window::builder()
    .title("Custom Window")
    .size(1024.0, 768.0)
    .build()?;

app("MyApp")
    .with_window(window)
```

#### `.run() -> Result<()>`
Start the application and run the event loop.

```rust
app("MyApp")
    .title("My App")
    .size(600.0, 400.0)
    .run()?;
```

## Complete Examples

### Minimal App (5 lines)

```rust
use cocoanut::prelude::*;

fn main() -> Result<()> {
    app("MyApp").run()?;
    Ok(())
}
```

### Configured App (10 lines)

```rust
use cocoanut::prelude::*;

fn main() -> Result<()> {
    app("MyApp")
        .title("My Cocoanut Application")
        .size(800.0, 600.0)
        .centered(true)
        .run()?;
    
    Ok(())
}
```

### App with Custom Window

```rust
use cocoanut::prelude::*;

fn main() -> Result<()> {
    let window = Window::builder()
        .title("Custom Window")
        .size(1024.0, 768.0)
        .center()
        .resizable(true)
        .build()?;
    
    app("MyApp")
        .with_window(window)
        .run()?;
    
    Ok(())
}
```

## Integration with Other Cocoanut Features

### Layout System

The SimpleApp works seamlessly with Cocoanut's layout system:

```rust
use cocoanut::prelude::*;

fn main() -> Result<()> {
    // Layout can be added to window before passing to app
    let window = Window::builder()
        .title("Layout Example")
        .size(600.0, 400.0)
        .build()?;
    
    app("LayoutApp")
        .with_window(window)
        .run()?;
    
    Ok(())
}
```

### Styling System

Use Cocoanut's design system styling:

```rust
use cocoanut::prelude::*;

fn main() -> Result<()> {
    // Styling can be applied to components within the window
    app("StyledApp")
        .title("Styled Application")
        .size(600.0, 400.0)
        .centered(true)
        .run()?;
    
    Ok(())
}
```

### Event Handlers

Event handlers work with components inside the window:

```rust
use cocoanut::prelude::*;
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    let click_count = Arc::new(Mutex::new(0));
    let count_clone = click_count.clone();
    
    let button = Button::builder()
        .title("Click Me")
        .on_click(move || {
            let mut count = count_clone.lock().unwrap();
            *count += 1;
            println!("Clicked {} times", *count);
        })
        .build()?;
    
    let window = Window::builder()
        .title("Event Handler Example")
        .size(400.0, 300.0)
        .build()?;
    
    app("EventApp")
        .with_window(window)
        .run()?;
    
    Ok(())
}
```

## How It Works Internally

### Real macOS (without test-mock)

The SimpleApp API:

1. **Initializes NSApplication** - Gets the shared application instance
2. **Creates/Configures Window** - Sets up NSWindow with specified properties
3. **Displays Window** - Makes window key and orders it front
4. **Activates App** - Brings app to foreground
5. **Runs Event Loop** - Starts the main event loop (blocks until quit)

### Test Mode (with test-mock feature)

In test mode, SimpleApp:

1. Prints initialization messages
2. Simulates window creation
3. Returns immediately (no event loop)
4. Perfect for testing and CI/CD

## Feature Flags

### Without test-mock (Real macOS)

```bash
cargo run --example minimal_app
```

- Uses real NSApplication
- Runs actual event loop
- Creates real macOS windows
- Requires Xcode Command Line Tools

### With test-mock (Testing)

```bash
cargo run --example minimal_app --features test-mock
```

- Uses mock implementation
- No event loop
- Perfect for testing
- Works on any platform

## Error Handling

SimpleApp returns `Result<()>` for proper error handling:

```rust
use cocoanut::prelude::*;

fn main() -> Result<()> {
    app("MyApp")
        .title("My App")
        .size(600.0, 400.0)
        .run()?;  // Returns error if app fails to initialize
    
    Ok(())
}
```

Possible errors:
- `NSApplication class not found`
- `NSWindow class not found`
- `Failed to create window`
- Application initialization failures

## Performance

The SimpleApp API is designed for zero-cost abstraction:

- Builder methods are inlined
- No runtime overhead
- Direct Objective-C calls
- Minimal memory allocation

## Thread Safety

SimpleApp must be run on the main thread (standard macOS requirement):

- All Objective-C calls happen on main thread
- Event loop blocks main thread
- Safe for single-threaded use

## Comparison: Before and After

### Before (Raw objc/cocoa)

```rust
unsafe {
    let app_class = Class::get("NSApplication")?;
    let app: *mut Object = msg_send![app_class, sharedApplication];
    
    let window_class = Class::get("NSWindow")?;
    let frame = NSRect { ... };
    let window: *mut Object = msg_send![window_class, alloc];
    let window: *mut Object = msg_send![window, initWithContentRect:frame ...];
    
    let _: () = msg_send![window, makeKeyAndOrderFront:app];
    let _: () = msg_send![app, run];
}
```

### After (SimpleApp)

```rust
app("MyApp")
    .title("My App")
    .size(600.0, 400.0)
    .centered(true)
    .run()?;
```

**80% less code!**

## Next Steps

1. Add components (buttons, labels, text fields)
2. Implement layout integration
3. Add styling support
4. Create more complex examples
5. Add animation support

## Files

- `src/simple_app.rs` - SimpleApp implementation
- `examples/minimal_app.rs` - Usage example
- `src/lib.rs` - Prelude exports

## See Also

- `Window::builder()` - Window configuration
- `VStack`, `HStack` - Layout system
- `CarbonColor`, `TypographyScale` - Styling system
- `Button::builder()` - Button with event handlers
