//! macOS-specific features
pub mod macos_features;
pub mod macos_integration;

pub use macos_features::{NativeFeel, DesignStyle, DarkModeManager, Appearance, TouchBarManager, TouchBarItem, ContinuityManager};
pub use macos_integration::{MacOSIntegrationManager, DesignLanguageManager, AccessibilityManager, DesignLanguageComponent, AccessibleComponent, NativeWindow, NativeButton};
