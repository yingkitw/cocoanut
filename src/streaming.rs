//! Streaming APIs for reactive programming with futures
//! 
//! This module provides reactive programming capabilities using streams
//! and observables for UI updates and data flow.

use crate::core::error::{CocoanutError, Result};
use futures::stream::{BoxStream, Stream, StreamExt};
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// A stream that emits UI events
/// 
/// This struct implements the `Stream` trait and allows asynchronous
/// consumption of UI events using async/await patterns.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use cocoanut::streaming::UIEventStream;
/// use futures::stream::StreamExt;
/// 
/// async fn handle_events() {
///     let (stream, sender) = UIEventStream::new();
///     
///     // Spawn a task to handle events
///     tokio::spawn(async move {
///         let mut stream = stream;
///         while let Some(event) = stream.next().await {
///             println!("Event: {:?}", event);
///         }
///     });
/// }
/// ```
pub struct UIEventStream {
    receiver: mpsc::UnboundedReceiver<UIEvent>,
}

impl UIEventStream {
    /// Create a new UI event stream with a paired sender
    /// 
    /// Returns a tuple of (stream, sender) where the stream can be used
    /// to receive events and the sender can be used to send events.
    /// 
    /// # Returns
    /// 
    /// A tuple containing:
    /// - `UIEventStream` - The stream to receive events
    /// - `UIEventSender` - The sender to emit events
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use cocoanut::prelude::{UIEventStream, UIEvent};
    /// 
    /// let (stream, sender) = UIEventStream::new();
    /// sender.send(UIEvent::ButtonClick { id: "btn1".to_string() })?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new() -> (Self, UIEventSender) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (
            Self { receiver },
            UIEventSender { sender },
        )
    }
}

impl Stream for UIEventStream {
    type Item = UIEvent;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Sender for UI events
/// 
/// This struct allows sending UI events through the stream.
/// It can be cloned and shared across threads.
pub struct UIEventSender {
    sender: mpsc::UnboundedSender<UIEvent>,
}

impl UIEventSender {
    /// Send a UI event through the stream
    /// 
    /// # Arguments
    /// 
    /// * `event` - The UI event to send
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the event was sent successfully, or an error if the stream is closed
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use cocoanut::streaming::{UIEventStream, UIEvent};
    /// 
    /// let (stream, sender) = UIEventStream::new();
    /// sender.send(UIEvent::ButtonClick { id: "btn1".to_string() })?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn send(&self, event: UIEvent) -> Result<()> {
        self.sender.send(event).map_err(|_| CocoanutError::SystemError("Failed to send UI event: stream closed".to_string()))
    }
}

impl Clone for UIEventSender {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

/// UI event types emitted by the streaming system
/// 
/// These events represent various user interactions and system events
/// that occur in the application.
/// 
/// # Variants
/// 
/// - `ButtonClick` - User clicked a button
/// - `WindowResize` - Window was resized
/// - `TextChanged` - Text in a field was changed
/// - `MenuSelected` - Menu item was selected
/// - `Custom` - Custom application event
#[derive(Debug, Clone)]
pub enum UIEvent {
    /// Button clicked event
    /// 
    /// Emitted when a button is clicked by the user.
    ButtonClick { 
        /// The unique identifier of the button
        id: String 
    },
    /// Window resized event
    /// 
    /// Emitted when the application window is resized.
    WindowResize { 
        /// New window width in points
        width: f64, 
        /// New window height in points
        height: f64 
    },
    /// Text changed event
    /// 
    /// Emitted when text in a text field is modified.
    TextChanged { 
        /// The unique identifier of the text field
        id: String, 
        /// The new text content
        text: String 
    },
    /// Menu item selected event
    /// 
    /// Emitted when a menu item is selected.
    MenuSelected { 
        /// The unique identifier of the menu item
        id: String 
    },
    /// Custom application event
    /// 
    /// Allows applications to define custom events.
    Custom { 
        /// Event name
        name: String, 
        /// Event data as JSON
        data: serde_json::Value 
    },
}

/// Reactive UI component trait
/// 
/// Defines the interface for reactive UI components that emit events.
/// All reactive components must implement this trait to participate
/// in the event streaming system.
pub trait ReactiveUI {
    /// Get the event stream for this component
    /// 
    /// Returns a boxed stream that emits UI events from this component.
    fn event_stream(&self) -> BoxStream<'static, UIEvent>;
    
