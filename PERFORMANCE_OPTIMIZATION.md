# Performance Optimization Guide

## Overview

This document describes performance optimization strategies for Cocoanut, including profiling hot paths, memory optimization, and compilation improvements.

---

## Part 1: Hot Path Profiling

### Identifying Hot Paths

Hot paths are code sections that execute frequently and consume significant CPU time.

#### Common Hot Paths in Cocoanut

1. **Event System** - Fires on every user interaction
2. **Layout System** - Recalculates on window resize
3. **Rendering** - Redraws on every frame
4. **String Conversion** - NSString ↔ Rust string conversions
5. **Memory Management** - Retain/release operations

### Profiling Tools

#### Using Instruments (macOS)

```bash
# Profile with Instruments
cargo build --release
instruments -t "System Trace" ./target/release/cocoanut_app

# Profile with Time Profiler
instruments -t "Time Profiler" ./target/release/cocoanut_app
```

#### Using flamegraph

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --release

# View result
open flamegraph.svg
```

#### Using perf (Linux)

```bash
# Profile with perf
perf record -g ./target/release/cocoanut_app
perf report
```

### Optimization Techniques

#### 1. Caching

```rust
// Before: Recalculates every time
fn get_class_name(obj: *mut Object) -> Result<String> {
    unsafe { cstring_to_string(get_objc_class_name(obj)) }
}

// After: Cache the result
pub struct CachedObject {
    obj: *mut Object,
    class_name: Option<String>,
}

impl CachedObject {
    pub fn class_name(&mut self) -> Result<&str> {
        if self.class_name.is_none() {
            self.class_name = Some(unsafe { 
                cstring_to_string(get_objc_class_name(self.obj))? 
            });
        }
        Ok(self.class_name.as_ref().unwrap())
    }
}
```

#### 2. Lazy Initialization

```rust
// Before: Initialize everything upfront
pub struct Component {
    event_system: EventSystem,
    layout_system: LayoutSystem,
    animation_system: AnimationSystem,
}

// After: Initialize on demand
pub struct Component {
    event_system: Option<EventSystem>,
    layout_system: Option<LayoutSystem>,
    animation_system: Option<AnimationSystem>,
}

impl Component {
    pub fn event_system(&mut self) -> &mut EventSystem {
        if self.event_system.is_none() {
            self.event_system = Some(EventSystem::new());
        }
        self.event_system.as_mut().unwrap()
    }
}
```

#### 3. Batch Operations

```rust
// Before: Multiple individual operations
for item in items {
    component.add_item(item)?;
}

// After: Batch operation
component.add_items(items)?;
```

#### 4. Early Exit

```rust
// Before: Process everything
fn process(items: Vec<Item>) -> Result<()> {
    for item in items {
        validate_item(&item)?;
        process_item(&item)?;
    }
    Ok(())
}

// After: Exit early on error
fn process(items: Vec<Item>) -> Result<()> {
    // Validate all first
    for item in &items {
        validate_item(item)?;
    }
    // Then process
    for item in items {
        process_item(&item)?;
    }
    Ok(())
}
```

---

## Part 2: Memory Optimization

### Memory Profiling

#### Using Valgrind (Linux)

```bash
valgrind --leak-check=full ./target/debug/cocoanut_app
```

#### Using Heaptrack (Linux)

```bash
heaptrack ./target/debug/cocoanut_app
heaptrack_gui heaptrack.cocoanut_app.*.gz
```

#### Using Xcode Instruments (macOS)

```bash
# Memory profiling
instruments -t "Allocations" ./target/release/cocoanut_app
```

### Memory Optimization Techniques

#### 1. String Interning

```rust
// Before: Duplicate strings
pub struct Button {
    id: String,  // "button_1"
    name: String,  // "button_1"
}

// After: Shared strings
use std::sync::Arc;

pub struct Button {
    id: Arc<str>,
    name: Arc<str>,
}
```

#### 2. Object Pooling

```rust
// Before: Create/destroy on each use
pub struct EventPool {
    events: Vec<UIEvent>,
}

impl EventPool {
    pub fn acquire(&mut self) -> UIEvent {
        self.events.pop().unwrap_or_default()
    }
    
    pub fn release(&mut self, event: UIEvent) {
        self.events.push(event);
    }
}
```

#### 3. Reduce Allocations

```rust
// Before: Multiple allocations
fn process(items: &[Item]) -> Result<Vec<String>> {
    let mut results = Vec::new();
    for item in items {
        let name = item.name.clone();  // Allocation
        results.push(name);
    }
    Ok(results)
}

// After: Reuse references
fn process(items: &[Item]) -> Result<Vec<&str>> {
    items.iter().map(|item| Ok(item.name.as_str())).collect()
}
```

#### 4. Smart Pointers

```rust
// Before: Manual memory management
pub struct Component {
    ns_view: *mut Object,
}

// After: Automatic with Arc
use std::sync::Arc;

pub struct Component {
    ns_view: Arc<NSView>,
}
```

#### 5. Stack Allocation

```rust
// Before: Heap allocation
let large_buffer = Box::new([0u8; 1024]);

