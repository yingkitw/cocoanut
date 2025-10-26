# Cocoanut Documentation Index

Complete documentation for the Cocoanut macOS GUI framework.

## 📍 Start Here

- **[README.md](README.md)** - Quick start, features, and examples
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System design, patterns, and module organization
- **[TODO.md](TODO.md)** - Project roadmap and completion status

## 📚 Documentation Folders

### [docs/guides/](docs/guides/)
Feature guides and API documentation:
- **SIMPLIFICATION_GUIDE.md** - API simplification patterns and before/after comparisons
- **API_IMPROVEMENTS.md** - Complete API reference and improvements
- **NEXT_IMPROVEMENTS.md** - Future enhancement roadmap

### [docs/examples/](docs/examples/)
Working examples and demonstrations:
- **EXAMPLES_AND_TESTS.md** - Complete examples guide with descriptions
- **COMPREHENSIVE_EXAMPLE_SUMMARY.md** - Full feature demonstration
- **LAYOUT_CONTAINERS_DEMO.md** - Layout system and containers
- **VISUAL_LAYOUT_DEMO.md** - Visual layout patterns
- **CONTAINERS_WITH_BORDERS.md** - Container styling and borders

### [docs/archive/](docs/archive/)
Historical documentation from completed phases:
- Phase migration completion reports
- Macro refactoring documentation
- Publication and release notes

## 🚀 Quick Start

### Installation
```bash
cargo add cocoanut
```

### Minimal Example
```rust
use cocoanut::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app("MyApp")
        .title("Hello Cocoanut")
        .size(600.0, 400.0)
        .centered(true)
        .run()?;
    Ok(())
}
```

### Run Examples
```bash
cargo build --examples
cargo run --example minimal_app
cargo run --example comprehensive_app
cargo run --example menu_app
```

## 📋 Key Topics

### Core Concepts
- **Trait-Based Architecture** - See [ARCHITECTURE.md](ARCHITECTURE.md#trait-based-architecture)
- **Builder Patterns** - See [docs/guides/SIMPLIFICATION_GUIDE.md](docs/guides/SIMPLIFICATION_GUIDE.md)
- **Component System** - See [ARCHITECTURE.md](ARCHITECTURE.md#gui-components)
- **Event System** - See [ARCHITECTURE.md](ARCHITECTURE.md#event-system)

### Features
- **67 Streamlit-Inspired Widgets** - See [TODO.md](TODO.md#streamlit-migration-complete)
- **macOS Integration** - See [ARCHITECTURE.md](ARCHITECTURE.md#macos-features)
- **Layout System** - See [docs/examples/LAYOUT_CONTAINERS_DEMO.md](docs/examples/LAYOUT_CONTAINERS_DEMO.md)
- **Styling** - See [ARCHITECTURE.md](ARCHITECTURE.md#styling-system)

### Development
- **Project Structure** - See [ARCHITECTURE.md](ARCHITECTURE.md#module-organization)
- **Testing** - See [TODO.md](TODO.md#testing)
- **Building** - See [README.md](README.md#building)

## 📊 Project Status

- ✅ **Core Framework** - Complete and production-ready
- ✅ **67 Widgets** - All Streamlit-inspired widgets implemented
- ✅ **260+ Tests** - Comprehensive test coverage
- ✅ **11 Examples** - Working examples demonstrating all features
- ✅ **Documentation** - Complete and well-organized

See [TODO.md](TODO.md) for detailed roadmap and completion status.

## 🔗 Navigation

### By Use Case

**I want to build a simple app:**
1. Read [README.md](README.md)
2. Run `cargo run --example minimal_app`
3. Check [docs/examples/EXAMPLES_AND_TESTS.md](docs/examples/EXAMPLES_AND_TESTS.md)

**I want to understand the design:**
1. Read [ARCHITECTURE.md](ARCHITECTURE.md)
2. Review [docs/guides/SIMPLIFICATION_GUIDE.md](docs/guides/SIMPLIFICATION_GUIDE.md)
3. Look at source code in `src/`

**I want to see all features:**
1. Run `cargo run --example comprehensive_app`
2. Read [docs/examples/COMPREHENSIVE_EXAMPLE_SUMMARY.md](docs/examples/COMPREHENSIVE_EXAMPLE_SUMMARY.md)
3. Check [TODO.md](TODO.md#features)

**I want API reference:**
1. See [docs/guides/API_IMPROVEMENTS.md](docs/guides/API_IMPROVEMENTS.md)
2. Check [ARCHITECTURE.md](ARCHITECTURE.md)
3. Review inline code documentation

## 📁 File Organization

```
cocoanut/
├── README.md                    # Quick start
├── ARCHITECTURE.md              # System design
├── TODO.md                      # Roadmap
├── DOCUMENTATION.md             # This file
├── docs/
│   ├── README.md               # Docs overview
│   ├── guides/                 # Feature guides
│   ├── examples/               # Example documentation
│   └── archive/                # Historical docs
├── src/                        # Source code
├── examples/                   # Working examples
└── tests/                      # Test suite
```

## 🎯 Documentation Goals

- **Concise** - Clear, focused content without unnecessary verbosity
- **Practical** - Real examples and use cases
- **Organized** - Logical structure and easy navigation
- **Maintained** - Kept up-to-date with code changes
- **Accessible** - Easy for new users to get started

## 📞 Support

- **Questions?** Check the relevant documentation section above
- **Found an issue?** See [TODO.md](TODO.md#known-issues)
- **Want to contribute?** See [ARCHITECTURE.md](ARCHITECTURE.md#contributing)

---

**Last Updated:** October 26, 2025  
**Version:** 0.1.2  
**Status:** Production Ready ✅
