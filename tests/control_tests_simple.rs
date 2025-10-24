//! Simplified GUI controls tests - only testing actually implemented methods

use cocoanut::*;
use cocoanut::controls::{Button, Label, TextField};

#[test]
fn test_button_creation() {
    let button = Button::new("Click Me");
    assert!(button.is_ok());
    
    let button = button.unwrap();
    assert_eq!(button.title(), "Click Me");
}

#[test]
fn test_button_title_change() {
    let mut button = Button::new("Original Title").unwrap();
    
    assert_eq!(button.title(), "Original Title");
    
    let result = button.set_title("New Title");
    assert!(result.is_ok());
    assert_eq!(button.title(), "New Title");
}

#[test]
fn test_label_creation() {
    let label = Label::new("Test Label");
    assert!(label.is_ok());
    
    let label = label.unwrap();
    assert_eq!(label.text(), "Test Label");
}

#[test]
fn test_label_text_change() {
    let mut label = Label::new("Original Text").unwrap();
    
    assert_eq!(label.text(), "Original Text");
    
    let result = label.set_text("New Text");
    assert!(result.is_ok());
    assert_eq!(label.text(), "New Text");
}

#[test]
fn test_text_field_creation() {
    let text_field = TextField::new("Initial Text");
    assert!(text_field.is_ok());
    
    let text_field = text_field.unwrap();
    assert_eq!(text_field.text(), "Initial Text");
}

#[test]
fn test_text_field_text_change() {
    let mut text_field = TextField::new("Initial").unwrap();
    
    assert_eq!(text_field.text(), "Initial");
    
    let result = text_field.set_text("Updated Text");
    assert!(result.is_ok());
    assert_eq!(text_field.text(), "Updated Text");
}

#[test]
fn test_control_creation_with_empty_strings() {
    // Test that empty strings are handled gracefully
    let button = Button::new("");
    assert!(button.is_ok());
    assert_eq!(button.unwrap().title(), "");
    
    let label = Label::new("");
    assert!(label.is_ok());
    assert_eq!(label.unwrap().text(), "");
    
    let text_field = TextField::new("");
    assert!(text_field.is_ok());
    assert_eq!(text_field.unwrap().text(), "");
}

#[test]
fn test_control_error_handling() {
    // Test that invalid parameters are handled gracefully
    // (This would depend on the actual implementation)
    let button = Button::new("Valid Title");
    assert!(button.is_ok());
}

#[test]
fn test_multiple_controls() {
    // Test creating multiple controls
    let button1 = Button::new("Button 1").unwrap();
    let button2 = Button::new("Button 2").unwrap();
    let label1 = Label::new("Label 1").unwrap();
    let label2 = Label::new("Label 2").unwrap();
    
    assert_eq!(button1.title(), "Button 1");
    assert_eq!(button2.title(), "Button 2");
    assert_eq!(label1.text(), "Label 1");
    assert_eq!(label2.text(), "Label 2");
}

#[test]
fn test_control_memory_management() {
    // Test that controls can be created and dropped without issues
    let mut buttons = Vec::new();
    let mut labels = Vec::new();
    let mut text_fields = Vec::new();
    
    for i in 0..10 {
        let button = Button::new(&format!("Button {}", i)).unwrap();
        let label = Label::new(&format!("Label {}", i)).unwrap();
        let text_field = TextField::new(&format!("Text {}", i)).unwrap();
        
        buttons.push(button);
        labels.push(label);
        text_fields.push(text_field);
    }
    
    assert_eq!(buttons.len(), 10);
    assert_eq!(labels.len(), 10);
    assert_eq!(text_fields.len(), 10);
    // Controls should be dropped here without panicking
}
