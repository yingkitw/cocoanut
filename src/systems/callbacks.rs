//! Phase 4: Callbacks & Event Handlers
//! 
//! Implements callback system for state changes and events.

use crate::core::error::Result;
use std::sync::{Arc, Mutex};

/// Change callback - triggered when value changes
pub struct ChangeCallback<T: Clone + Send + Sync + 'static> {
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(T) + Send + Sync>>>>,
}

impl<T: Clone + Send + Sync + 'static> ChangeCallback<T> {
    /// Create a new change callback
    pub fn new() -> Self {
        ChangeCallback {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register a callback
    pub fn on_change<F>(&self, callback: F) -> Result<()>
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        callbacks.push(Box::new(callback));
        Ok(())
    }

    /// Trigger all callbacks
    pub fn trigger(&self, value: T) -> Result<()> {
        let callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        for callback in callbacks.iter() {
            callback(value.clone());
        }
        Ok(())
    }

    /// Get number of registered callbacks
    pub fn count(&self) -> Result<usize> {
        let callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        Ok(callbacks.len())
    }

    /// Clear all callbacks
    pub fn clear(&self) -> Result<()> {
        let mut callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        callbacks.clear();
        Ok(())
    }
}

impl<T: Clone + Send + Sync> Default for ChangeCallback<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + Sync> Clone for ChangeCallback<T> {
    fn clone(&self) -> Self {
        ChangeCallback {
            callbacks: Arc::clone(&self.callbacks),
        }
    }
}

/// Click callback - triggered on click events
pub struct ClickCallback {
    callbacks: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>>,
}

impl ClickCallback {
    /// Create a new click callback
    pub fn new() -> Self {
        ClickCallback {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register a callback
    pub fn on_click<F>(&self, callback: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        callbacks.push(Box::new(callback));
        Ok(())
    }

    /// Trigger all callbacks
    pub fn trigger(&self) -> Result<()> {
        let callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        for callback in callbacks.iter() {
            callback();
        }
        Ok(())
    }

    /// Get number of registered callbacks
    pub fn count(&self) -> Result<usize> {
        let callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        Ok(callbacks.len())
    }

    /// Clear all callbacks
    pub fn clear(&self) -> Result<()> {
        let mut callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        callbacks.clear();
        Ok(())
    }
}

impl Default for ClickCallback {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ClickCallback {
    fn clone(&self) -> Self {
        ClickCallback {
            callbacks: Arc::clone(&self.callbacks),
        }
    }
}

/// Submit callback - triggered on form submission
pub struct SubmitCallback {
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(String) + Send + Sync>>>>,
}

impl SubmitCallback {
    /// Create a new submit callback
    pub fn new() -> Self {
        SubmitCallback {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register a callback with form data
    pub fn on_submit<F>(&self, callback: F) -> Result<()>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        callbacks.push(Box::new(callback));
        Ok(())
    }

    /// Trigger all callbacks with form data
    pub fn trigger(&self, form_data: String) -> Result<()> {
        let callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        for callback in callbacks.iter() {
            callback(form_data.clone());
        }
        Ok(())
    }

    /// Get number of registered callbacks
    pub fn count(&self) -> Result<usize> {
        let callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        Ok(callbacks.len())
    }

    /// Clear all callbacks
    pub fn clear(&self) -> Result<()> {
        let mut callbacks = self.callbacks.lock().map_err(|_| "Failed to lock callbacks")?;
        callbacks.clear();
        Ok(())
    }
}

impl Default for SubmitCallback {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SubmitCallback {
    fn clone(&self) -> Self {
        SubmitCallback {
            callbacks: Arc::clone(&self.callbacks),
        }
    }
}

/// Event dispatcher - generic event handling
pub struct EventDispatcher {
    handlers: Arc<Mutex<std::collections::HashMap<String, Vec<Box<dyn Fn() + Send + Sync>>>>>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new() -> Self {
        EventDispatcher {
            handlers: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Register an event handler
    pub fn on<F>(&self, event: &str, handler: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.lock().map_err(|_| "Failed to lock handlers")?;
        handlers
            .entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
        Ok(())
    }

    /// Trigger event handlers
    pub fn trigger(&self, event: &str) -> Result<()> {
        let handlers = self.handlers.lock().map_err(|_| "Failed to lock handlers")?;
        if let Some(event_handlers) = handlers.get(event) {
            for handler in event_handlers.iter() {
                handler();
            }
        }
        Ok(())
    }

    /// Get number of handlers for an event
    pub fn count(&self, event: &str) -> Result<usize> {
        let handlers = self.handlers.lock().map_err(|_| "Failed to lock handlers")?;
        Ok(handlers.get(event).map(|h| h.len()).unwrap_or(0))
    }

    /// Clear handlers for an event
    pub fn clear_event(&self, event: &str) -> Result<()> {
        let mut handlers = self.handlers.lock().map_err(|_| "Failed to lock handlers")?;
        handlers.remove(event);
        Ok(())
    }

    /// Clear all handlers
    pub fn clear_all(&self) -> Result<()> {
        let mut handlers = self.handlers.lock().map_err(|_| "Failed to lock handlers")?;
        handlers.clear();
        Ok(())
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EventDispatcher {
    fn clone(&self) -> Self {
        EventDispatcher {
            handlers: Arc::clone(&self.handlers),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_change_callback() {
        let callback: ChangeCallback<i32> = ChangeCallback::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        callback
            .on_change(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();

        callback.trigger(42).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_click_callback() {
        let callback = ClickCallback::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        callback
            .on_click(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();

        callback.trigger().unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_submit_callback() {
        let callback = SubmitCallback::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        callback
            .on_submit(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();

        callback.trigger("form_data".to_string()).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_dispatcher() {
        let dispatcher = EventDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        dispatcher
            .on("click", move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();

        dispatcher.trigger("click").unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_callback_count() {
        let callback: ChangeCallback<String> = ChangeCallback::new();
        callback.on_change(|_| {}).unwrap();
        callback.on_change(|_| {}).unwrap();
        assert_eq!(callback.count().unwrap(), 2);
    }
}
