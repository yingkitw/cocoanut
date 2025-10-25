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

pub mod application;
pub mod window;
pub mod menu;
pub mod controls;
pub mod events;
pub mod drawing;
pub mod error;
pub mod utils;
pub mod async_ui;
pub mod streaming;
pub mod zero_cost;
pub mod macos_integration;
pub mod builder;
pub mod layout;
pub mod styling;
pub mod checkbox;
pub mod radio;
pub mod slider;
pub mod advanced_controls;
pub mod containers;
pub mod data_display;
pub mod phase3_features;
pub mod simple_app;

/// Re-exports for convenient usage
pub mod prelude {
    pub use crate::application::Application;
    pub use crate::window::Window;
    pub use crate::menu::{Menu, MenuItem};
    pub use crate::controls::{Button, Label, TextField};
    pub use crate::events::{Event, EventHandler};
    pub use crate::drawing::{Color, Point, Size, Rect};
    pub use crate::error::{CocoanutError, Result};
    
    // Modern Rust patterns
    pub use crate::async_ui::{AsyncUI, AsyncUIExecutor, AsyncUIContext, AsyncWindow, AsyncButton};
    pub use crate::streaming::{ReactiveUI, UIEvent, UIEventStream, ReactiveButton, ReactiveTextField, ReactiveWindow, ReactiveUIManager};
    pub use crate::zero_cost::{ZeroCostObject, ZeroCostString, ZeroCostArray, ZeroCostPoint, ZeroCostSize, ZeroCostRect, ZeroCostColor};
    
    // macOS integration
    pub use crate::macos_integration::{
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
    
    // Simple high-level API
    pub use crate::simple_app::{SimpleApp, app};
}

pub use error::{CocoanutError, Result};
