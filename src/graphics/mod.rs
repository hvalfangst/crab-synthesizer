pub mod sprite;

// Constants for window dimensions
pub const WINDOW_WIDTH: usize = 640;
pub const WINDOW_HEIGHT: usize = 480;

// Constants for tile dimensions
pub const TILE_WIDTH: usize = 64;
pub const TILE_HEIGHT: usize = 48;
pub const TANGENT_WIDTH: usize = 30;

// Constants for waveforms

pub const WAVEFORM_SINE: usize = 0;

pub const WAVEFORM_SQUARE: usize = 1;

// Constants for keys
pub const KEY_IDLE: usize = 0;
pub const KEY_PRESSED: usize = 1;

// Constants for tangents
pub const TANGENT_IDLE: usize = 0;
pub const TANGENT_PRESSED: usize = 1;

// Constants for musical note indices
pub const NOTE_A: usize = 0;
pub const NOTE_A_SHARP: usize = 1;
pub const NOTE_B: usize = 2;
pub const NOTE_C: usize = 3;
pub const NOTE_C_SHARP: usize = 4;
pub const NOTE_D: usize = 5;
pub const NOTE_D_SHARP: usize = 6;
pub const NOTE_E: usize = 7;
pub const NOTE_F: usize = 8;
pub const NOTE_F_SHARP: usize = 9;
pub const NOTE_G: usize = 10;
pub const NOTE_G_SHARP: usize = 11;

// Helper function to convert an image to a buffer of u32 pixels
// Each pixel in the buffer is represented in ARGB format
pub fn img_to_buffer(img: &image::DynamicImage) -> Vec<u32> {
    img.to_rgba8().pixels().map(|p| {
        let channels = p.0;
        ((channels[3] as u32) << 24) // Alpha channel
            | ((channels[0] as u32) << 16) // Red channel
            | ((channels[1] as u32) << 8)  // Green channel
            | (channels[2] as u32)         // Blue channel
    }).collect()
}