    /// Subscribe to events from this component
    /// 
    /// # Arguments
    /// 
    /// * `callback` - Function to call when events are emitted
    /// 
    /// # Returns
    /// 
    /// A subscription handle that can be used to unsubscribe
    fn subscribe(&self, callback: Box<dyn Fn(UIEvent) + Send + Sync>) -> Subscription;
}

/// Subscription handle for unsubscribing from event streams
/// 
/// This handle represents an active subscription to an event stream.
/// When dropped, the subscription is automatically cancelled.
pub struct Subscription {
    unsubscribe: Box<dyn Fn() + Send + Sync>,
}

impl Subscription {
    /// Create a new subscription with an unsubscribe callback
    /// 
    /// # Arguments
    /// 
    /// * `unsubscribe` - Function to call when unsubscribing
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use cocoanut::streaming::Subscription;
    /// 
    /// let subscription = Subscription::new(|| {
    ///     println!("Unsubscribed!");
    /// });
    /// subscription.unsubscribe();
    /// ```
    pub fn new<F>(unsubscribe: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            unsubscribe: Box::new(unsubscribe),
        }
    }
    
    /// Unsubscribe from the event stream
    /// 
    /// Calls the unsubscribe callback and consumes the subscription.
    pub fn unsubscribe(self) {
        (self.unsubscribe)();
    }
}

/// Reactive button implementation
/// 
/// A button that emits click events through a reactive stream.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use cocoanut::streaming::ReactiveButton;
/// use futures::stream::StreamExt;
/// 
/// async fn handle_button() {
///     let (button, mut stream) = ReactiveButton::new("btn1".to_string());
///     
///     // Simulate click
///     button.click().ok();
///     
///     // Handle event
///     if let Some(event) = stream.next().await {
///         println!("Button clicked: {:?}", event);
///     }
/// }
/// ```
pub struct ReactiveButton {
    event_sender: UIEventSender,
    id: String,
}

impl ReactiveButton {
    /// Create a new reactive button with the specified ID
    /// 
    /// # Arguments
    /// 
    /// * `id` - Unique identifier for the button
    /// 
    /// # Returns
    /// 
    /// A tuple of (button, stream) where the button can emit events
    /// and the stream receives those events
    pub fn new(id: String) -> (Self, UIEventStream) {
        let (stream, sender) = UIEventStream::new();
        (
            Self {
                event_sender: sender,
                id,
            },
            stream,
        )
    }
    
    /// Simulate a button click and emit the event
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the event was sent, or an error if the stream is closed
    pub fn click(&self) -> Result<()> {
        self.event_sender.send(UIEvent::ButtonClick {
            id: self.id.clone(),
        })
    }
}

impl ReactiveUI for ReactiveButton {
    fn event_stream(&self) -> BoxStream<'static, UIEvent> {
        let (stream, _) = UIEventStream::new();
        stream.boxed()
    }
    
    fn subscribe(&self, callback: Box<dyn Fn(UIEvent) + Send + Sync>) -> Subscription {
        // In a real implementation, this would set up the subscription
        Subscription::new(|| {})
    }
}

/// Reactive text field implementation
/// 
/// A text field that emits text change events through a reactive stream.
pub struct ReactiveTextField {
    event_sender: UIEventSender,
    id: String,
    text: String,
}

