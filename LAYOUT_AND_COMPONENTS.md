# Cocoanut Layout & Components Guide

## Overview

Cocoanut provides a **Streamlit-inspired layout system** with support for interactive components including checkboxes, dropdowns, and multi-line text areas. All components are automatically positioned and sized with intelligent boundary checking.

---

## Layout System

### Streamlit-Inspired Single-Column Layout

Components flow vertically in a single column with automatic spacing and boundary management.

```
Window (700x1000)
├─ Top Padding: 40px
├─ Component 1 (full width - margins)
├─ Gap: 12px
├─ Component 2 (full width - margins)
├─ Gap: 12px
├─ Component 3 (full width - margins)
└─ Bottom Padding: 20px (minimum)
```

### Layout Presets

**Default (Balanced Spacing):**
```rust
Layout::default()
// top_padding: 40px, margin: 20px, gap: 12px
```

**Compact (Tight Spacing):**
```rust
Layout::compact()
// top_padding: 20px, margin: 10px, gap: 8px
```

**Spacious (Generous Spacing):**
```rust
Layout::spacious()
// top_padding: 60px, margin: 40px, gap: 20px
```

### Usage

```rust
app("MyApp")
    .title("My Application")
    .size(700.0, 1000.0)
    .centered(true)
    .layout(Layout::default())  // or compact(), spacious()
    .add(Comp::new(Kind::Button).text("Click"))
    .add(Comp::new(Kind::Label).text("Status"))
    .run()?;
```

---

## Components

### 1. Checkboxes

**Purpose:** Boolean toggle controls

**Features:**
- ✅ Toggle on/off
- ✅ Display with checkmark symbol (☑)
- ✅ Customizable text
- ✅ Configurable size

**Example:**
```rust
.add(Comp::new(Kind::Checkbox)
    .text("☑ Enable notifications")
    .size(300.0, 25.0))
```

**Output:**
```
☑ Enable notifications
☑ Auto-save enabled
☑ Dark mode
```

---

### 2. Dropdowns

**Purpose:** Selection from multiple options

**Features:**
- ✅ Smart choice detection based on title
- ✅ Multiple predefined choice sets
- ✅ Click to open menu
- ✅ Select from options
- ✅ Display selected value

**Smart Choices:**
- **"theme"** → Light, Dark, Auto
- **"language"** → English, Spanish, French, German
- **"size"** or **"Font"** → Small, Medium, Large, Extra Large
- **Other** → Option 1, Option 2, Option 3

**Example:**
```rust
.add(Comp::new(Kind::Dropdown)
    .text("▼ Select theme")
    .size(300.0, 30.0))

.add(Comp::new(Kind::Dropdown)
    .text("▼ Choose language")
    .size(300.0, 30.0))

.add(Comp::new(Kind::Dropdown)
    .text("▼ Font size")
    .size(300.0, 30.0))
```

**Output:**
```
▼ Select theme
  • Light
  • Dark
  • Auto

▼ Choose language
  • English
  • Spanish
  • French
  • German

▼ Font size
  • Small
  • Medium
  • Large
  • Extra Large
```

---

### 3. TextArea (Multi-line Text Box)

**Purpose:** Multi-line text input

**Features:**
- ✅ Multiple lines of text
- ✅ Word wrapping
- ✅ Editable content
- ✅ Selectable text
- ✅ Newline preservation
- ✅ White background
- ✅ Vertical scrolling

**Configuration:**
- `setEditable:true` - Allow editing
- `setSelectable:true` - Allow selection
- `setHorizontallyResizable:false` - No horizontal scroll
- `setVerticallyResizable:true` - Vertical scroll
- `setBackgroundColor:white` - Visible text area

**Example:**
```rust
.add(Comp::new(Kind::TextArea)
    .text("Line 1: Enter your feedback here...\nLine 2: You can write multiple lines\nLine 3: This is a multi-line text area\nLine 4: Keep typing for more content")
    .size(650.0, 200.0))
```

**Output:**
```
Line 1: Enter your feedback here...
Line 2: You can write multiple lines
Line 3: This is a multi-line text area
Line 4: Keep typing for more content
```

---

## Boundary & Overflow Management

### Automatic Boundary Checking

**Vertical Overflow Prevention:**
```rust
// Check if component would overflow
let next_y = y_position - comp.height - layout.gap;
if next_y < bottom_padding {
    println!("⚠️  Component would overflow - skipping");
    continue;
}
```

**Horizontal Overflow Prevention:**
```rust
// Adapt component width to available space
let available_width = window_width - (margin * 2.0);
let comp_width = if comp.width > available_width {
    available_width  // Clamp to available width
} else {
    comp.width
};
```

### Boundary Constraints

