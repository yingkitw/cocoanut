//! Phase 2: File & Media Input Widgets
//! 
//! Implements file and media input widgets for macOS GUI.

use crate::core::error::Result;

/// File uploader widget
pub struct FileUploader {
    label: Option<String>,
    accept_types: Vec<String>,
    multiple: bool,
    disabled: bool,
}

impl FileUploader {
    /// Create a new file uploader widget
    pub fn new() -> Self {
        FileUploader {
            label: None,
            accept_types: Vec::new(),
            multiple: false,
            disabled: false,
        }
    }

    /// Set label
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Add accepted file type (e.g., ".pdf", ".jpg", "image/*")
    pub fn accept(mut self, file_type: impl Into<String>) -> Self {
        self.accept_types.push(file_type.into());
        self
    }

    /// Allow multiple file selection
    pub fn multiple(mut self, allow: bool) -> Self {
        self.multiple = allow;
        self
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get accepted types
    pub fn get_accept_types(&self) -> &[String] {
        &self.accept_types
    }

    /// Check if multiple selection allowed
    pub fn allows_multiple(&self) -> bool {
        self.multiple
    }
}

impl Default for FileUploader {
    fn default() -> Self {
        Self::new()
    }
}

/// Camera input widget - capture photo from camera
pub struct CameraInput {
    label: Option<String>,
    disabled: bool,
}

impl CameraInput {
    /// Create a new camera input widget
    pub fn new() -> Self {
        CameraInput {
            label: None,
            disabled: false,
        }
    }

    /// Set label
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl Default for CameraInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Audio input widget - record audio
pub struct AudioInput {
    label: Option<String>,
    max_duration_seconds: Option<f64>,
    disabled: bool,
}

impl AudioInput {
    /// Create a new audio input widget
    pub fn new() -> Self {
        AudioInput {
            label: None,
            max_duration_seconds: None,
            disabled: false,
        }
    }

    /// Set label
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Set maximum recording duration in seconds
    pub fn max_duration(mut self, seconds: f64) -> Self {
        self.max_duration_seconds = Some(seconds);
        self
    }

    crate::disabled_field!();

    /// Get label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get max duration
    pub fn get_max_duration(&self) -> Option<f64> {
        self.max_duration_seconds
    }
}

impl Default for AudioInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Image display widget
pub struct Image {
    source: String,
    caption: Option<String>,
    width: Option<f64>,
    use_column_width: bool,
}

impl Image {
    /// Create a new image widget from URL or file path
    pub fn new(source: impl Into<String>) -> Self {
        Image {
            source: source.into(),
            caption: None,
            width: None,
            use_column_width: false,
        }
    }

    /// Set caption text
    pub fn caption(mut self, text: impl Into<String>) -> Self {
        self.caption = Some(text.into());
        self
    }

    /// Set image width
    pub fn width(mut self, w: f64) -> Self {
        self.width = Some(w);
        self
    }

    /// Use full column width
    pub fn use_column_width(mut self, use_width: bool) -> Self {
        self.use_column_width = use_width;
        self
    }

    /// Get source
    pub fn get_source(&self) -> &str {
        &self.source
    }

    /// Get caption
    pub fn get_caption(&self) -> Option<&str> {
        self.caption.as_deref()
    }

    /// Get width
    pub fn get_width(&self) -> Option<f64> {
        self.width
    }
}

/// Audio player widget
pub struct Audio {
    source: String,
    format: Option<String>,
    start_time: f64,
}

impl Audio {
    /// Create a new audio player widget
    pub fn new(source: impl Into<String>) -> Self {
        Audio {
            source: source.into(),
            format: None,
            start_time: 0.0,
        }
    }

    /// Set audio format (e.g., "mp3", "wav", "ogg")
    pub fn format(mut self, fmt: impl Into<String>) -> Self {
        self.format = Some(fmt.into());
        self
    }

    /// Set start time in seconds
    pub fn start_time(mut self, time: f64) -> Self {
        self.start_time = time;
        self
    }

    /// Get source
    pub fn get_source(&self) -> &str {
        &self.source
    }

    /// Get format
    pub fn get_format(&self) -> Option<&str> {
        self.format.as_deref()
    }

    /// Get start time
    pub fn get_start_time(&self) -> f64 {
        self.start_time
    }
}

/// Video player widget
pub struct Video {
    source: String,
    format: Option<String>,
    start_time: f64,
    width: Option<f64>,
}

impl Video {
    /// Create a new video player widget
    pub fn new(source: impl Into<String>) -> Self {
        Video {
            source: source.into(),
            format: None,
            start_time: 0.0,
            width: None,
        }
    }

    /// Set video format (e.g., "mp4", "webm", "ogg")
    pub fn format(mut self, fmt: impl Into<String>) -> Self {
        self.format = Some(fmt.into());
        self
    }

    /// Set start time in seconds
    pub fn start_time(mut self, time: f64) -> Self {
        self.start_time = time;
        self
    }

    /// Set video width
    pub fn width(mut self, w: f64) -> Self {
        self.width = Some(w);
        self
    }

    /// Get source
    pub fn get_source(&self) -> &str {
        &self.source
    }

    /// Get format
    pub fn get_format(&self) -> Option<&str> {
        self.format.as_deref()
    }

    /// Get start time
    pub fn get_start_time(&self) -> f64 {
        self.start_time
    }

    /// Get width
    pub fn get_width(&self) -> Option<f64> {
        self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_uploader() {
        let uploader = FileUploader::new()
            .accept(".pdf")
            .accept(".doc")
            .multiple(true);
        assert_eq!(uploader.get_accept_types().len(), 2);
        assert!(uploader.allows_multiple());
    }

    #[test]
    fn test_camera_input() {
        let camera = CameraInput::new().label("Take Photo");
        assert_eq!(camera.get_label(), Some("Take Photo"));
    }

    #[test]
    fn test_audio_input() {
        let audio = AudioInput::new().max_duration(60.0);
        assert_eq!(audio.get_max_duration(), Some(60.0));
    }

    #[test]
    fn test_image_widget() {
        let img = Image::new("image.jpg").caption("My Image").width(300.0);
        assert_eq!(img.get_source(), "image.jpg");
        assert_eq!(img.get_caption(), Some("My Image"));
        assert_eq!(img.get_width(), Some(300.0));
    }

    #[test]
    fn test_audio_player() {
        let audio = Audio::new("song.mp3").format("mp3").start_time(10.5);
        assert_eq!(audio.get_format(), Some("mp3"));
        assert_eq!(audio.get_start_time(), 10.5);
    }

    #[test]
    fn test_video_player() {
        let video = Video::new("video.mp4")
            .format("mp4")
            .start_time(5.0)
            .width(640.0);
        assert_eq!(video.get_source(), "video.mp4");
        assert_eq!(video.get_width(), Some(640.0));
    }
}
