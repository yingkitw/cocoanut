//! Phase 5: Custom Components
//! 
//! Implements custom component framework for extensibility.

use crate::core::error::Result;
use std::collections::HashMap;

/// Component property - key-value pair for component configuration
pub struct ComponentProperty {
    key: String,
    value: String,
}

impl ComponentProperty {
    /// Create a new property
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        ComponentProperty {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Get key
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Get value
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

/// Custom component - user-defined component
pub struct CustomComponent {
    name: String,
    component_type: String,
    properties: HashMap<String, String>,
    children: Vec<String>,
}

impl CustomComponent {
    /// Create a new custom component
    pub fn new(name: impl Into<String>, component_type: impl Into<String>) -> Self {
        CustomComponent {
            name: name.into(),
            component_type: component_type.into(),
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Add property
    pub fn add_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
    }

    /// Get property
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// Add child component
    pub fn add_child(&mut self, child_name: impl Into<String>) {
        self.children.push(child_name.into());
    }

    /// Get name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get component type
    pub fn get_type(&self) -> &str {
        &self.component_type
    }

    /// Get all properties
    pub fn get_properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    /// Get children
    pub fn get_children(&self) -> &[String] {
        &self.children
    }

    /// Get property count
    pub fn property_count(&self) -> usize {
        self.properties.len()
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

/// Component registry - registry for custom components
pub struct ComponentRegistry {
    components: HashMap<String, CustomComponent>,
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        ComponentRegistry {
            components: HashMap::new(),
        }
    }

    /// Register a component
    pub fn register(&mut self, component: CustomComponent) -> Result<()> {
        let name = component.get_name().to_string();
        self.components.insert(name, component);
        Ok(())
    }

    /// Get a component
    pub fn get(&self, name: &str) -> Option<&CustomComponent> {
        self.components.get(name)
    }

    /// Get mutable component
    pub fn get_mut(&mut self, name: &str) -> Option<&mut CustomComponent> {
        self.components.get_mut(name)
    }

    /// Unregister a component
    pub fn unregister(&mut self, name: &str) -> Option<CustomComponent> {
        self.components.remove(name)
    }

    /// Get all components
    pub fn get_all(&self) -> Vec<&CustomComponent> {
        self.components.values().collect()
    }

    /// Get component count
    pub fn count(&self) -> usize {
        self.components.len()
    }

    /// Clear all components
    pub fn clear(&mut self) {
        self.components.clear();
    }

    /// Check if component exists
    pub fn exists(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Component template - reusable component template
pub struct ComponentTemplate {
    name: String,
    base_type: String,
    default_properties: HashMap<String, String>,
}

impl ComponentTemplate {
    /// Create a new component template
    pub fn new(name: impl Into<String>, base_type: impl Into<String>) -> Self {
        ComponentTemplate {
            name: name.into(),
            base_type: base_type.into(),
            default_properties: HashMap::new(),
        }
    }

    /// Add default property
    pub fn add_default_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.default_properties.insert(key.into(), value.into());
    }

    /// Create instance from template
    pub fn create_instance(&self, instance_name: impl Into<String>) -> CustomComponent {
        let mut component = CustomComponent::new(instance_name, self.base_type.clone());
        
        for (key, value) in &self.default_properties {
            component.add_property(key.clone(), value.clone());
        }

        component
    }

    /// Get name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get base type
    pub fn get_base_type(&self) -> &str {
        &self.base_type
    }

    /// Get default properties
    pub fn get_default_properties(&self) -> &HashMap<String, String> {
        &self.default_properties
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_property() {
        let prop = ComponentProperty::new("color", "red");
        assert_eq!(prop.get_key(), "color");
        assert_eq!(prop.get_value(), "red");
    }

    #[test]
    fn test_custom_component() {
        let mut comp = CustomComponent::new("button1", "Button");
        comp.add_property("label", "Click me");
        comp.add_property("color", "blue");

        assert_eq!(comp.get_name(), "button1");
        assert_eq!(comp.get_type(), "Button");
        assert_eq!(comp.get_property("label"), Some("Click me"));
        assert_eq!(comp.property_count(), 2);
    }

    #[test]
    fn test_component_children() {
        let mut comp = CustomComponent::new("container", "Container");
        comp.add_child("button1");
        comp.add_child("label1");

        assert_eq!(comp.child_count(), 2);
        assert_eq!(comp.get_children().len(), 2);
    }

    #[test]
    fn test_component_registry() {
        let mut registry = ComponentRegistry::new();
        let comp = CustomComponent::new("btn1", "Button");

        registry.register(comp).unwrap();
        assert!(registry.exists("btn1"));
        assert_eq!(registry.count(), 1);

        let retrieved = registry.get("btn1");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_component_template() {
        let mut template = ComponentTemplate::new("PrimaryButton", "Button");
        template.add_default_property("color", "blue");
        template.add_default_property("size", "large");

        let instance = template.create_instance("btn1");
        assert_eq!(instance.get_property("color"), Some("blue"));
        assert_eq!(instance.get_property("size"), Some("large"));
    }

    #[test]
    fn test_registry_unregister() {
        let mut registry = ComponentRegistry::new();
        let comp = CustomComponent::new("btn1", "Button");

        registry.register(comp).unwrap();
        assert_eq!(registry.count(), 1);

        registry.unregister("btn1");
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_registry_get_all() {
        let mut registry = ComponentRegistry::new();
        registry.register(CustomComponent::new("btn1", "Button")).unwrap();
        registry.register(CustomComponent::new("label1", "Label")).unwrap();
        registry.register(CustomComponent::new("input1", "Input")).unwrap();

        let all = registry.get_all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_registry_clear() {
        let mut registry = ComponentRegistry::new();
        registry.register(CustomComponent::new("btn1", "Button")).unwrap();
        registry.register(CustomComponent::new("label1", "Label")).unwrap();

        assert_eq!(registry.count(), 2);
        registry.clear();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_registry_get_mut() {
        let mut registry = ComponentRegistry::new();
        let mut comp = CustomComponent::new("btn1", "Button");
        comp.add_property("color", "blue");
        registry.register(comp).unwrap();

        if let Some(c) = registry.get_mut("btn1") {
            c.add_property("size", "large");
        }

        let retrieved = registry.get("btn1").unwrap();
        assert_eq!(retrieved.get_property("size"), Some("large"));
    }

    #[test]
    fn test_component_property_getters() {
        let prop = ComponentProperty::new("enabled", "true");
        assert_eq!(prop.get_key(), "enabled");
        assert_eq!(prop.get_value(), "true");
    }

    #[test]
    fn test_custom_component_multiple_properties() {
        let mut comp = CustomComponent::new("form", "Form");
        comp.add_property("method", "POST");
        comp.add_property("action", "/submit");
        comp.add_property("enctype", "multipart/form-data");

        assert_eq!(comp.property_count(), 3);
        assert_eq!(comp.get_property("method"), Some("POST"));
        assert_eq!(comp.get_property("action"), Some("/submit"));
        assert_eq!(comp.get_property("enctype"), Some("multipart/form-data"));
    }

    #[test]
    fn test_custom_component_multiple_children() {
        let mut comp = CustomComponent::new("container", "Container");
        comp.add_child("child1");
        comp.add_child("child2");
        comp.add_child("child3");
        comp.add_child("child4");

        assert_eq!(comp.child_count(), 4);
        assert_eq!(comp.get_children().len(), 4);
    }

    #[test]
    fn test_template_multiple_instances() {
        let mut template = ComponentTemplate::new("Card", "Container");
        template.add_default_property("border", "1px solid gray");
        template.add_default_property("padding", "16px");
        template.add_default_property("radius", "8px");

        let inst1 = template.create_instance("card1");
        let inst2 = template.create_instance("card2");
        let inst3 = template.create_instance("card3");

        assert_eq!(inst1.get_name(), "card1");
        assert_eq!(inst2.get_name(), "card2");
        assert_eq!(inst3.get_name(), "card3");

        assert_eq!(inst1.get_property("border"), Some("1px solid gray"));
        assert_eq!(inst2.get_property("padding"), Some("16px"));
        assert_eq!(inst3.get_property("radius"), Some("8px"));
    }

    #[test]
    fn test_template_override_properties() {
        let mut template = ComponentTemplate::new("Button", "Button");
        template.add_default_property("color", "blue");
        template.add_default_property("size", "medium");

        let mut instance = template.create_instance("btn");
        instance.add_property("color", "red"); // Override

        assert_eq!(instance.get_property("color"), Some("red"));
        assert_eq!(instance.get_property("size"), Some("medium"));
    }

    #[test]
    fn test_registry_multiple_operations() {
        let mut registry = ComponentRegistry::new();

        // Add components
        for i in 1..=5 {
            let name = format!("comp{}", i);
            registry.register(CustomComponent::new(&name, "Component")).unwrap();
        }

        assert_eq!(registry.count(), 5);

        // Check existence
        assert!(registry.exists("comp1"));
        assert!(registry.exists("comp5"));
        assert!(!registry.exists("comp6"));

        // Get all
        let all = registry.get_all();
        assert_eq!(all.len(), 5);

        // Unregister one
        registry.unregister("comp3");
        assert_eq!(registry.count(), 4);
        assert!(!registry.exists("comp3"));
    }

    #[test]
    fn test_component_with_nested_structure() {
        let mut parent = CustomComponent::new("form", "Form");
        parent.add_property("id", "contact_form");
        parent.add_child("fieldset1");
        parent.add_child("fieldset2");

        let mut fieldset1 = CustomComponent::new("fieldset1", "Fieldset");
        fieldset1.add_property("legend", "Personal Info");
        fieldset1.add_child("input_name");
        fieldset1.add_child("input_email");

        assert_eq!(parent.child_count(), 2);
        assert_eq!(fieldset1.child_count(), 2);
        assert_eq!(fieldset1.property_count(), 1);
    }

    #[test]
    fn test_template_with_many_defaults() {
        let mut template = ComponentTemplate::new("ComplexButton", "Button");
        
        for i in 1..=10 {
            let key = format!("prop{}", i);
            let value = format!("value{}", i);
            template.add_default_property(&key, &value);
        }

        let instance = template.create_instance("complex_btn");
        assert_eq!(instance.property_count(), 10);

        for i in 1..=10 {
            let key = format!("prop{}", i);
            let expected = format!("value{}", i);
            assert_eq!(instance.get_property(&key), Some(expected.as_str()));
        }
    }
}
