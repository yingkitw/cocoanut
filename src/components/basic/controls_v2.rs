//! Refactored UI controls using traits and capability-facing design
//!
//! This module provides concise, trait-based UI controls that are test-friendly and modular.

use crate::core::error::{CocoanutError, Result};
use crate::core::traits::{Drawable, Textual, Positionable};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::ffi::CString;

/// Macro to reduce boilerplate for NSString creation
macro_rules! ns_string {
    ($text:expr) => {{
        let cstr = CString::new($text)
            .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
        let ns_string_class = objc::class!(NSString);
        let ns_str: *mut Object = msg_send![ns_string_class, stringWithUTF8String: cstr.as_ptr()];
        ns_str
    }};
}

/// Base control structure
struct ControlBase {
    ns_view: *mut Object,
    id: String,
}

impl ControlBase {
    fn new(ns_view: *mut Object, id: &str) -> Self {
        Self {
            ns_view,
            id: id.to_string(),
        }
    }
}

impl Drawable for ControlBase {
    fn as_view(&self) -> *mut Object {
        self.ns_view
    }

    fn set_visible(&self, visible: bool) -> Result<()> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let _: () = msg_send![self.ns_view, setHidden: !visible];
        }
        Ok(())
    }

    fn is_visible(&self) -> bool {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let hidden: bool = msg_send![self.ns_view, isHidden];
            !hidden
        }
        #[cfg(feature = "test-mock")]
        true
    }
}

impl Positionable for ControlBase {
    fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};
            let frame = NSRect {
                origin: NSPoint { x, y },
                size: NSSize { width, height },
            };
            let _: () = msg_send![self.ns_view, setFrame: frame];
        }
        Ok(())
    }

    fn frame(&self) -> (f64, f64, f64, f64) {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::NSRect;
            let frame: NSRect = msg_send![self.ns_view, frame];
            (frame.origin.x, frame.origin.y, frame.size.width, frame.size.height)
        }
        #[cfg(feature = "test-mock")]
        (0.0, 0.0, 100.0, 40.0)
    }
}

/// Button control
pub struct Button {
    base: ControlBase,
    title: String,
}

impl Button {
    /// Create a new button
    pub fn new(title: &str) -> Result<Self> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(Button {
                base: ControlBase::new(std::ptr::null_mut(), "button"),
                title: title.to_string(),
            });
        }

        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};

            let button_class = objc::class!(NSButton);
            let ns_button: *mut Object = msg_send![button_class, alloc];

            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize { width: 100.0, height: 40.0 },
            };

            let ns_button: *mut Object = msg_send![ns_button, initWithFrame: frame];
            if ns_button.is_null() {
                return Err(CocoanutError::ControlCreationFailed("Button creation failed".into()));
            }

            let title_ns = ns_string!(title);
            let _: () = msg_send![ns_button, setTitle: title_ns];
            let _: () = msg_send![ns_button, setButtonType: 0];

            Ok(Button {
                base: ControlBase::new(ns_button, "button"),
                title: title.to_string(),
            })
        }
    }

    /// Create a button builder
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
}

impl Drawable for Button {
    fn as_view(&self) -> *mut Object {
        self.base.as_view()
    }

    fn set_visible(&self, visible: bool) -> Result<()> {
        self.base.set_visible(visible)
    }

    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }
}

impl Positionable for Button {
    fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
        self.base.set_frame(x, y, width, height)
    }

    fn frame(&self) -> (f64, f64, f64, f64) {
        self.base.frame()
    }
}

impl Textual for Button {
    fn text(&self) -> &str {
        &self.title
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let title_ns = ns_string!(text);
            let _: () = msg_send![self.base.ns_view, setTitle: title_ns];
        }
        self.title = text.to_string();
        Ok(())
    }
}

/// Button builder
#[derive(Default)]
pub struct ButtonBuilder {
    title: String,
    width: Option<f64>,
    height: Option<f64>,
}

impl ButtonBuilder {
    /// Set button title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set button size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Build the button
    pub fn build(self) -> Result<Button> {
        let button = Button::new(&self.title)?;
        if let (Some(w), Some(h)) = (self.width, self.height) {
            button.set_frame(0.0, 0.0, w, h)?;
        }
        Ok(button)
    }
}

/// Label control
pub struct Label {
    base: ControlBase,
    text: String,
}

impl Label {
    /// Create a new label
    pub fn new(text: &str) -> Result<Self> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(Label {
                base: ControlBase::new(std::ptr::null_mut(), "label"),
                text: text.to_string(),
            });
        }

        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};

            let label_class = objc::class!(NSTextField);
            let ns_label: *mut Object = msg_send![label_class, alloc];

            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize { width: 200.0, height: 30.0 },
            };

            let ns_label: *mut Object = msg_send![ns_label, initWithFrame: frame];
            if ns_label.is_null() {
                return Err(CocoanutError::ControlCreationFailed("Label creation failed".into()));
            }

            let text_ns = ns_string!(text);
            let _: () = msg_send![ns_label, setStringValue: text_ns];
            let _: () = msg_send![ns_label, setEditable: false];
            let _: () = msg_send![ns_label, setBezeled: false];
            let _: () = msg_send![ns_label, setDrawsBackground: false];

            Ok(Label {
                base: ControlBase::new(ns_label, "label"),
                text: text.to_string(),
            })
        }
    }

    /// Create a label builder
    pub fn builder() -> LabelBuilder {
        LabelBuilder::default()
    }
}

