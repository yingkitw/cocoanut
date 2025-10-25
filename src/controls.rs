//! UI controls for macOS GUI applications

use crate::error::{CocoanutError, Result};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::ffi::CString;

/// A macOS button control
pub struct Button {
    ns_button: *mut Object,
    title: String,
}

/// A macOS label control
pub struct Label {
    ns_label: *mut Object,
    text: String,
}

/// A macOS text field control
pub struct TextField {
    ns_text_field: *mut Object,
    text: String,
}

impl Button {
    /// Create a new button builder for fluent API
    /// 
    /// # Returns
    /// 
    /// Returns a `ButtonBuilder` for constructing a button with a fluent interface
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use cocoanut::prelude::*;
    /// 
    /// let button = Button::builder()
    ///     .title("Click Me")
    ///     .size(100.0, 50.0)
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn builder() -> crate::builder::ButtonBuilder {
        crate::builder::ButtonBuilder::new()
    }

    /// Create a new button
    /// 
    /// # Arguments
    /// 
    /// * `title` - The button title
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Button>` containing the new button instance
    pub fn new(title: &str) -> Result<Self> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(Button {
                ns_button: std::ptr::null_mut(),
                title: title.to_string(),
            });
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};
            
            let button_class = objc::class!(NSButton);
            let ns_button: *mut Object = msg_send![button_class, alloc];
            
            let title_cstr = CString::new(title)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            
            // Create NSRect as a C struct (not an Objective-C class)
            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize { width: 100.0, height: 40.0 },
            };
            
            let ns_button: *mut Object = msg_send![
                ns_button,
                initWithFrame: frame
            ];
            
            if ns_button.is_null() {
                return Err(CocoanutError::ControlCreationFailed(
                    "Failed to create NSButton".to_string()
                ));
            }
            
            // Set button title
            let ns_string_class = objc::class!(NSString);
            let title_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String: title_cstr.as_ptr()];
            let _: () = msg_send![ns_button, setTitle: title_nsstring];
            
            // Set button style
            let _: () = msg_send![ns_button, setButtonType: 0]; // NSButtonTypeMomentaryPushIn
            
            Ok(Button {
                ns_button,
                title: title.to_string(),
            })
        }
    }
    
    /// Get the button title
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// Set the button title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            self.title = title.to_string();
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let title_cstr = CString::new(title)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            let ns_string_class = objc::class!(NSString);
            let title_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String: title_cstr.as_ptr()];
            let _: () = msg_send![self.ns_button, setTitle: title_nsstring];
            self.title = title.to_string();
            Ok(())
        }
    }
    
    /// Get the underlying NSButton pointer
    pub(crate) fn ns_button(&self) -> *mut Object {
        self.ns_button
    }
    
    /// Get the button as a view for adding to windows
    pub fn as_view(&self) -> *mut Object {
        self.ns_button
    }
}

impl Label {
    /// Create a new label builder for fluent API
    /// 
    /// # Returns
    /// 
    /// Returns a `LabelBuilder` for constructing a label with a fluent interface
    pub fn builder() -> crate::builder::LabelBuilder {
        crate::builder::LabelBuilder::new()
    }

    /// Create a new label
    /// 
    /// # Arguments
    /// 
    /// * `text` - The label text
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Label>` containing the new label instance
    pub fn new(text: &str) -> Result<Self> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(Label {
                ns_label: std::ptr::null_mut(),
                text: text.to_string(),
            });
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};
            
            let label_class = objc::class!(NSTextField);
            let ns_label: *mut Object = msg_send![label_class, alloc];
            
            let text_cstr = CString::new(text)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            
            // Create NSRect as a C struct (not an Objective-C class)
            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize { width: 200.0, height: 30.0 },
            };
            
            let ns_label: *mut Object = msg_send![
                ns_label,
                initWithFrame: frame
            ];
            
            if ns_label.is_null() {
                return Err(CocoanutError::ControlCreationFailed(
                    "Failed to create NSTextField for label".to_string()
                ));
            }
            
            // Set label properties
            let ns_string_class = objc::class!(NSString);
            let text_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String: text_cstr.as_ptr()];
            let _: () = msg_send![ns_label, setStringValue: text_nsstring];
            let _: () = msg_send![ns_label, setBezeled: false];
            let _: () = msg_send![ns_label, setDrawsBackground: false];
            let _: () = msg_send![ns_label, setEditable: false];
            let _: () = msg_send![ns_label, setSelectable: false];
            
            Ok(Label {
                ns_label,
                text: text.to_string(),
            })
        }
    }
    
    /// Get the label text
    pub fn text(&self) -> &str {
        &self.text
    }
    
    /// Set the label text
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            self.text = text.to_string();
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let text_cstr = CString::new(text)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            let ns_string_class = objc::class!(NSString);
            let text_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String: text_cstr.as_ptr()];
            let _: () = msg_send![self.ns_label, setStringValue: text_nsstring];
            self.text = text.to_string();
            Ok(())
        }
    }
    
    /// Get the underlying NSTextField pointer
    pub(crate) fn ns_label(&self) -> *mut Object {
        self.ns_label
    }
    
    /// Get the label as a view for adding to windows
    pub fn as_view(&self) -> *mut Object {
        self.ns_label
    }
}

