//! Essential features for Cocoanut
//!
//! This module implements Priority 2 essential features:
//! - Event system with callback-based handling
//! - Auto Layout integration
//! - Core Animation wrapper
//! - Custom view support
//! - Reactive data binding

use crate::core::error::Result;
use std::sync::Arc;

/// Event callback type
pub type EventCallback = Arc<dyn Fn() + Send + Sync>;

/// Event system for callback-based event handling
pub struct EventSystem {
    callbacks: std::sync::Mutex<Vec<(String, EventCallback)>>,
}

impl EventSystem {
    /// Create a new event system
    pub fn new() -> Self {
        Self {
            callbacks: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Register an event callback
    pub fn on<F>(&self, event_name: &str, callback: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        callbacks.push((event_name.to_string(), Arc::new(callback)));
        Ok(())
    }

    /// Emit an event
    pub fn emit(&self, event_name: &str) -> Result<()> {
        let callbacks = self.callbacks.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        for (name, callback) in callbacks.iter() {
            if name == event_name {
                callback();
            }
        }
        Ok(())
    }

    /// Remove all callbacks for an event
    pub fn off(&self, event_name: &str) -> Result<()> {
        let mut callbacks = self.callbacks.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        callbacks.retain(|(name, _)| name != event_name);
        Ok(())
    }

    /// Get callback count
    pub fn callback_count(&self) -> Result<usize> {
        let callbacks = self.callbacks.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;
        Ok(callbacks.len())
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Auto Layout constraint builder
pub struct LayoutConstraint {
    /// Constraint identifier
    pub identifier: String,
    /// Constraint priority (0-1000)
    pub priority: f64,
    /// Constraint constant
    pub constant: f64,
}

impl LayoutConstraint {
    /// Create a new layout constraint
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            priority: 750.0, // Default priority
            constant: 0.0,
        }
    }

    /// Set constraint priority
    pub fn priority(mut self, priority: f64) -> Self {
        self.priority = priority.clamp(0.0, 1000.0);
        self
    }

    /// Set constraint constant
    pub fn constant(mut self, constant: f64) -> Self {
        self.constant = constant;
        self
    }
}

/// Auto Layout system
pub struct AutoLayout {
    constraints: std::sync::Mutex<Vec<LayoutConstraint>>,
}

impl AutoLayout {
    /// Create a new Auto Layout system
    pub fn new() -> Self {
        Self {
            constraints: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Add a constraint
    pub fn add_constraint(&self, constraint: LayoutConstraint) -> Result<()> {
        let mut constraints = self.constraints.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        constraints.push(constraint);
        Ok(())
    }

    /// Get all constraints
    pub fn constraints(&self) -> Result<Vec<String>> {
        let constraints = self.constraints.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        Ok(constraints.iter().map(|c| c.identifier.clone()).collect())
    }

    /// Remove a constraint
    pub fn remove_constraint(&self, identifier: &str) -> Result<()> {
        let mut constraints = self.constraints.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        constraints.retain(|c| c.identifier != identifier);
        Ok(())
    }
}

impl Default for AutoLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Core Animation wrapper for animations
pub struct Animation {
    /// Animation duration
    pub duration: f64,
    /// Animation delay
    pub delay: f64,
    /// Animation timing function
    pub timing: TimingFunction,
}

/// Animation timing function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingFunction {
    /// Linear timing
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in-out
    EaseInOut,
}

impl Animation {
    /// Create a new animation
    pub fn new(duration: f64) -> Self {
        Self {
            duration,
            delay: 0.0,
            timing: TimingFunction::EaseInOut,
        }
    }

    /// Set animation delay
    pub fn delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// Set animation timing function
    pub fn timing(mut self, timing: TimingFunction) -> Self {
        self.timing = timing;
        self
    }
}

/// Custom view trait for user-defined components
pub trait CustomView {
    /// Draw the view
    fn draw(&self) -> Result<()>;

    /// Handle mouse event
    fn on_mouse_event(&self, x: f64, y: f64) -> Result<()>;

    /// Handle keyboard event
    fn on_key_event(&self, key: &str) -> Result<()>;

    /// Update view
    fn update(&self) -> Result<()>;
}

/// Data binding for reactive updates
pub struct DataBinding<T: Clone + Send + Sync + 'static> {
    value: std::sync::Arc<std::sync::Mutex<T>>,
    observers: std::sync::Mutex<Vec<Arc<dyn Fn(T) + Send + Sync>>>,
}

impl<T: Clone + Send + Sync + 'static> DataBinding<T> {
    /// Create a new data binding
    pub fn new(initial_value: T) -> Self {
        Self {
            value: std::sync::Arc::new(std::sync::Mutex::new(initial_value)),
            observers: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Get the current value
    pub fn get(&self) -> Result<T> {
        Ok(self.value.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?.clone())
    }

    /// Set the value and notify observers
    pub fn set(&self, new_value: T) -> Result<()> {
        {
            let mut value = self.value.lock().map_err(|_| {
                crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
            })?;
            *value = new_value.clone();
        }

        let observers = self.observers.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        for observer in observers.iter() {
            observer(new_value.clone());
        }

        Ok(())
    }

    /// Subscribe to changes
    pub fn subscribe<F>(&self, observer: F) -> Result<()>
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let mut observers = self.observers.lock().map_err(|_| {
            crate::core::error::CocoanutError::ThreadingError("Failed to acquire lock".into())
        })?;

        observers.push(Arc::new(observer));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_system() {
        let system = EventSystem::new();
        let called = std::sync::Arc::new(std::sync::Mutex::new(false));
        let called_clone = called.clone();

        system
            .on("test", move || {
                *called_clone.lock().unwrap() = true;
            })
            .unwrap();

        system.emit("test").unwrap();
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_layout_constraint() {
        let constraint = LayoutConstraint::new("test")
            .priority(800.0)
            .constant(10.0);

        assert_eq!(constraint.identifier, "test");
        assert_eq!(constraint.priority, 800.0);
        assert_eq!(constraint.constant, 10.0);
    }

    #[test]
    fn test_auto_layout() {
        let layout = AutoLayout::new();
        let constraint = LayoutConstraint::new("c1");
        layout.add_constraint(constraint).unwrap();

        let constraints = layout.constraints().unwrap();
        assert_eq!(constraints.len(), 1);
    }

    #[test]
    fn test_animation() {
        let anim = Animation::new(0.3)
            .delay(0.1)
            .timing(TimingFunction::EaseOut);

        assert_eq!(anim.duration, 0.3);
        assert_eq!(anim.delay, 0.1);
        assert_eq!(anim.timing, TimingFunction::EaseOut);
    }

    #[test]
    fn test_data_binding() {
        let binding = DataBinding::new(42);
        assert_eq!(binding.get().unwrap(), 42);

        binding.set(100).unwrap();
        assert_eq!(binding.get().unwrap(), 100);
    }

    #[test]
    fn test_data_binding_subscription() {
        let binding = DataBinding::new(0);
        let received = std::sync::Arc::new(std::sync::Mutex::new(0));
        let received_clone = received.clone();

        binding
            .subscribe(move |value| {
                *received_clone.lock().unwrap() = value;
            })
            .unwrap();

        binding.set(42).unwrap();
        assert_eq!(*received.lock().unwrap(), 42);
    }
}