// After: Stack allocation (if size is known)
let large_buffer = [0u8; 1024];
```

---

## Part 3: Compilation Optimization

### Build Time Optimization

#### 1. Incremental Compilation

```toml
# Cargo.toml
[profile.dev]
incremental = true

[profile.release]
incremental = true
```

#### 2. Parallel Compilation

```bash
# Use all available cores
CARGO_BUILD_JOBS=8 cargo build

# Or set in .cargo/config.toml
[build]
jobs = 8
```

#### 3. Link-Time Optimization (LTO)

```toml
# Cargo.toml
[profile.release]
lto = true
codegen-units = 1
```

#### 4. Reduce Dependencies

```bash
# Find unused dependencies
cargo tree --duplicates

# Remove unused crates
cargo remove unused_crate
```

#### 5. Feature Flags

```toml
# Cargo.toml
[features]
default = ["core"]
core = []
advanced = ["core"]
full = ["core", "advanced"]
```

### Runtime Optimization

#### 1. Release Profile

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

#### 2. Inline Hints

```rust
// Hot path - inline
#[inline]
pub fn is_null(ptr: *mut Object) -> bool {
    ptr.is_null()
}

// Cold path - don't inline
#[inline(never)]
pub fn handle_error(error: CocoanutError) {
    eprintln!("Error: {}", error);
}
```

#### 3. Avoid Allocations in Loops

```rust
// Before: Allocation in loop
for _ in 0..1000 {
    let vec = Vec::new();  // Allocates each iteration
    process(&vec)?;
}

// After: Allocate once
let mut vec = Vec::new();
for _ in 0..1000 {
    vec.clear();
    process(&vec)?;
}
```

---

## Part 4: Benchmarking

### Creating Benchmarks

```rust
#[cfg(test)]
mod benches {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn bench_string_conversion() {
        let iterations = 10_000;
        let start = Instant::now();
        
        for i in 0..iterations {
            let s = format!("string_{}", i);
            let _ = string_to_cstring(&s);
        }
        
        let elapsed = start.elapsed();
        println!("Time: {:?}, Per iteration: {:?}", 
                 elapsed, 
                 elapsed / iterations as u32);
    }
}
```

### Using Criterion

```toml
# Cargo.toml
[dev-dependencies]
criterion = "0.5"
```

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn string_conversion_benchmark(c: &mut Criterion) {
    c.bench_function("string_to_cstring", |b| {
        b.iter(|| string_to_cstring(black_box("test")))
    });
}

criterion_group!(benches, string_conversion_benchmark);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

---

## Part 5: Performance Checklist

### Before Optimization
- [ ] Identify bottlenecks with profiling
- [ ] Measure baseline performance
- [ ] Document current metrics

### During Optimization
- [ ] Change one thing at a time
- [ ] Measure impact of each change
- [ ] Keep changes that improve performance
- [ ] Revert changes that don't help

### After Optimization
- [ ] Document optimizations made
- [ ] Update benchmarks
- [ ] Compare before/after metrics
- [ ] Share results with team

---

## Part 6: Performance Targets

### Cocoanut Performance Goals

| Operation | Target | Current | Status |
|-----------|--------|---------|--------|
| Component Creation | < 1ms | TBD | ⏳ |
| Event Emission | < 0.1ms | TBD | ⏳ |
| Layout Calculation | < 5ms | TBD | ⏳ |
| String Conversion | < 0.01ms | TBD | ⏳ |
| Memory per Component | < 1KB | TBD | ⏳ |
| Build Time (Release) | < 30s | TBD | ⏳ |

---

## Part 7: Common Pitfalls

### ❌ Don't

- Optimize prematurely without profiling
- Sacrifice readability for micro-optimizations
- Ignore memory leaks
- Create unnecessary copies
- Use inefficient algorithms

### ✅ Do

- Profile first, optimize second
- Maintain code clarity
- Test performance improvements
- Use appropriate data structures
- Consider algorithmic improvements

---

## Part 8: Tools and Resources

### Profiling Tools
- **Instruments** (macOS) - Built-in profiler
- **flamegraph** - Visualize CPU usage
- **perf** (Linux) - Performance analysis
- **Valgrind** (Linux) - Memory profiling

### Benchmarking
- **Criterion** - Statistical benchmarking
- **Bencher** - Continuous benchmarking
- **Flamegraph** - Flame graphs

### Documentation
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/)
- [Flamegraph](https://www.brendangregg.com/flamegraphs.html)

---

## Summary

Performance optimization is an iterative process:

1. **Profile** - Identify bottlenecks
2. **Optimize** - Apply targeted improvements
3. **Measure** - Verify improvements
4. **Repeat** - Continue optimizing

Focus on:
- ✅ Hot paths (high frequency, high cost)
- ✅ Memory efficiency (reduce allocations)
- ✅ Compilation speed (faster iteration)

---

**Last Updated:** October 25, 2025  
**Version:** 1.0  
**Status:** Active