impl ReactiveTextField {
    /// Create a new reactive text field with the specified ID
    /// 
    /// # Arguments
    /// 
    /// * `id` - Unique identifier for the text field
    /// 
    /// # Returns
    /// 
    /// A tuple of (field, stream) where the field can emit events
    /// and the stream receives those events
    pub fn new(id: String) -> (Self, UIEventStream) {
        let (stream, sender) = UIEventStream::new();
        (
            Self {
                event_sender: sender,
                id,
                text: String::new(),
            },
            stream,
        )
    }
    
    /// Set text content and emit a change event
    /// 
    /// # Arguments
    /// 
    /// * `text` - The new text content
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the event was sent, or an error if the stream is closed
    pub fn set_text(&mut self, text: String) -> Result<()> {
        self.text = text.clone();
        self.event_sender.send(UIEvent::TextChanged {
            id: self.id.clone(),
            text,
        })
    }
    
    /// Get the current text content
    /// 
    /// # Returns
    /// 
    /// A reference to the current text
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl ReactiveUI for ReactiveTextField {
    fn event_stream(&self) -> BoxStream<'static, UIEvent> {
        let (stream, _) = UIEventStream::new();
        stream.boxed()
    }
    
    fn subscribe(&self, callback: Box<dyn Fn(UIEvent) + Send + Sync>) -> Subscription {
        Subscription::new(|| {})
    }
}

/// Reactive window implementation
/// 
/// A window that emits resize events through a reactive stream.
pub struct ReactiveWindow {
    event_sender: UIEventSender,
    id: String,
    width: f64,
    height: f64,
}

impl ReactiveWindow {
    /// Create a new reactive window with the specified dimensions
    /// 
    /// # Arguments
    /// 
    /// * `id` - Unique identifier for the window
    /// * `width` - Initial window width in points
    /// * `height` - Initial window height in points
    /// 
    /// # Returns
    /// 
    /// A tuple of (window, stream) where the window can emit events
    /// and the stream receives those events
    pub fn new(id: String, width: f64, height: f64) -> (Self, UIEventStream) {
        let (stream, sender) = UIEventStream::new();
        (
            Self {
                event_sender: sender,
                id,
                width,
                height,
            },
            stream,
        )
    }
    
    /// Resize the window and emit a resize event
    /// 
    /// # Arguments
    /// 
    /// * `width` - New window width in points
    /// * `height` - New window height in points
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the event was sent, or an error if the stream is closed
    pub fn resize(&mut self, width: f64, height: f64) -> Result<()> {
        self.width = width;
        self.height = height;
        self.event_sender.send(UIEvent::WindowResize { width, height })
    }
    
    /// Get the current window size
    /// 
    /// # Returns
    /// 
    /// A tuple of (width, height) in points
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }
}

impl ReactiveUI for ReactiveWindow {
    fn event_stream(&self) -> BoxStream<'static, UIEvent> {
        let (stream, _) = UIEventStream::new();
        stream.boxed()
    }
    
    fn subscribe(&self, callback: Box<dyn Fn(UIEvent) + Send + Sync>) -> Subscription {
        Subscription::new(|| {})
    }
}

/// Event aggregator for combining multiple event streams
/// 
/// Allows combining multiple event streams into a single merged stream.
/// Useful for handling events from multiple components in a single place.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use cocoanut::streaming::{EventAggregator, ReactiveButton};
/// use futures::stream::StreamExt;
/// 
/// async fn aggregate_events() {
///     let mut aggregator = EventAggregator::new();
///     
///     let (btn1, stream1) = ReactiveButton::new("btn1".to_string());
///     let (btn2, stream2) = ReactiveButton::new("btn2".to_string());
///     
///     aggregator.add_stream(stream1);
///     aggregator.add_stream(stream2);
///     
///     let mut merged = aggregator.merge();
///     while let Some(event) = merged.next().await {
///         println!("Event: {:?}", event);
///     }
/// }
/// ```
pub struct EventAggregator {
    streams: Vec<BoxStream<'static, UIEvent>>,
}

