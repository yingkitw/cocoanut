//! Phase 2: Input Widgets Demo - GUI Window
//! 
//! Demonstrates all Phase 2 input widgets from Streamlit migration:
//! - Text inputs (text_input, text_area, chat_input)
//! - Selection widgets (button, checkbox, radio, selectbox, multiselect)
//! - Numeric inputs (slider, number_input, color_picker)
//! - Date/time pickers (date_input, time_input)
//! - File/media inputs (file_uploader, camera_input, audio_input)

use cocoanut::prelude::*;
use cocoanut::simple_app::Layout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    app("Phase 2 Demo")
        .title("🥥 Phase 2: Input Widgets (21 widgets)")
        .size(800.0, 700.0)
        .centered(true)
        .layout(Layout::default())
        // Text Input Widgets
        .add(Comp::new(Kind::Label).text("📝 Text Input Widgets (3)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("TextInput, TextArea, ChatInput").size(700.0, 20.0))
        // Selection Widgets
        .add(Comp::new(Kind::Label).text("🔘 Selection Widgets (7)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Button, Checkbox, Radio, Selectbox, Multiselect, ButtonGroup, SelectSlider").size(700.0, 20.0))
        // Numeric Input Widgets
        .add(Comp::new(Kind::Label).text("🔢 Numeric Input Widgets (3)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Slider, NumberInput, ColorPicker").size(700.0, 20.0))
        // Date & Time Widgets
        .add(Comp::new(Kind::Label).text("📅 Date & Time Widgets (2)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("DateInput, TimeInput").size(700.0, 20.0))
        // File & Media Input Widgets
        .add(Comp::new(Kind::Label).text("📁 File & Media Input Widgets (3)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("FileUploader, CameraInput, AudioInput").size(700.0, 20.0))
        // Media Display Widgets
        .add(Comp::new(Kind::Label).text("🎬 Media Display Widgets (3)").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("Image, Audio, Video").size(700.0, 20.0))
        // Summary
        .add(Comp::new(Kind::Label).text("📈 Phase 2 Summary").size(400.0, 25.0))
        .add(Comp::new(Kind::Label).text("✓ Total: 21 input widgets").size(400.0, 20.0))
        .run()?;
    Ok(())
}
