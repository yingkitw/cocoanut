//! Phase 4: State & Caching Demo - GUI Window
//! 
//! Demonstrates all Phase 4 state management and caching widgets:
//! - Session state management
//! - Query parameters
//! - Data caching
//! - Resource caching
//! - Callbacks and event handling

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    app("Phase 4 Demo")
        .title("🥥 Phase 4: State & Caching (8 widgets)")
        .size(800.0, 600.0)
        .centered(true)
        .layout(Layout::default())
        // Session State
        .add(Comp::new(Kind::Label).text("💾 Session State Management (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("SessionState - Persistent state across reruns").size(700.0, 20.0))
        // Query Parameters
        .add(Comp::new(Kind::Label).text("🔗 Query Parameters (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("QueryParams - URL parameter management").size(700.0, 20.0))
        // Data Cache
        .add(Comp::new(Kind::Label).text("📦 Data Cache (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("DataCache - Generic data caching").size(700.0, 20.0))
        // Resource Cache
        .add(Comp::new(Kind::Label).text("🔄 Resource Cache (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("ResourceCache - Resource lifecycle management").size(700.0, 20.0))
        // Callbacks
        .add(Comp::new(Kind::Label).text("📡 Callbacks (3)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("ChangeCallback, ClickCallback, SubmitCallback").size(700.0, 20.0))
        // Event Handlers
        .add(Comp::new(Kind::Label).text("⚡ Event Handlers (1)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("EventDispatcher - Event routing & handling").size(700.0, 20.0))
        // Summary
        .add(Comp::new(Kind::Label).text("📈 Phase 4 Summary").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("✓ Total: 8 state & caching widgets").size(400.0, 20.0))
        .run()?;
    Ok(())
}
