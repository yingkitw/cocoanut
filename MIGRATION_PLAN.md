# Source Code Reorganization Migration Plan

## Overview

This document outlines the plan to reorganize the `src/` folder from a flat structure to a hierarchical module structure for better maintainability.

## Current State

**31 files** in `src/` directory:
- Core: error.rs, traits.rs, utils.rs
- Components: controls.rs, controls_v2.rs, checkbox.rs, radio.rs, slider.rs, advanced_controls.rs, containers.rs, data_display.rs
- Systems: events.rs, layout.rs, builder.rs, essential_features.rs, core_fixes.rs
- Features: macos_features.rs, macos_integration.rs, styling.rs, drawing.rs, zero_cost.rs, phase3_features.rs
- High-level: application.rs, window.rs, simple_app.rs, menu.rs, async_ui.rs, streaming.rs, macros.rs

## Target State

**Organized into 5 main modules:**
- `core/` - Foundation types and traits
- `components/` - GUI components (basic, advanced, containers, data_display)
- `systems/` - Core systems (events, layout, animation, data_binding, builder)
- `features/` - macOS features (macos, styling, drawing, zero_cost)
- `utils/` - Utilities (memory, threading, macros)

## Migration Steps

### Phase 1: Create Module Structure (COMPLETED)
- [x] Create `core/mod.rs`
- [x] Create `components/mod.rs`
- [x] Create `systems/mod.rs`
- [x] Create `features/mod.rs`
- [x] Create `utils/mod.rs`
- [x] Create `SRC_ORGANIZATION.md`
- [x] Create `MIGRATION_PLAN.md`

### Phase 2: Move Core Files
- [ ] Move `error.rs` → `core/error.rs`
- [ ] Move `traits.rs` → `core/traits.rs`
- [ ] Move `utils.rs` → `core/utils.rs`
- [ ] Update `core/mod.rs` with proper exports

### Phase 3: Move Component Files
- [ ] Create `components/basic/mod.rs`
- [ ] Move `controls.rs` → `components/basic/button.rs`
- [ ] Move `controls_v2.rs` → `components/basic/mod.rs`
- [ ] Create `components/advanced/mod.rs`
- [ ] Move `checkbox.rs` → `components/advanced/checkbox.rs`
- [ ] Move `radio.rs` → `components/advanced/radio.rs`
- [ ] Move `slider.rs` → `components/advanced/slider.rs`
- [ ] Move `advanced_controls.rs` → `components/advanced/mod.rs`
- [ ] Create `components/containers/mod.rs`
- [ ] Move `containers.rs` → `components/containers/mod.rs`
- [ ] Create `components/data_display/mod.rs`
- [ ] Move `data_display.rs` → `components/data_display/mod.rs`
- [ ] Update `components/mod.rs` with proper exports

### Phase 4: Move System Files
- [ ] Move `events.rs` → `systems/events.rs`
- [ ] Move `layout.rs` → `systems/layout.rs`
- [ ] Move `builder.rs` → `systems/builder.rs`
- [ ] Move `essential_features.rs` → `systems/animation.rs` and `systems/data_binding.rs`
- [ ] Update `systems/mod.rs` with proper exports

### Phase 5: Move Feature Files
- [ ] Create `features/macos/mod.rs`
- [ ] Move `macos_features.rs` → `features/macos/mod.rs`
- [ ] Move `macos_integration.rs` → `features/macos/integration.rs`
- [ ] Move `styling.rs` → `features/styling.rs`
- [ ] Move `drawing.rs` → `features/drawing.rs`
- [ ] Move `zero_cost.rs` → `features/zero_cost.rs`
- [ ] Move `phase3_features.rs` → `features/phase3.rs`
- [ ] Update `features/mod.rs` with proper exports

### Phase 6: Move Utility Files
- [ ] Create `utils/memory.rs` (from `core_fixes.rs`)
- [ ] Create `utils/threading.rs` (from `core_fixes.rs`)
- [ ] Move `macros.rs` → `utils/macros.rs`
- [ ] Update `utils/mod.rs` with proper exports

### Phase 7: Update lib.rs
- [ ] Update module declarations
- [ ] Update prelude exports
- [ ] Update public API

### Phase 8: Update Imports
- [ ] Update all internal imports
- [ ] Update example imports
- [ ] Update test imports

### Phase 9: Verification
- [ ] Run `cargo build` - should succeed
- [ ] Run `cargo test` - all tests should pass
- [ ] Run `cargo build --examples` - all examples should build
- [ ] Run examples - all should run correctly

## File Mapping

### Core Module
```
error.rs              → core/error.rs
traits.rs             → core/traits.rs
utils.rs              → core/utils.rs
```

### Components Module
```
controls.rs           → components/basic/button.rs
controls_v2.rs        → components/basic/mod.rs
checkbox.rs           → components/advanced/checkbox.rs
radio.rs              → components/advanced/radio.rs
slider.rs             → components/advanced/slider.rs
advanced_controls.rs  → components/advanced/mod.rs
containers.rs         → components/containers/mod.rs
data_display.rs       → components/data_display/mod.rs
```

### Systems Module
```
events.rs             → systems/events.rs
layout.rs             → systems/layout.rs
builder.rs            → systems/builder.rs
essential_features.rs → systems/animation.rs + systems/data_binding.rs
```

### Features Module
```
macos_features.rs     → features/macos/mod.rs
macos_integration.rs  → features/macos/integration.rs
styling.rs            → features/styling.rs
drawing.rs            → features/drawing.rs
zero_cost.rs          → features/zero_cost.rs
phase3_features.rs    → features/phase3.rs
```

### Utils Module
```
core_fixes.rs         → utils/memory.rs + utils/threading.rs
macros.rs             → utils/macros.rs
```

### Root Level (No Change)
```
lib.rs                (update imports)
application.rs        (no change)
window.rs             (no change)
simple_app.rs         (no change)
menu.rs               (no change)
async_ui.rs           (no change)
streaming.rs          (no change)
```

## Benefits

1. **Logical Organization** - Related code is grouped together
2. **Easier Navigation** - Clear structure helps developers find code
3. **Better Discoverability** - New developers can understand the codebase faster
4. **Reduced Coupling** - Components are isolated from each other
5. **Scalability** - Easy to add new components/systems
6. **Maintainability** - Related changes are in one place
7. **Testing** - Easier to write focused tests

## Timeline

- **Phase 1**: ✅ COMPLETED
- **Phase 2-6**: To be executed
- **Phase 7-9**: Verification and testing

## Rollback Plan

If issues arise, we can:
1. Keep the old flat structure as backup
2. Gradually move files and test
3. Revert individual files if needed
4. Keep both structures temporarily during transition

## Notes

- All functionality remains the same
- No breaking changes to public API
- Backward compatibility maintained
- All tests should continue to pass
- All examples should continue to work