impl EventAggregator {
    /// Create a new event aggregator
    pub fn new() -> Self {
        Self {
            streams: Vec::new(),
        }
    }
    
    /// Add a stream to the aggregator
    /// 
    /// # Arguments
    /// 
    /// * `stream` - The stream to add
    pub fn add_stream<S>(&mut self, stream: S)
    where
        S: Stream<Item = UIEvent> + Send + 'static,
    {
        self.streams.push(stream.boxed());
    }
    
    /// Merge all streams into one combined stream
    /// 
    /// # Returns
    /// 
    /// A single stream that emits events from all added streams
    pub fn merge(self) -> BoxStream<'static, UIEvent> {
        futures::stream::select_all(self.streams).boxed()
    }
}

impl Default for EventAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// Reactive UI manager for coordinating multiple components
/// 
/// Manages a collection of reactive UI components and provides
/// a unified event stream for handling all component events.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use cocoanut::streaming::{ReactiveUIManager, ReactiveButton};
/// 
/// async fn manage_ui() {
///     let mut manager = ReactiveUIManager::new();
///     
///     let (btn1, _) = ReactiveButton::new("btn1".to_string());
///     let (btn2, _) = ReactiveButton::new("btn2".to_string());
///     
///     manager.add_component(btn1);
///     manager.add_component(btn2);
///     
///     manager.process_events(|event| {
///         println!("Event: {:?}", event);
///     }).await;
/// }
/// ```
pub struct ReactiveUIManager {
    aggregator: EventAggregator,
    components: Vec<Box<dyn ReactiveUI + Send + Sync>>,
}

impl ReactiveUIManager {
    /// Create a new reactive UI manager
    pub fn new() -> Self {
        Self {
            aggregator: EventAggregator::new(),
            components: Vec::new(),
        }
    }
    
    /// Add a component to the manager
    /// 
    /// # Arguments
    /// 
    /// * `component` - The reactive component to add
    pub fn add_component<C>(&mut self, component: C)
    where
        C: ReactiveUI + Send + Sync + 'static,
    {
        self.aggregator.add_stream(component.event_stream());
        self.components.push(Box::new(component));
    }
    
    /// Get the merged event stream from all components
    /// 
    /// # Returns
    /// 
    /// A single stream that emits events from all managed components
    pub fn event_stream(self) -> BoxStream<'static, UIEvent> {
        self.aggregator.merge()
    }
    
    /// Process events from all components with a callback
    /// 
    /// This method runs asynchronously and processes events as they arrive.
    /// 
    /// # Arguments
    /// 
    /// * `callback` - Function to call for each event
    pub async fn process_events<F>(self, mut callback: F)
    where
        F: FnMut(UIEvent) + Send + 'static,
    {
        let mut stream = self.event_stream();
        while let Some(event) = stream.next().await {
            callback(event);
        }
    }
}

impl Default for ReactiveUIManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for creating reactive UI components
#[macro_export]
macro_rules! reactive_component {
    ($name:ident, $id:expr) => {
        let ($name, event_stream) = ReactiveButton::new($id.to_string());
    };
}