- **Top:** `layout.top_padding` (40px default)
- **Left/Right:** `layout.horizontal_margin` (20px default)
- **Bottom:** 20px (fixed minimum)
- **Component spacing:** `layout.gap` (12px default)

### Output Information

```
✓ Window created (700x1000)
Adding 16 component(s)...
  ✓ Label added: "📋 CHECKBOXES" (400x25)
  ✓ Checkbox added: "☑ Enable notifications" (300x25)
  ...
  ℹ️  16 of 16 components displayed (window height: 1000px)
```

---

## Complete Example

```rust
use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    app("Interactive Controls")
        .title("🥥 Cocoanut - Interactive Controls Demo")
        .size(700.0, 1000.0)
        .centered(true)
        .layout(Layout::default())
        
        // Checkboxes Section
        .add(Comp::new(Kind::Label).text("📋 CHECKBOXES:").size(400.0, 25.0))
        .add(Comp::new(Kind::Checkbox).text("☑ Enable notifications").size(300.0, 25.0))
        .add(Comp::new(Kind::Checkbox).text("☑ Auto-save enabled").size(300.0, 25.0))
        .add(Comp::new(Kind::Checkbox).text("☑ Dark mode").size(300.0, 25.0))
        
        // Dropdowns Section
        .add(Comp::new(Kind::Label).text("🎯 DROPDOWNS:").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Theme: Light • Dark • Auto").size(400.0, 20.0))
        .add(Comp::new(Kind::Dropdown).text("▼ Select theme").size(300.0, 30.0))
        
        .add(Comp::new(Kind::Label).text("Language: English • Spanish • French • German").size(400.0, 20.0))
        .add(Comp::new(Kind::Dropdown).text("▼ Choose language").size(300.0, 30.0))
        
        // TextArea Section
        .add(Comp::new(Kind::Label).text("📝 TEXTAREA:").size(400.0, 25.0))
        .add(Comp::new(Kind::TextArea)
            .text("Line 1: Enter your feedback here...\nLine 2: You can write multiple lines\nLine 3: This is a multi-line text area")
            .size(650.0, 200.0))
        
        // Action Buttons
        .add(Comp::new(Kind::Button).text("✓ Submit").size(140.0, 40.0))
        .add(Comp::new(Kind::Button).text("↻ Reset").size(140.0, 40.0))
        
        .run()?;

    Ok(())
}
```

---

## Window Close Behavior

### Graceful Shutdown

The application now supports graceful shutdown through multiple methods:

**Method 1: Close Button**
- Click the red close button (X) in window title bar
- Window closes immediately
- Application terminates gracefully

**Method 2: Keyboard Shortcut**
- Press `Cmd+Q`
- Application terminates gracefully

**Implementation:**
```rust
// Enable window close button
let _: () = msg_send![window.ns_window(), setReleasedWhenClosed:true];

// Make close button terminate the app
let ns_window = window.ns_window();
let _: () = msg_send![app, setDelegate:ns_window];
```

---

## Component Sizing Guidelines

### Recommended Sizes

**Labels:**
- Width: 300-400px (content-dependent)
- Height: 20-25px

**Checkboxes:**
- Width: 250-300px
- Height: 25px

**Dropdowns:**
- Width: 250-300px
- Height: 30px

**TextArea:**
- Width: 600-650px (near full width)
- Height: 150-200px (multi-line)

**Buttons:**
- Width: 100-150px
- Height: 35-40px

---

## Layout Calculation Formula

```
Y Position Calculation:
y_position = window_height - title_bar_height - top_padding

Component Placement:
component_y = y_position - component_height

Next Position:
y_position = component_y - gap

Width Calculation:
available_width = window_width - (horizontal_margin * 2)
component_width = min(requested_width, available_width)
```

---

## Features Summary

| Feature | Status | Details |
|---------|--------|---------|
| Streamlit Layout | ✅ | Single column, automatic flow |
| Boundary Checking | ✅ | Prevents overflow |
| Checkboxes | ✅ | Toggle controls |
| Dropdowns | ✅ | Smart choice detection |
| TextArea | ✅ | Multi-line editing |
| Graceful Close | ✅ | Close button + Cmd+Q |
| Layout Presets | ✅ | Default, Compact, Spacious |
| Component Sizing | ✅ | Adaptive width, fixed height |
| Responsive | ✅ | Adapts to window size |

---

## Running the Example

```bash
cargo run --example interactive_controls
```

**Output:**
- Real macOS GUI window
- 16 components displayed
- All interactive
- Closes gracefully

---

## Next Steps

1. **Event Handlers** - Add click/change callbacks
2. **Data Binding** - Reactive updates
3. **Validation** - Input validation
4. **Styling** - Custom colors and fonts
5. **Accessibility** - VoiceOver support
