# Cocoanut Improvement Roadmap

## 🎯 Vision: Make Cocoanut the Easiest Way to Build macOS GUIs in Rust

This roadmap outlines improvements to make Cocoanut easier to use and cover all major macOS GUI components.

## 📊 Current State

### ✅ What We Have
- Basic window management
- Button, Label, TextField controls
- Builder patterns for components
- VStack/HStack layout system
- Carbon Design System styling
- Menu system
- Event handling basics
- Drawing utilities
- Async/await support
- Streaming APIs
- Zero-cost abstractions
- 169 comprehensive tests
- Excellent documentation

### ❌ What's Missing
- More UI controls (Checkbox, Radio, Slider, etc.)
- Table/Collection views
- Advanced layout (Grid, Scroll)
- Event binding convenience
- Window builder
- Macros for UI declaration
- Hot reload support
- More macOS features (Touch Bar, Accessibility)

## 🚀 Phase 1: Ease of Use (Immediate)

### 1.1 Window Builder
**Goal**: Make window creation as simple as components

```rust
// Current
let window = Window::new("Title", 800.0, 600.0)?;

// Proposed
let window = Window::builder()
    .title("My App")
    .size(800.0, 600.0)
    .center()
    .resizable(true)
    .build()?;
```

**Implementation**:
- Create `WindowBuilder` in `src/builder.rs`
- Add methods: `title()`, `size()`, `center()`, `resizable()`, `minimizable()`, `closable()`
- Support fluent API

**Effort**: 2-3 hours
**Tests**: 8 new tests

### 1.2 Simplified Event Binding
**Goal**: Make event handling intuitive

```rust
// Current
let mut button = Button::builder().title("Click").build()?;
// Manual event handling...

// Proposed
let button = Button::builder()
    .title("Click")
    .on_click(|| println!("Clicked!"))
    .build()?;
```

**Implementation**:
- Add `on_click()` method to `ButtonBuilder`
- Add `on_change()` method to `TextFieldBuilder`
- Store closures in component state
- Trigger on events

**Effort**: 3-4 hours
**Tests**: 10 new tests

### 1.3 Quick App Macro
**Goal**: Create apps in 5 lines

```rust
cocoanut::app! {
    window("My App", 800.0, 600.0) {
        vstack {
            button("Click me").on_click(|| println!("Hi!"))
            label("Hello, Cocoanut!")
        }
    }
}
```

**Implementation**:
- Create `src/macros.rs` with `app!` macro
- Support declarative syntax
- Generate builder code

**Effort**: 4-5 hours
**Tests**: 5 new tests

## 🎨 Phase 2: More Components (1-2 weeks)

### 2.1 Essential Controls
Add these missing controls:

| Control | Priority | Effort | Tests |
|---------|----------|--------|-------|
| Checkbox | High | 2h | 5 |
| Radio Button | High | 2h | 5 |
| Slider | High | 2h | 5 |
| Segmented Control | Medium | 2h | 4 |
| Stepper | Medium | 1.5h | 4 |
| Switch | Medium | 1.5h | 4 |
| Popup Button | Medium | 2h | 4 |
| Combo Box | Low | 2h | 4 |

**Total Effort**: ~15 hours
**Total Tests**: ~35 new tests

### 2.2 Container Views
Add container components:

| Container | Priority | Effort | Tests |
|-----------|----------|--------|-------|
| ScrollView | High | 3h | 6 |
| TabView | High | 3h | 6 |
| SplitView | Medium | 3h | 5 |
| GroupBox | Medium | 2h | 4 |
| Box | Low | 1h | 2 |

**Total Effort**: ~12 hours
**Total Tests**: ~23 new tests

### 2.3 Data Display
Add data-driven components:

| Component | Priority | Effort | Tests |
|-----------|----------|--------|-------|
| TableView | High | 5h | 8 |
| OutlineView | Medium | 4h | 6 |
| CollectionView | Low | 5h | 6 |

**Total Effort**: ~14 hours
**Total Tests**: ~20 new tests

## 🎯 Phase 3: Advanced Features (2-3 weeks)

### 3.1 Layout Improvements
- [ ] GridView for grid layouts
- [ ] Spacer improvements
- [ ] Constraint-based layout
- [ ] Responsive layouts
- [ ] Safe area support

**Effort**: 8 hours
**Tests**: 15 new tests

### 3.2 macOS Integration
- [ ] Touch Bar support
- [ ] Accessibility (VoiceOver)
- [ ] Dark mode support
- [ ] Handoff support
- [ ] Drag & drop

**Effort**: 12 hours
**Tests**: 20 new tests

### 3.3 Advanced Styling
- [ ] Themes (Light/Dark)
- [ ] Custom fonts
- [ ] Gradients
- [ ] Shadows
- [ ] Animations

**Effort**: 10 hours
**Tests**: 15 new tests

## 📋 Implementation Priority

### Week 1: Ease of Use
1. Window builder (2-3h)
2. Event binding (3-4h)
3. Quick app macro (4-5h)
4. Tests & docs (2h)

**Total**: 11-14 hours → ~1 week

### Week 2-3: Essential Components
1. Basic controls (15h)
2. Container views (12h)
3. Tests & docs (5h)

**Total**: 32 hours → ~2 weeks

