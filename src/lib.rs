//! # Cocoanut
//! 
//! A Rust wrapper for Cocoa to develop macOS-specific GUI applications.
//! 
//! This crate provides a safe, idiomatic Rust interface to macOS's Cocoa framework,
//! enabling developers to create native macOS applications with ease.
//! 
//! ## Features
//! 
//! - **Window Management**: Create and manage native macOS windows
//! - **Menu System**: Build application menus and context menus
//! - **Controls**: Native macOS UI controls and widgets
//! - **Event Handling**: Comprehensive event system for user interactions
//! - **Drawing**: Core Graphics integration for custom drawing
//! - **Thread Safety**: Safe cross-thread operations
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use cocoanut::prelude::*;
//! 
//! fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//!     let app = Application::new("My App")?;
//!     let window = Window::new("My Window", 800.0, 600.0)?;
//!     
//!     app.run(window)?;
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_op_in_unsafe_fn)]

// Organized module structure
pub mod core;
pub mod components;
pub mod systems;
pub mod features;
pub mod utils;

// High-level API
pub mod window;
pub mod menu;
pub mod simple_app;
pub mod application;
pub mod async_ui;
pub mod streaming;

// Legacy exports for backward compatibility
pub mod controls {
    pub use crate::components::basic::*;
}
pub mod controls_v2 {
    pub use crate::components::basic::*;
}
pub mod checkbox {
    pub use crate::components::advanced::checkbox::*;
}
pub mod radio {
    pub use crate::components::advanced::radio::*;
}
pub mod slider {
    pub use crate::components::advanced::slider::*;
}
pub mod advanced_controls {
    pub use crate::components::advanced::*;
}
pub mod containers {
    pub use crate::components::containers::*;
}
pub mod data_display {
    pub use crate::components::data_display::*;
}
pub mod events {
    pub use crate::systems::events::*;
}
pub mod layout {
    pub use crate::systems::layout::*;
}
pub mod builder {
    pub use crate::systems::builder::*;
}
pub mod essential_features {
    pub use crate::systems::essential_features::*;
}
pub mod drawing {
    pub use crate::features::drawing::*;
}
pub mod styling {
    pub use crate::features::styling::*;
}
pub mod zero_cost {
    pub use crate::features::zero_cost::*;
}
pub mod macos_features {
    pub use crate::features::macos::*;
}
pub mod macos_integration {
    pub use crate::features::macos::*;
}
pub mod phase3_features {
    pub use crate::features::phase3_features::*;
}
pub mod advanced_views {
    pub use crate::features::advanced_views::*;
}
pub mod core_fixes {
    pub use crate::utils::*;
}

/// Re-exports for convenient usage
pub mod prelude {
    pub use crate::application::Application;
    pub use crate::window::Window;
    pub use crate::menu::{Menu, MenuItem};
    pub use crate::components::basic::{Button, Label, TextField};
    pub use crate::systems::events::{Event, EventHandler};
    pub use crate::features::drawing::{Color, Point, Size, Rect};
    pub use crate::core::error::{CocoanutError, Result};
    
    // Modern Rust patterns
    pub use crate::async_ui::{AsyncUI, AsyncUIExecutor, AsyncUIContext, AsyncWindow, AsyncButton};
    pub use crate::streaming::{ReactiveUI, UIEvent, UIEventStream, ReactiveButton, ReactiveTextField, ReactiveWindow, ReactiveUIManager};
    pub use crate::features::zero_cost::{ZeroCostObject, ZeroCostString, ZeroCostArray, ZeroCostPoint, ZeroCostSize, ZeroCostRect, ZeroCostColor};
    
    // macOS integration
    pub use crate::features::macos::{
        MacOSIntegrationManager, DesignLanguageManager, AccessibilityManager, DarkModeManager, TouchBarManager,
        DesignStyle, Appearance, TouchBarItem, DesignLanguageComponent, AccessibleComponent,
        NativeWindow, NativeButton
    };
    
    // Builders for simplified API
    pub use crate::builder::{ButtonBuilder, LabelBuilder, TextFieldBuilder, WindowBuilder};
    
    // Layout system
    pub use crate::layout::{VStack, HStack, Spacer, Spacing, Alignment};
    
    // Styling system
    pub use crate::styling::{
        CarbonColor, TypographyScale, SpacingScale, CornerRadiusScale, ComponentStyle,
    };
    
    // Phase 2: Basic Controls
    pub use crate::checkbox::{Checkbox, CheckboxBuilder};
    pub use crate::radio::{RadioButton, RadioButtonBuilder};
    pub use crate::slider::{Slider, SliderBuilder};
    pub use crate::advanced_controls::{
        SegmentedControl, SegmentedControlBuilder,
        Stepper, StepperBuilder,
        Switch, SwitchBuilder,
    };
    
    // Phase 2: Container Views
    pub use crate::containers::{
        ScrollView, ScrollViewBuilder,
        TabView, TabViewBuilder,
        SplitView, SplitViewBuilder, SplitOrientation,
        GroupBox, GroupBoxBuilder,
    };
    
    // Phase 3: Data Display
    pub use crate::data_display::{
        TableView, TableViewBuilder,
        OutlineView, OutlineViewBuilder, OutlineItem,
        CollectionView, CollectionViewBuilder,
    };
    
    // Phase 3: macOS Features
    pub use crate::phase3_features::{
        GridView, GridViewBuilder,
        TouchBar as TouchBarFeature, TouchBarBuilder as TouchBarFeatureBuilder, TouchBarItem as TouchBarFeatureItem,
        AccessibilityOptions, AccessibilityBuilder,
        DarkModeManager as DarkModeFeature, Appearance as AppearanceMode,
        DragDropManager,
        AdvancedStyling, AdvancedStylingBuilder,
    };
    
    // macOS Integration Features (new)
    pub use crate::macos_features::{
        NativeFeel,
        DesignStyle as NativeDesignStyle,
        DarkModeManager as DarkMode,
        Appearance as SystemAppearance,
        TouchBarManager as MacTouchBar,
        TouchBarItem as MacTouchBarItem,
        ContinuityManager,
    };
    
    // Core fixes and improvements
    pub use crate::utils::{
        ThreadSafeView, MemoryManager, ErrorContext, ApiConsistency, CompilationTracker,
    };
    
    // Essential features
    pub use crate::essential_features::{
        EventSystem, EventCallback,
        LayoutConstraint, AutoLayout,
        Animation, TimingFunction, CustomView,
        DataBinding,
    };
    
    // Advanced views
    pub use crate::advanced_views::{
        TableView as TableViewComponent, TableViewDataSource,
        CollectionView as CollectionViewComponent, CollectionViewItem,
        SplitView as SplitViewComponent, SplitOrientation as SplitViewOrientation,
        TabView as TabViewComponent, TabViewItem,
        WebView as WebViewComponent,
    };
    
    // Simple high-level API
    pub use crate::simple_app::{SimpleApp, app, ComponentConfig, ComponentType};
}

pub use core::error::{CocoanutError, Result};
