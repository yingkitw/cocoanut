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
pub struct UIEventStream {
    receiver: mpsc::UnboundedReceiver<UIEvent>,
}

impl UIEventStream {
    /// Create a new UI event stream
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
pub struct UIEventSender {
    sender: mpsc::UnboundedSender<UIEvent>,
}

impl UIEventSender {
    /// Send a UI event
    pub fn send(&self, event: UIEvent) -> Result<()> {
        self.sender.send(event).map_err(|_| CocoanutError::SystemError("Failed to send UI event".to_string()))
    }
}

/// UI event types
#[derive(Debug, Clone)]
pub enum UIEvent {
    /// Button clicked
    ButtonClick { id: String },
    /// Window resized
    WindowResize { width: f64, height: f64 },
    /// Text field changed
    TextChanged { id: String, text: String },
    /// Menu item selected
    MenuSelected { id: String },
    /// Custom event
    Custom { name: String, data: serde_json::Value },
}

/// Reactive UI component trait
pub trait ReactiveUI {
    /// Get the event stream for this component
    fn event_stream(&self) -> BoxStream<'static, UIEvent>;
    
    /// Subscribe to events
    fn subscribe(&self, callback: Box<dyn Fn(UIEvent) + Send + Sync>) -> Subscription;
}

/// Subscription handle for unsubscribing
pub struct Subscription {
    unsubscribe: Box<dyn Fn() + Send + Sync>,
}

impl Subscription {
    /// Create a new subscription
    pub fn new<F>(unsubscribe: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            unsubscribe: Box::new(unsubscribe),
        }
    }
    
    /// Unsubscribe from the stream
    pub fn unsubscribe(self) {
        (self.unsubscribe)();
    }
}

/// Reactive button implementation
pub struct ReactiveButton {
    event_sender: UIEventSender,
    id: String,
}

impl ReactiveButton {
    /// Create a new reactive button
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
    
    /// Simulate button click
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
pub struct ReactiveTextField {
    event_sender: UIEventSender,
    id: String,
    text: String,
}

impl ReactiveTextField {
    /// Create a new reactive text field
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
    
    /// Set text and emit event
    pub fn set_text(&mut self, text: String) -> Result<()> {
        self.text = text.clone();
        self.event_sender.send(UIEvent::TextChanged {
            id: self.id.clone(),
            text,
        })
    }
    
    /// Get current text
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
pub struct ReactiveWindow {
    event_sender: UIEventSender,
    id: String,
    width: f64,
    height: f64,
}

impl ReactiveWindow {
    /// Create a new reactive window
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
    
    /// Resize window and emit event
    pub fn resize(&mut self, width: f64, height: f64) -> Result<()> {
        self.width = width;
        self.height = height;
        self.event_sender.send(UIEvent::WindowResize { width, height })
    }
    
    /// Get current size
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
    pub fn add_stream<S>(&mut self, stream: S)
    where
        S: Stream<Item = UIEvent> + Send + 'static,
    {
        self.streams.push(stream.boxed());
    }
    
    /// Merge all streams into one
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
    pub fn add_component<C>(&mut self, component: C)
    where
        C: ReactiveUI + Send + Sync + 'static,
    {
        self.aggregator.add_stream(component.event_stream());
        self.components.push(Box::new(component));
    }
    
    /// Get the merged event stream
    pub fn event_stream(self) -> BoxStream<'static, UIEvent> {
        self.aggregator.merge()
    }
    
    /// Process events with a callback
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
}