### Week 4-5: Advanced Features
1. Layout improvements (8h)
2. macOS integration (12h)
3. Advanced styling (10h)
4. Tests & docs (5h)

**Total**: 35 hours → ~2 weeks

## 🛠️ Implementation Details

### Window Builder Example
```rust
// src/builder.rs - Add WindowBuilder

pub struct WindowBuilder {
    title: String,
    width: f64,
    height: f64,
    center: bool,
    resizable: bool,
    minimizable: bool,
    closable: bool,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            title: "Window".to_string(),
            width: 800.0,
            height: 600.0,
            center: false,
            resizable: true,
            minimizable: true,
            closable: true,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    pub fn build(self) -> Result<Window> {
        let mut window = Window::new(&self.title, self.width, self.height)?;
        if self.center {
            window.center()?;
        }
        Ok(window)
    }
}
```

### Event Binding Example
```rust
// src/builder.rs - Add to ButtonBuilder

pub struct ButtonBuilder {
    title: String,
    size: Option<(f64, f64)>,
    enabled: bool,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
}

impl ButtonBuilder {
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    pub fn build(self) -> Result<Button> {
        let mut button = Button::new(&self.title)?;
        if let Some((w, h)) = self.size {
            button.set_size(w, h)?;
        }
        if let Some(callback) = self.on_click {
            button.set_click_handler(callback)?;
        }
        Ok(button)
    }
}
```

### Quick App Macro Example
```rust
// src/macros.rs

#[macro_export]
macro_rules! app {
    (
        window($title:expr, $width:expr, $height:expr) {
            $($content:tt)*
        }
    ) => {
        {
            let app = $crate::Application::new($title)?;
            let window = $crate::Window::builder()
                .title($title)
                .size($width, $height)
                .build()?;
            
            $($content)*
            
            app.run(window)?;
        }
    };
}
```

## 📊 Coverage Goals

### Current Components
- ✅ Button
- ✅ Label
- ✅ TextField
- ✅ Window
- ✅ Menu/MenuItem
- ✅ VStack/HStack

### Phase 2 Components (New)
- ⬜ Checkbox
- ⬜ Radio Button
- ⬜ Slider
- ⬜ Segmented Control
- ⬜ Stepper
- ⬜ Switch
- ⬜ Popup Button
- ⬜ ScrollView
- ⬜ TabView
- ⬜ SplitView
- ⬜ TableView

### Phase 3 Components (Advanced)
- ⬜ OutlineView
- ⬜ CollectionView
- ⬜ GridView
- ⬜ WebView
- ⬜ ImageView
- ⬜ ProgressBar

## 🎯 Success Metrics

### Ease of Use
- [ ] Can build complete app in <50 lines
- [ ] New users productive in <1 hour
- [ ] 90% of common tasks have examples

### Coverage
- [ ] 20+ UI components
- [ ] 5+ container types
- [ ] 3+ data display options

### Quality
- [ ] 200+ tests
- [ ] 90%+ test coverage
- [ ] Zero compilation warnings (in examples)

### Documentation
- [ ] 50+ examples
- [ ] Complete API reference
- [ ] 10+ tutorials

## 📈 Timeline

| Phase | Duration | Components | Tests | Docs |
|-------|----------|------------|-------|------|
| 1: Ease | 1 week | 0 new | 23 | 10 |
| 2: Components | 2 weeks | 15 new | 78 | 20 |
| 3: Advanced | 2 weeks | 6 new | 50 | 15 |
| **Total** | **5 weeks** | **21 new** | **151** | **45** |

## 🚀 Quick Wins (This Week)

1. **Window Builder** (2h)
   - Add `WindowBuilder` struct
   - Implement fluent API
   - Add 8 tests

2. **Event Binding** (3h)
   - Add `on_click()` to `ButtonBuilder`
   - Add `on_change()` to `TextFieldBuilder`
   - Add 10 tests

3. **Documentation** (2h)
   - Update examples
   - Add tutorial
   - Update README

**Total**: 7 hours → Significant UX improvement

## 🎓 Learning Path for Users

1. **Beginner** (30 min)
   - Read "Why Cocoanut?"
   - Run quick app macro example
   - Build simple window

2. **Intermediate** (2 hours)
   - Learn builder patterns
   - Explore layout system
   - Build multi-control app

3. **Advanced** (4 hours)
   - Event binding
   - Custom styling
   - Table views
   - macOS integration

## 💡 Key Principles

1. **Simplicity First** - Easy things should be easy
2. **Consistency** - All APIs follow same patterns
3. **Discoverability** - IDE autocomplete should guide users
4. **Performance** - Zero-cost abstractions
5. **Safety** - No unsafe code in public API
6. **Documentation** - Every feature has examples

## 🎉 Vision Statement

> Cocoanut makes macOS GUI development in Rust as simple as writing Swift, with the safety and performance of Rust, and the educational value of understanding Cocoa.

---

## Next Steps

1. **Implement Window Builder** (Priority 1)
2. **Add Event Binding** (Priority 1)
3. **Create Quick App Macro** (Priority 2)
4. **Add Essential Controls** (Priority 2)
5. **Expand Documentation** (Ongoing)

**Start Date**: Immediately
**Target Completion**: 5 weeks