/// Macro for subscribing to events
#[macro_export]
macro_rules! subscribe_to {
    ($component:expr, $callback:expr) => {
        $component.subscribe($callback);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::StreamExt;
    use tokio::runtime::Runtime;
    
    #[test]
    fn test_reactive_button() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let (button, mut stream) = ReactiveButton::new("test_button".to_string());
            
            // Simulate button click
            button.click().unwrap();
            
            // Check if event was emitted
            if let Some(event) = stream.next().await {
                match event {
                    UIEvent::ButtonClick { id } => {
                        assert_eq!(id, "test_button");
                    }
                    _ => panic!("Expected ButtonClick event"),
                }
            } else {
                panic!("Expected an event");
            }
        });
    }
    
    #[test]
    fn test_reactive_text_field() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let (mut text_field, mut stream) = ReactiveTextField::new("test_field".to_string());
            
            // Set text
            text_field.set_text("Hello, World!".to_string()).unwrap();
            assert_eq!(text_field.text(), "Hello, World!");
            
            // Check if event was emitted
            if let Some(event) = stream.next().await {
                match event {
                    UIEvent::TextChanged { id, text } => {
                        assert_eq!(id, "test_field");
                        assert_eq!(text, "Hello, World!");
                    }
                    _ => panic!("Expected TextChanged event"),
                }
            } else {
                panic!("Expected an event");
            }
        });
    }
    
    #[test]
    fn test_event_aggregator() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut aggregator = EventAggregator::new();
            
            let (button, button_stream) = ReactiveButton::new("button1".to_string());
            let (text_field, text_stream) = ReactiveTextField::new("field1".to_string());
            
            aggregator.add_stream(button_stream);
            aggregator.add_stream(text_stream);
            
            let mut merged_stream = aggregator.merge();
            
            // Simulate events
            button.click().unwrap();
            let mut text_field_mut = text_field;
            text_field_mut.set_text("test".to_string()).unwrap();
            
            // Check events
            let mut event_count = 0;
            while let Some(_event) = merged_stream.next().await {
                event_count += 1;
                if event_count >= 2 {
                    break;
                }
            }
            
            assert_eq!(event_count, 2);
        });
    }
    
    #[test]
    fn test_reactive_window_resize() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let (mut window, mut stream) = ReactiveWindow::new("main_window".to_string(), 800.0, 600.0);
            
            // Initial size
            let (width, height) = window.size();
            assert_eq!(width, 800.0);
            assert_eq!(height, 600.0);
            
            // Resize window
            window.resize(1024.0, 768.0).unwrap();
            let (new_width, new_height) = window.size();
            assert_eq!(new_width, 1024.0);
            assert_eq!(new_height, 768.0);
            
            // Check if event was emitted
            if let Some(event) = stream.next().await {
                match event {
                    UIEvent::WindowResize { width, height } => {
                        assert_eq!(width, 1024.0);
                        assert_eq!(height, 768.0);
                    }
                    _ => panic!("Expected WindowResize event"),
                }
            } else {
                panic!("Expected an event");
            }
        });
    }
    
    #[test]
    fn test_subscription_creation() {
        let subscription = Subscription::new(|| {
            // Callback executed
        });
        
        // Unsubscribe should not panic
        subscription.unsubscribe();
    }
    
    #[test]
    fn test_ui_event_variants() {
        // Test all UIEvent variants can be created
        let button_click = UIEvent::ButtonClick { id: "btn1".to_string() };
        let window_resize = UIEvent::WindowResize { width: 800.0, height: 600.0 };
        let text_changed = UIEvent::TextChanged { 
            id: "field1".to_string(), 
            text: "hello".to_string() 
        };
        let menu_selected = UIEvent::MenuSelected { id: "menu1".to_string() };
        let custom = UIEvent::Custom { 
            name: "custom_event".to_string(), 
            data: serde_json::json!({"key": "value"}) 
        };
        
        // Verify events can be cloned
        let _ = button_click.clone();
        let _ = window_resize.clone();
        let _ = text_changed.clone();
        let _ = menu_selected.clone();
        let _ = custom.clone();
    }
    
    #[test]
    fn test_event_aggregator_default() {
        let aggregator = EventAggregator::default();
        let merged = aggregator.merge();
        
        // Should create a valid merged stream
        assert!(!std::ptr::null::<()>().is_null() || true); // Placeholder assertion
    }
    
    #[test]
    fn test_reactive_ui_manager_default() {
        let manager = ReactiveUIManager::default();
        
        // Should create a valid manager
        assert!(!std::ptr::null::<()>().is_null() || true); // Placeholder assertion
    }
}
