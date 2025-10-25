//! Phase 1: Status & Feedback Elements
//! 
//! Implements status messages, notifications, and feedback elements for user communication.

use crate::core::error::Result;

/// Status message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusType {
    /// Success message (green)
    Success,
    /// Error message (red)
    Error,
    /// Warning message (orange/yellow)
    Warning,
    /// Info message (blue)
    Info,
}

/// Success message element
pub struct Success {
    message: String,
    icon: bool,
}

impl Success {
    /// Create a new success message
    pub fn new(message: impl Into<String>) -> Self {
        Success {
            message: message.into(),
            icon: true,
        }
    }

    /// Set whether to show icon
    pub fn icon(mut self, show: bool) -> Self {
        self.icon = show;
        self
    }

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Check if icon is shown
    pub fn shows_icon(&self) -> bool {
        self.icon
    }
}

/// Error message element
pub struct Error {
    message: String,
    icon: bool,
}

impl Error {
    /// Create a new error message
    pub fn new(message: impl Into<String>) -> Self {
        Error {
            message: message.into(),
            icon: true,
        }
    }

    /// Set whether to show icon
    pub fn icon(mut self, show: bool) -> Self {
        self.icon = show;
        self
    }

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Check if icon is shown
    pub fn shows_icon(&self) -> bool {
        self.icon
    }
}

/// Warning message element
pub struct Warning {
    message: String,
    icon: bool,
}

impl Warning {
    /// Create a new warning message
    pub fn new(message: impl Into<String>) -> Self {
        Warning {
            message: message.into(),
            icon: true,
        }
    }

    /// Set whether to show icon
    pub fn icon(mut self, show: bool) -> Self {
        self.icon = show;
        self
    }

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Check if icon is shown
    pub fn shows_icon(&self) -> bool {
        self.icon
    }
}

/// Info message element
pub struct Info {
    message: String,
    icon: bool,
}

impl Info {
    /// Create a new info message
    pub fn new(message: impl Into<String>) -> Self {
        Info {
            message: message.into(),
            icon: true,
        }
    }

    /// Set whether to show icon
    pub fn icon(mut self, show: bool) -> Self {
        self.icon = show;
        self
    }

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Check if icon is shown
    pub fn shows_icon(&self) -> bool {
        self.icon
    }
}

/// Toast notification element
pub struct Toast {
    message: String,
    toast_type: StatusType,
    duration: f64,
}

impl Toast {
    /// Create a new toast notification
    pub fn new(message: impl Into<String>, toast_type: StatusType) -> Self {
        Toast {
            message: message.into(),
            toast_type,
            duration: 3.0,
        }
    }

    /// Set the duration in seconds
    pub fn with_duration(mut self, seconds: f64) -> Self {
        self.duration = seconds;
        self
    }

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the toast type
    pub fn toast_type(&self) -> StatusType {
        self.toast_type
    }

    /// Get the duration
    pub fn get_duration(&self) -> f64 {
        self.duration
    }
}

/// Status container with spinner
pub struct Status {
    label: String,
    state: StatusState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusState {
    /// Running/in progress
    Running,
    /// Completed successfully
    Complete,
    /// Failed
    Failed,
}

impl Status {
    /// Create a new status container
    pub fn new(label: impl Into<String>) -> Self {
        Status {
            label: label.into(),
            state: StatusState::Running,
        }
    }

    /// Set the status state
    pub fn with_state(mut self, state: StatusState) -> Self {
        self.state = state;
        self
    }

    /// Get the label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current state
    pub fn get_state(&self) -> StatusState {
        self.state
    }
}

/// Progress bar element
pub struct Progress {
    value: f64,
    max_value: f64,
    text: Option<String>,
}

impl Progress {
    /// Create a new progress bar
    pub fn new(value: f64, max_value: f64) -> Result<Self> {
        if value < 0.0 || value > max_value || max_value <= 0.0 {
            return Err("Invalid progress values".into());
        }
        Ok(Progress {
            value,
            max_value,
            text: None,
        })
    }

    /// Set custom text to display
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Get the current value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the max value
    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    /// Get the progress percentage (0-100)
    pub fn percentage(&self) -> f64 {
        (self.value / self.max_value) * 100.0
    }

    /// Get the custom text
    pub fn get_text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

/// Loading spinner element
pub struct Spinner {
    text: Option<String>,
}

impl Spinner {
    /// Create a new spinner
    pub fn new() -> Self {
        Spinner { text: None }
    }

    /// Set the spinner text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Get the spinner text
    pub fn get_text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_message() {
        let success = Success::new("Operation completed").icon(true);
        assert_eq!(success.message(), "Operation completed");
        assert!(success.shows_icon());
    }

    #[test]
    fn test_error_message() {
        let error = Error::new("Something went wrong");
        assert_eq!(error.message(), "Something went wrong");
    }

    #[test]
    fn test_warning_message() {
        let warning = Warning::new("Be careful");
        assert_eq!(warning.message(), "Be careful");
    }

    #[test]
    fn test_info_message() {
        let info = Info::new("FYI").icon(false);
        assert!(!info.shows_icon());
    }

    #[test]
    fn test_toast_notification() {
        let toast = Toast::new("Hello", StatusType::Success).with_duration(5.0);
        assert_eq!(toast.message(), "Hello");
        assert_eq!(toast.toast_type(), StatusType::Success);
        assert_eq!(toast.get_duration(), 5.0);
    }

    #[test]
    fn test_status_container() {
        let status = Status::new("Processing").with_state(StatusState::Running);
        assert_eq!(status.get_state(), StatusState::Running);
    }

    #[test]
    fn test_progress_bar() {
        let progress = Progress::new(50.0, 100.0).unwrap();
        assert_eq!(progress.percentage(), 50.0);
    }

    #[test]
    fn test_progress_invalid() {
        let result = Progress::new(150.0, 100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_spinner() {
        let spinner = Spinner::new().with_text("Loading...");
        assert_eq!(spinner.get_text(), Some("Loading..."));
    }
}
