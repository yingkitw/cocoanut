# Source Code Organization

## Overview

The `src/` folder is organized into logical modules for better maintainability and clarity.

## Directory Structure

```
src/
├── core/                    # Foundation & Core Types
│   ├── mod.rs              # Core module exports
│   ├── error.rs            # Error types
│   ├── traits.rs           # Foundation traits
│   └── utils.rs            # Core utilities
│
├── components/             # GUI Components
│   ├── mod.rs              # Component exports
│   ├── basic/              # Basic controls
│   │   ├── mod.rs
│   │   ├── button.rs       # Button component
│   │   ├── label.rs        # Label component
│   │   └── textfield.rs    # TextField component
│   ├── advanced/           # Advanced controls
│   │   ├── mod.rs
│   │   ├── checkbox.rs
│   │   ├── radio.rs
│   │   ├── slider.rs
│   │   ├── segmented.rs
│   │   ├── stepper.rs
│   │   └── switch.rs
│   ├── containers/         # Container views
│   │   ├── mod.rs
│   │   ├── scrollview.rs
│   │   ├── tabview.rs
│   │   ├── splitview.rs
│   │   └── groupbox.rs
│   └── data_display/       # Data display components
│       ├── mod.rs
│       ├── tableview.rs
│       ├── outlineview.rs
│       └── collectionview.rs
│
├── systems/                # Core Systems
│   ├── mod.rs              # Systems exports
│   ├── events.rs           # Event system
│   ├── layout.rs           # Layout system
│   ├── animation.rs        # Animation system
│   ├── data_binding.rs     # Data binding
│   └── builder.rs          # Builder patterns
│
├── features/               # macOS Features
│   ├── mod.rs              # Features exports
│   ├── macos/              # macOS integration
│   │   ├── mod.rs
│   │   ├── native_feel.rs
│   │   ├── accessibility.rs
│   │   ├── dark_mode.rs
│   │   ├── touchbar.rs
│   │   └── continuity.rs
│   ├── styling.rs          # Design system
│   ├── drawing.rs          # Drawing primitives
│   └── zero_cost.rs        # Zero-cost abstractions
│
├── utils/                  # Utilities & Helpers
│   ├── mod.rs              # Utils exports
│   ├── memory.rs           # Memory management
│   ├── threading.rs        # Thread safety
│   └── macros.rs           # Helper macros
│
├── lib.rs                  # Library root
├── application.rs          # Application type
├── window.rs               # Window management
├── simple_app.rs           # High-level API
├── menu.rs                 # Menu system
├── async_ui.rs             # Async UI operations
├── streaming.rs            # Streaming APIs
└── macros.rs               # Procedural macros
```

## Module Purposes

### `core/`
Foundation types and traits that everything else depends on.
- **error.rs** - Error types and Result alias
- **traits.rs** - Foundation traits (Drawable, Textual, Positionable, etc.)
- **utils.rs** - Core utility functions

### `components/`
All GUI component types organized by category.
- **basic/** - Button, Label, TextField
- **advanced/** - Checkbox, Radio, Slider, etc.
- **containers/** - ScrollView, TabView, SplitView, GroupBox
- **data_display/** - TableView, OutlineView, CollectionView

### `systems/`
Core systems that power the framework.
- **events.rs** - Event system with callbacks
- **layout.rs** - Auto Layout constraints
- **animation.rs** - Core Animation wrapper
- **data_binding.rs** - Reactive data updates
- **builder.rs** - Builder patterns

### `features/`
macOS-specific features and integrations.
- **macos/** - Native macOS features
- **styling.rs** - Design system (Carbon)
- **drawing.rs** - Drawing primitives
- **zero_cost.rs** - Zero-cost abstractions

### `utils/`
Utilities and helper types.
- **memory.rs** - Memory management (ARC)
- **threading.rs** - Thread safety
- **macros.rs** - Helper macros

### Root Level
High-level APIs and integration points.
- **lib.rs** - Library root and prelude
- **application.rs** - Application type
- **window.rs** - Window management
- **simple_app.rs** - High-level SimpleApp API
- **menu.rs** - Menu system
- **async_ui.rs** - Async UI operations
- **streaming.rs** - Streaming APIs

## Migration Guide

### Old Structure → New Structure

```
controls.rs              → components/basic/button.rs, label.rs, textfield.rs
controls_v2.rs          → components/basic/mod.rs
checkbox.rs             → components/advanced/checkbox.rs
radio.rs                → components/advanced/radio.rs
slider.rs               → components/advanced/slider.rs
advanced_controls.rs    → components/advanced/mod.rs
containers.rs           → components/containers/mod.rs
data_display.rs         → components/data_display/mod.rs

events.rs               → systems/events.rs
layout.rs               → systems/layout.rs
builder.rs              → systems/builder.rs
essential_features.rs   → systems/mod.rs (animation, data_binding)

macos_features.rs       → features/macos/mod.rs
macos_integration.rs    → features/macos/mod.rs
styling.rs              → features/styling.rs
drawing.rs              → features/drawing.rs
zero_cost.rs            → features/zero_cost.rs

core_fixes.rs           → utils/memory.rs, threading.rs
utils.rs                → utils/mod.rs
error.rs                → core/error.rs
traits.rs               → core/traits.rs
```

## Import Patterns

### From Core
```rust
use cocoanut::core::{Result, Drawable, Textual};
```

### From Components
```rust
use cocoanut::components::{Button, Label, TextField};
use cocoanut::components::advanced::{Checkbox, Slider};
use cocoanut::components::containers::TabView;
```

### From Systems
```rust
use cocoanut::systems::{EventSystem, AutoLayout, Animation, DataBinding};
```

### From Features
```rust
use cocoanut::features::{NativeFeel, DarkMode, TouchBar};
use cocoanut::features::styling::CarbonColor;
```

### From Utils
```rust
use cocoanut::utils::{MemoryManager, ThreadSafeView};
```

## Benefits

1. **Clear Organization** - Easy to find related code
2. **Reduced Coupling** - Components don't depend on each other
3. **Better Discoverability** - Logical grouping helps new developers
4. **Easier Testing** - Isolated modules are easier to test
5. **Scalability** - Easy to add new components/systems
6. **Maintainability** - Related code is grouped together

## Adding New Components

1. Create a new file in the appropriate `components/` subdirectory
2. Implement the component with trait support
3. Add to the subdirectory's `mod.rs`
4. Export from `components/mod.rs`
5. Add to prelude in `lib.rs` if needed

## Adding New Systems

1. Create a new file in `systems/`
2. Implement the system
3. Add to `systems/mod.rs`
4. Export from `systems/mod.rs`
5. Add to prelude in `lib.rs` if needed

## Adding New Features

1. Create a new file or subdirectory in `features/`
2. Implement the feature
3. Add to `features/mod.rs`
4. Export from `features/mod.rs`
5. Add to prelude in `lib.rs` if needed