impl TextField {
    /// Create a new text field builder for fluent API
    /// 
    /// # Returns
    /// 
    /// Returns a `TextFieldBuilder` for constructing a text field with a fluent interface
    pub fn builder() -> crate::builder::TextFieldBuilder {
        crate::builder::TextFieldBuilder::new()
    }

    /// Create a new text field
    /// 
    /// # Arguments
    /// 
    /// * `text` - The initial text content
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<TextField>` containing the new text field instance
    pub fn new(text: &str) -> Result<Self> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(TextField {
                ns_text_field: std::ptr::null_mut(),
                text: text.to_string(),
            });
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};
            
            let text_field_class = objc::class!(NSTextField);
            let ns_text_field: *mut Object = msg_send![text_field_class, alloc];
            
            let text_cstr = CString::new(text)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            
            // Create NSRect as a C struct (not an Objective-C class)
            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize { width: 200.0, height: 30.0 },
            };
            
            let ns_text_field: *mut Object = msg_send![
                ns_text_field,
                initWithFrame: frame
            ];
            
            if ns_text_field.is_null() {
                return Err(CocoanutError::ControlCreationFailed(
                    "Failed to create NSTextField".to_string()
                ));
            }
            
            // Set text field properties
            let ns_string_class = objc::class!(NSString);
            let text_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String: text_cstr.as_ptr()];
            let _: () = msg_send![ns_text_field, setStringValue: text_nsstring];
            let _: () = msg_send![ns_text_field, setBezeled: true];
            let _: () = msg_send![ns_text_field, setDrawsBackground: true];
            let _: () = msg_send![ns_text_field, setEditable: true];
            let _: () = msg_send![ns_text_field, setSelectable: true];
            
            Ok(TextField {
                ns_text_field,
                text: text.to_string(),
            })
        }
    }
    
    /// Get the text field content
    pub fn text(&self) -> &str {
        &self.text
    }
    
    /// Set the text field content
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        #[cfg(feature = "test-mock")]
        {
            self.text = text.to_string();
            return Ok(());
        }
        
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let text_cstr = CString::new(text)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            let ns_string_class = objc::class!(NSString);
            let text_nsstring: *mut Object = msg_send![ns_string_class, stringWithUTF8String: text_cstr.as_ptr()];
            let _: () = msg_send![self.ns_text_field, setStringValue: text_nsstring];
            self.text = text.to_string();
            Ok(())
        }
    }
    
    /// Get the underlying NSTextField pointer
    pub(crate) fn ns_text_field(&self) -> *mut Object {
        self.ns_text_field
    }
    
    /// Get the text field as a view for adding to windows
    pub fn as_view(&self) -> *mut Object {
        self.ns_text_field
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_button, release];
        }
    }
}

impl Drop for Label {
    fn drop(&mut self) {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_label, release];
        }
    }
}

impl Drop for TextField {
    fn drop(&mut self) {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_text_field, release];
        }
    }
}

unsafe impl Send for Button {}
unsafe impl Sync for Button {}
unsafe impl Send for Label {}
unsafe impl Sync for Label {}
unsafe impl Send for TextField {}
unsafe impl Sync for TextField {}
