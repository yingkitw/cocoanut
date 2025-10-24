//! Event handling for macOS GUI applications

use crate::error::{CocoanutError, Result};
use std::ffi::CString;

/// Event types that can be handled
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Window events
    WindowClose,
    WindowResize,
    WindowMove,
    
    /// Mouse events
    MouseDown,
    MouseUp,
    MouseMove,
    MouseEnter,
    MouseExit,
    
    /// Keyboard events
    KeyDown,
    KeyUp,
    
    /// Button events
    ButtonClick,
    
    /// Text field events
    TextChanged,
    
    /// Menu events
    MenuAction(String),
    
    /// Application events
    ApplicationWillTerminate,
    ApplicationDidFinishLaunching,
}

/// Event handler trait for processing events
pub trait EventHandler {
    /// Handle an event
    /// 
    /// # Arguments
    /// 
    /// * `event` - The event to handle
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<()>` indicating success or failure
    fn handle_event(&mut self, event: Event) -> Result<()>;
}

/// Event manager for coordinating event handling
pub struct EventManager {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventManager {
    /// Create a new event manager
    pub fn new() -> Self {
        EventManager {
            handlers: Vec::new(),
        }
    }
    
    /// Add an event handler
    /// 
    /// # Arguments
    /// 
    /// * `handler` - The event handler to add
    pub fn add_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }
    
    /// Process an event through all registered handlers
    /// 
    /// # Arguments
    /// 
    /// * `event` - The event to process
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<()>` indicating success or failure
    pub fn process_event(&mut self, event: Event) -> Result<()> {
        for handler in &mut self.handlers {
            handler.handle_event(event.clone())?;
        }
        Ok(())
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple event handler that prints events
pub struct PrintEventHandler;

impl EventHandler for PrintEventHandler {
    fn handle_event(&mut self, event: Event) -> Result<()> {
        println!("Event received: {:?}", event);
        Ok(())
    }
}

/// Event handler that can be used to store events for testing
pub struct TestEventHandler {
    pub events: Vec<Event>,
}

impl TestEventHandler {
    /// Create a new test event handler
    pub fn new() -> Self {
        TestEventHandler {
            events: Vec::new(),
        }
    }
    
    /// Get all received events
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    
    /// Clear all stored events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl EventHandler for TestEventHandler {
    fn handle_event(&mut self, event: Event) -> Result<()> {
        self.events.push(event);
        Ok(())
    }
}

impl Default for TestEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for event handling
pub mod utils {
    use super::*;
    
    /// Convert an Objective-C selector to an event
    /// 
    /// # Arguments
    /// 
    /// * `selector` - The Objective-C selector name
    /// 
    /// # Returns
    /// 
    /// Returns an `Option<Event>` if the selector can be converted
    pub fn selector_to_event(selector: &str) -> Option<Event> {
        match selector {
            "windowShouldClose:" => Some(Event::WindowClose),
            "windowDidResize:" => Some(Event::WindowResize),
            "windowDidMove:" => Some(Event::WindowMove),
            "mouseDown:" => Some(Event::MouseDown),
            "mouseUp:" => Some(Event::MouseUp),
            "mouseMoved:" => Some(Event::MouseMove),
            "mouseEntered:" => Some(Event::MouseEnter),
            "mouseExited:" => Some(Event::MouseExit),
            "keyDown:" => Some(Event::KeyDown),
            "keyUp:" => Some(Event::KeyUp),
            "buttonClicked:" => Some(Event::ButtonClick),
            "textDidChange:" => Some(Event::TextChanged),
            "applicationWillTerminate:" => Some(Event::ApplicationWillTerminate),
            "applicationDidFinishLaunching:" => Some(Event::ApplicationDidFinishLaunching),
            _ => {
                if selector.starts_with("menuAction") {
                    Some(Event::MenuAction(selector.to_string()))
                } else {
                    None
                }
            }
        }
    }
    
    /// Create a C string from a Rust string for Objective-C calls
    /// 
    /// # Arguments
    /// 
    /// * `s` - The Rust string
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<CString>` containing the C string
    pub fn create_c_string(s: &str) -> Result<CString> {
        CString::new(s).map_err(|e| CocoanutError::InvalidParameter(e.to_string()))
    }
}
