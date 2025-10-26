//! Core Systems
//!
//! This module contains core systems that power the framework.

pub mod events;
pub mod layout;
pub mod builder;
pub mod essential_features;
pub mod target_action;

// Phase 1: Streamlit Migration - Display Elements
pub mod display;
pub mod data_display;
pub mod feedback;

// Phase 2: Streamlit Migration - Input Widgets
pub mod input_widgets;
pub mod selection_widgets;
pub mod file_media_input;

// Phase 3: Streamlit Migration - Advanced Layouts
pub mod layout_containers;
pub mod advanced_layouts;

// Phase 4: Streamlit Migration - State & Caching
pub mod state_management;
pub mod callbacks;

// Phase 5: Streamlit Migration - Advanced Features
pub mod multi_page;
pub mod custom_components;

// Builder macros for DRY principle
pub mod builder_macros;

pub use events::*;
pub use layout::*;
pub use builder::*;
pub use essential_features::*;
pub use target_action::*;
pub use display::*;
pub use data_display::*;
pub use feedback::*;
pub use input_widgets::*;
pub use selection_widgets::*;
pub use file_media_input::*;
pub use layout_containers::*;
pub use advanced_layouts::*;
pub use state_management::*;
pub use callbacks::*;
pub use multi_page::*;
pub use custom_components::*;
