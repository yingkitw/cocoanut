//! Phase 5: Advanced Features Demo
//! 
//! Demonstrates all Phase 5 advanced feature widgets:
//! - Multi-page navigation
//! - Custom components
//! - Component templates
//! - Component registry

use cocoanut::systems::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Phase 5: Advanced Features Demo\n");

    // Multi-Page Navigation
    println!("📄 Multi-Page Navigation:");

    let mut nav = Navigation::new();
    
    let home = Page::new("home", "Home").icon("🏠");
    let about = Page::new("about", "About").icon("ℹ️");
    let contact = Page::new("contact", "Contact").icon("📧");
    let settings = Page::new("settings", "Settings").icon("⚙️");

    nav.add_page(home)?;
    nav.add_page(about)?;
    nav.add_page(contact)?;
    nav.add_page(settings)?;

    println!("  ✓ Navigation created with {} pages", nav.page_count());

    nav.navigate_to("home")?;
    println!("  ✓ Current page: {:?}", nav.get_current_page());

    nav.navigate_to("about")?;
    println!("  ✓ Navigated to: {:?}", nav.get_current_page());

    nav.navigate_to("contact")?;
    println!("  ✓ Navigated to: {:?}", nav.get_current_page());

    println!("  ✓ History length: {}", nav.get_history().len());

    nav.go_back()?;
    println!("  ✓ After go_back: {:?}", nav.get_current_page());

    let visible_pages = nav.get_visible_pages();
    println!("  ✓ Visible pages: {}", visible_pages.len());

    // Sidebar Navigation
    println!("\n📍 Sidebar Navigation:");

    let mut sidebar = SidebarNav::new();
    sidebar.add_page("dashboard");
    sidebar.add_page("analytics");
    sidebar.add_page("reports");
    sidebar.add_page("users");

    println!("  ✓ Sidebar created with {} pages", sidebar.page_count());

    sidebar.select_page("dashboard")?;
    println!("  ✓ Selected: {:?}", sidebar.get_current_page());

    sidebar.select_page("analytics")?;
    println!("  ✓ Selected: {:?}", sidebar.get_current_page());

    println!("  ✓ Sidebar collapsed: {}", sidebar.is_collapsed());
    sidebar.toggle_collapse();
    println!("  ✓ After toggle: {}", sidebar.is_collapsed());

    // Custom Components
    println!("\n🎨 Custom Components:");

    let mut button = CustomComponent::new("submit_btn", "Button");
    button.add_property("label", "Submit");
    button.add_property("color", "blue");
    button.add_property("size", "large");

    println!("  ✓ CustomComponent created: {}", button.get_name());
    println!("    - Type: {}", button.get_type());
    println!("    - Properties: {}", button.property_count());
    println!("    - Label: {:?}", button.get_property("label"));

    let mut form = CustomComponent::new("contact_form", "Form");
    form.add_child("name_input");
    form.add_child("email_input");
    form.add_child("submit_btn");

    println!("  ✓ Form component created with {} children", form.child_count());

    // Component Registry
    println!("\n📦 Component Registry:");

    let mut registry = ComponentRegistry::new();
    
    let btn1 = CustomComponent::new("btn1", "Button");
    let btn2 = CustomComponent::new("btn2", "Button");
    let label1 = CustomComponent::new("label1", "Label");

    registry.register(btn1)?;
    registry.register(btn2)?;
    registry.register(label1)?;

    println!("  ✓ Registry created with {} components", registry.count());

    if let Some(comp) = registry.get("btn1") {
        println!("  ✓ Retrieved component: {}", comp.get_name());
    }

    let all_comps = registry.get_all();
    println!("  ✓ All components: {}", all_comps.len());

    println!("  ✓ Component 'btn1' exists: {}", registry.exists("btn1"));
    println!("  ✓ Component 'unknown' exists: {}", registry.exists("unknown"));

    // Component Templates
    println!("\n🏗️  Component Templates:");

    let mut primary_btn_template = ComponentTemplate::new("PrimaryButton", "Button");
    primary_btn_template.add_default_property("color", "blue");
    primary_btn_template.add_default_property("size", "medium");
    primary_btn_template.add_default_property("style", "solid");

    println!("  ✓ Template created: {}", primary_btn_template.get_name());
    println!("    - Base type: {}", primary_btn_template.get_base_type());
    println!("    - Default properties: {}", primary_btn_template.get_default_properties().len());

    let instance1 = primary_btn_template.create_instance("btn_login");
    println!("  ✓ Instance created: {}", instance1.get_name());
    println!("    - Color: {:?}", instance1.get_property("color"));
    println!("    - Size: {:?}", instance1.get_property("size"));

    let instance2 = primary_btn_template.create_instance("btn_register");
    println!("  ✓ Instance created: {}", instance2.get_name());

    // Summary
    println!("\n📈 Phase 5 Summary:");
    println!("  ✓ Multi-Page Navigation: 2 types");
    println!("    - Navigation, SidebarNav");
    println!("  ✓ Custom Components: 3 types");
    println!("    - CustomComponent, ComponentRegistry, ComponentTemplate");
    println!("  ✓ Total: 5 advanced feature widgets implemented");

    println!("\n✨ Phase 5 Complete!");
    println!("Streamlit Migration: 62 + 5 = 67 total elements implemented");

    Ok(())
}
