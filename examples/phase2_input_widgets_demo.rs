//! Phase 2: Input Widgets Demo
//! 
//! Demonstrates all Phase 2 input widgets from Streamlit migration:
//! - Text inputs (text_input, text_area, chat_input)
//! - Selection widgets (button, checkbox, radio, selectbox, multiselect)
//! - Numeric inputs (slider, number_input, color_picker)
//! - Date/time pickers (date_input, time_input)
//! - File/media inputs (file_uploader, camera_input, audio_input)
//! - Media display (image, audio, video)

use cocoanut::systems::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🥥 Phase 2: Input Widgets Demo\n");

    // Text Input Widgets
    println!("📝 Text Input Widgets:");
    
    let text_input = TextInput::new()
        .placeholder("Enter your name")
        .max_chars(50);
    println!("  ✓ TextInput: placeholder={:?}, max_chars={:?}", 
        text_input.get_placeholder(), 
        text_input.get_value());

    let text_area = TextArea::new()
        .placeholder("Enter your message")
        .rows(5);
    println!("  ✓ TextArea: rows={}", text_area.get_rows());

    let chat_input = ChatInput::new().placeholder("Type a message...");
    println!("  ✓ ChatInput: placeholder={:?}", chat_input.get_placeholder());

    // Selection Widgets
    println!("\n🔘 Selection Widgets:");

    let button = Button::new("Click Me").variant(ButtonVariant::Primary);
    println!("  ✓ Button: label='{}', variant={:?}", 
        button.get_label(), button.get_variant());

    let checkbox = Checkbox::new("I agree").checked(true);
    println!("  ✓ Checkbox: label='{}', checked={}", 
        checkbox.get_label(), checkbox.is_checked());

    let radio = Radio::new(
        "Choose one",
        vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string()],
    )?;
    println!("  ✓ Radio: options={}, selected={:?}", 
        radio.get_options().len(), radio.get_selected_text());

    let selectbox = Selectbox::new(
        "Select item",
        vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()],
    )?;
    println!("  ✓ Selectbox: options={}", selectbox.get_options().len());

    let multiselect = Multiselect::new(
        "Select multiple",
        vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()],
    )?
    .select(0)?
    .select(2)?;
    println!("  ✓ Multiselect: options={}, selected={}", 
        multiselect.get_options().len(), multiselect.get_selected().len());

    let button_group = ButtonGroup::new(
        "Choose",
        vec!["Left".to_string(), "Center".to_string(), "Right".to_string()],
    )?;
    println!("  ✓ ButtonGroup: options={}, selected={:?}", 
        button_group.get_options().len(), button_group.get_selected());

    // Numeric Input Widgets
    println!("\n🔢 Numeric Input Widgets:");

    let slider = Slider::new(0.0, 100.0)?
        .value(50.0)?
        .step(5.0)
        .label("Volume");
    println!("  ✓ Slider: range=[{}, {}], value={}, step={}", 
        slider.get_min(), slider.get_max(), slider.get_value(), 5.0);

    let number_input = NumberInput::new(42.0)
        .min(0.0)
        .max(100.0)
        .step(0.5)
        .label("Count");
    println!("  ✓ NumberInput: value={}, range=[{:?}, {:?}]", 
        number_input.get_value(), number_input.get_min(), number_input.get_max());

    let color_picker = ColorPicker::new("#FF5733").label("Pick Color");
    println!("  ✓ ColorPicker: color='{}', label={:?}", 
        color_picker.get_color(), color_picker.get_label());

    // Date/Time Input Widgets
    println!("\n📅 Date & Time Input Widgets:");

    let date_input = DateInput::new()
        .value("2025-10-25")
        .with_label("Select date");
    println!("  ✓ DateInput: value='{}', label={:?}", 
        date_input.get_value(), date_input.get_label());

    let time_input = TimeInput::new()
        .value("14:30")
        .with_label("Select time");
    println!("  ✓ TimeInput: value='{}', label={:?}", 
        time_input.get_value(), time_input.get_label());

    // File & Media Input Widgets
    println!("\n📁 File & Media Input Widgets:");

    let file_uploader = FileUploader::new()
        .label("Upload file")
        .accept(".pdf")
        .accept(".doc")
        .multiple(true);
    println!("  ✓ FileUploader: accept_types={}, multiple={}", 
        file_uploader.get_accept_types().len(), file_uploader.allows_multiple());

    let camera_input = CameraInput::new().label("Take Photo");
    println!("  ✓ CameraInput: label={:?}", camera_input.get_label());

    let audio_input = AudioInput::new()
        .label("Record Audio")
        .max_duration(120.0);
    println!("  ✓ AudioInput: label={:?}, max_duration={:?}", 
        audio_input.get_label(), audio_input.get_max_duration());

    // Media Display Widgets
    println!("\n🎬 Media Display Widgets:");

    let image = Image::new("image.jpg")
        .caption("Sample Image")
        .width(300.0);
    println!("  ✓ Image: source='{}', caption={:?}, width={:?}", 
        image.get_source(), image.get_caption(), image.get_width());

    let audio = Audio::new("song.mp3")
        .format("mp3")
        .start_time(10.5);
    println!("  ✓ Audio: source='{}', format={:?}, start_time={}", 
        audio.get_source(), audio.get_format(), audio.get_start_time());

    let video = Video::new("video.mp4")
        .format("mp4")
        .start_time(5.0)
        .width(640.0);
    println!("  ✓ Video: source='{}', format={:?}, width={:?}", 
        video.get_source(), video.get_format(), video.get_width());

    // Summary
    println!("\n📈 Phase 2 Summary:");
    println!("  ✓ Text Input Widgets: 3 types");
    println!("  ✓ Selection Widgets: 7 types");
    println!("  ✓ Numeric Input Widgets: 3 types");
    println!("  ✓ Date & Time Widgets: 2 types");
    println!("  ✓ File & Media Input Widgets: 3 types");
    println!("  ✓ Media Display Widgets: 3 types");
    println!("  ✓ Total: 21 input widgets implemented");

    println!("\n✨ Phase 2 Complete!");
    println!("Next: Phase 3 - Advanced Layouts (v0.5.0)");

    Ok(())
}
