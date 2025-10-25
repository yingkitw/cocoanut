//! Phase 4: State & Caching Demo
//! 
//! Demonstrates all Phase 4 state management and caching widgets:
//! - Session state management
//! - Query parameters
//! - Data caching
//! - Resource caching
//! - Callbacks and event handlers

use cocoanut::systems::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Phase 4: State & Caching Demo\n");

    // Session State
    println!("💾 Session State Management:");

    let session = SessionState::new();
    session.set("user_id", "12345")?;
    session.set("username", "john_doe")?;
    session.set("theme", "dark")?;

    println!("  ✓ SessionState created");
    println!("    - user_id: {:?}", session.get("user_id")?);
    println!("    - username: {:?}", session.get("username")?);
    println!("    - theme: {:?}", session.get("theme")?);

    let keys = session.keys()?;
    println!("  ✓ Keys in session: {:?}", keys);

    let contains = session.contains_key("user_id")?;
    println!("  ✓ Contains 'user_id': {}", contains);

    // Query Parameters
    println!("\n🔗 Query Parameters:");

    let params = QueryParams::from_string("page=1&limit=10&sort=name")?;
    println!("  ✓ QueryParams parsed from string");
    println!("    - page: {:?}", params.get("page"));
    println!("    - limit: {:?}", params.get("limit"));
    println!("    - sort: {:?}", params.get("sort"));

    let query_string = params.to_string();
    println!("  ✓ Query string: {}", query_string);

    // Data Cache
    println!("\n📦 Data Cache:");

    let cache: DataCache<String> = DataCache::new();
    cache.set("user_profile", "John Doe, 30, Engineer".to_string(), None)?;
    cache.set("settings", "theme=dark, language=en".to_string(), Some(3600))?;

    println!("  ✓ DataCache created");
    println!("    - user_profile: {:?}", cache.get("user_profile")?);
    println!("    - settings: {:?}", cache.get("settings")?);

    let size = cache.size()?;
    println!("  ✓ Cache size: {} entries", size);

    // Resource Cache
    println!("\n🖼️  Resource Cache:");

    let resource_cache = ResourceCache::new();
    let image_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
    resource_cache.set("logo.jpg", image_data.clone(), None)?;

    println!("  ✓ ResourceCache created");
    println!("    - logo.jpg: {:?} bytes", resource_cache.get("logo.jpg")?.map(|d| d.len()));

    let bytes = resource_cache.size_bytes()?;
    println!("  ✓ Total cache size: {} bytes", bytes);

    // Change Callback
    println!("\n🔔 Callbacks:");

    let change_cb: ChangeCallback<i32> = ChangeCallback::new();
    change_cb.on_change(|value| {
        println!("    → Value changed to: {}", value);
    })?;
    change_cb.on_change(|value| {
        println!("    → New value received: {}", value);
    })?;

    println!("  ✓ ChangeCallback registered (count: {})", change_cb.count()?);
    change_cb.trigger(42)?;

    // Click Callback
    let click_cb = ClickCallback::new();
    click_cb.on_click(|| {
        println!("    → Button clicked!");
    })?;

    println!("  ✓ ClickCallback registered (count: {})", click_cb.count()?);
    click_cb.trigger()?;

    // Submit Callback
    let submit_cb = SubmitCallback::new();
    submit_cb.on_submit(|form_data| {
        println!("    → Form submitted with data: {}", form_data);
    })?;

    println!("  ✓ SubmitCallback registered (count: {})", submit_cb.count()?);
    submit_cb.trigger("name=John&email=john@example.com".to_string())?;

    // Event Dispatcher
    println!("\n⚡ Event Dispatchers:");

    let dispatcher = EventDispatcher::new();
    dispatcher.on("load", || {
        println!("    → Page loaded");
    })?;
    dispatcher.on("click", || {
        println!("    → Element clicked");
    })?;
    dispatcher.on("click", || {
        println!("    → Click event (second handler)");
    })?;

    println!("  ✓ EventDispatcher created");
    println!("    - 'load' handlers: {}", dispatcher.count("load")?);
    println!("    - 'click' handlers: {}", dispatcher.count("click")?);

    println!("  ✓ Triggering 'load' event:");
    dispatcher.trigger("load")?;

    println!("  ✓ Triggering 'click' event:");
    dispatcher.trigger("click")?;

    // Summary
    println!("\n📈 Phase 4 Summary:");
    println!("  ✓ Session State Management: 1 type");
    println!("  ✓ Query Parameters: 1 type");
    println!("  ✓ Data Cache: 1 type");
    println!("  ✓ Resource Cache: 1 type");
    println!("  ✓ Callbacks: 3 types (Change, Click, Submit)");
    println!("  ✓ Event Handlers: 1 type");
    println!("  ✓ Total: 8 state & caching widgets implemented");

    println!("\n✨ Phase 4 Complete!");
    println!("Next: Phase 5 - Advanced Features (v0.7.0+)");

    Ok(())
}