impl Drawable for Label {
    fn as_view(&self) -> *mut Object {
        self.base.as_view()
    }

    fn set_visible(&self, visible: bool) -> Result<()> {
        self.base.set_visible(visible)
    }

    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }
}

impl Positionable for Label {
    fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
        self.base.set_frame(x, y, width, height)
    }

    fn frame(&self) -> (f64, f64, f64, f64) {
        self.base.frame()
    }
}

impl Textual for Label {
    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let text_ns = ns_string!(text);
            let _: () = msg_send![self.base.ns_view, setStringValue: text_ns];
        }
        self.text = text.to_string();
        Ok(())
    }
}

/// Label builder
#[derive(Default)]
pub struct LabelBuilder {
    text: String,
    width: Option<f64>,
    height: Option<f64>,
}

impl LabelBuilder {
    /// Set label text
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Set label size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Build the label
    pub fn build(self) -> Result<Label> {
        let label = Label::new(&self.text)?;
        if let (Some(w), Some(h)) = (self.width, self.height) {
            label.set_frame(0.0, 0.0, w, h)?;
        }
        Ok(label)
    }
}

/// TextField control
pub struct TextField {
    base: ControlBase,
    text: String,
}

impl TextField {
    /// Create a new text field
    pub fn new(placeholder: &str) -> Result<Self> {
        #[cfg(feature = "test-mock")]
        {
            return Ok(TextField {
                base: ControlBase::new(std::ptr::null_mut(), "textfield"),
                text: placeholder.to_string(),
            });
        }

        #[cfg(not(feature = "test-mock"))]
        unsafe {
            use cocoa::foundation::{NSRect, NSPoint, NSSize};

            let tf_class = objc::class!(NSTextField);
            let ns_tf: *mut Object = msg_send![tf_class, alloc];

            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize { width: 200.0, height: 30.0 },
            };

            let ns_tf: *mut Object = msg_send![ns_tf, initWithFrame: frame];
            if ns_tf.is_null() {
                return Err(CocoanutError::ControlCreationFailed("TextField creation failed".into()));
            }

            let text_ns = ns_string!(placeholder);
            let _: () = msg_send![ns_tf, setStringValue: text_ns];

            Ok(TextField {
                base: ControlBase::new(ns_tf, "textfield"),
                text: placeholder.to_string(),
            })
        }
    }

    /// Create a text field builder
    pub fn builder() -> TextFieldBuilder {
        TextFieldBuilder::default()
    }
}

impl Drawable for TextField {
    fn as_view(&self) -> *mut Object {
        self.base.as_view()
    }

    fn set_visible(&self, visible: bool) -> Result<()> {
        self.base.set_visible(visible)
    }

    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }
}

impl Positionable for TextField {
    fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
        self.base.set_frame(x, y, width, height)
    }

    fn frame(&self) -> (f64, f64, f64, f64) {
        self.base.frame()
    }
}

impl Textual for TextField {
    fn text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        #[cfg(not(feature = "test-mock"))]
        unsafe {
            let text_ns = ns_string!(text);
            let _: () = msg_send![self.base.ns_view, setStringValue: text_ns];
        }
        self.text = text.to_string();
        Ok(())
    }
}

/// TextField builder
#[derive(Default)]
pub struct TextFieldBuilder {
    text: String,
    width: Option<f64>,
    height: Option<f64>,
}

impl TextFieldBuilder {
    /// Set text field content
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Set text field size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Build the text field
    pub fn build(self) -> Result<TextField> {
        let tf = TextField::new(&self.text)?;
        if let (Some(w), Some(h)) = (self.width, self.height) {
            tf.set_frame(0.0, 0.0, w, h)?;
        }
        Ok(tf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_builder() {
        let builder = ButtonBuilder::default()
            .title("Test")
            .size(100.0, 50.0);
        assert_eq!(builder.title, "Test");
        assert_eq!(builder.width, Some(100.0));
    }

    #[test]
    fn test_label_builder() {
        let builder = LabelBuilder::default()
            .text("Label")
            .size(200.0, 30.0);
        assert_eq!(builder.text, "Label");
        assert_eq!(builder.height, Some(30.0));
    }

    #[test]
    fn test_textfield_builder() {
        let builder = TextFieldBuilder::default()
            .text("Input")
            .size(300.0, 40.0);
        assert_eq!(builder.text, "Input");
        assert_eq!(builder.width, Some(300.0));
    }
}
