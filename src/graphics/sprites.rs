use std::collections::HashMap;

use image::GenericImageView;
use minifb::{Key, Window};

use crate::{
    graphics::constants::*,
    music_theory::note::Note,
    state::SynthesizerState,
};

pub struct Sprites {
    pub notes: Vec<(u32, u32, Vec<u32>)>,
    pub waveforms: Vec<(u32, u32, Vec<u32>)>,
    pub background: Vec<(u32, u32, Vec<u32>)>,
    pub numbers: Vec<(u32, u32, Vec<u32>)>,
    pub keys: Vec<(u32, u32, Vec<u32>)>,
    pub tangents: Vec<(u32, u32, Vec<u32>)>,
    pub text: Vec<(u32, u32, Vec<u32>)>
}

impl Sprites {
    pub fn new() -> Self {
        Self {
            notes: load_sprites_from_map("assets/notes.png", 64, 48),
            waveforms: load_sprites_from_map("assets/waveforms.png", 55, 60),
            background: load_sprites_from_map("assets/background.png", 64, 48),
            numbers: load_sprites_from_map("assets/numbers.png", 64, 48),
            keys: load_sprites_from_map("assets/keys.png", 64, 144),
            tangents: load_sprites_from_map("assets/tangents.png", 30, 96),
            text: load_sprites_from_map("assets/text.png", 120, 60)

        }
    }
}

/// Converts an image to a buffer of u32 pixels in ARGB format.
///
/// Each pixel in the buffer is represented as ARGB (Alpha, Red, Green, Blue).
///
/// # Parameters
/// - `img`: A reference to the `DynamicImage` to be converted.
///
/// # Returns
/// A vector of u32 pixels representing the image in ARGB format.
pub fn img_to_buffer(img: &image::DynamicImage) -> Vec<u32> {
    img.to_rgba8().pixels().map(|p| {
        let channels = p.0;
        ((channels[3] as u32) << 24) // Alpha channel
            | ((channels[0] as u32) << 16) // Red channel
            | ((channels[1] as u32) << 8)  // Green channel
            | (channels[2] as u32)         // Blue channel
    }).collect()
}

/// Fills the background buffer with sprites from a sprite map.
///
/// This function draws sprites onto the background buffer in a grid pattern,
/// using sprites provided in `sprite_map`. The grid dimensions are specified
/// by `grid_width` and `grid_height`, and the width of the window buffer is `window_width`.
///
/// # Parameters
/// - `buffer`: A mutable slice of u32 pixels representing the window buffer.
/// - `sprite_map`: A vector containing tuples of sprite dimensions and pixel data.
/// - `grid_width`: The number of sprites to be drawn horizontally.
/// - `grid_height`: The number of sprites to be drawn vertically.
/// - `window_width`: The width of the window buffer in pixels.
pub fn fill_background(buffer: &mut [u32], sprite_map: &Vec<(u32, u32, Vec<u32>)>, grid_width: usize, grid_height: usize, window_width: usize) {
    let mut counter = sprite_map.len()-1;

    for y in 0..grid_height {
        for x in 0..grid_width {
            draw_sprite(x * sprite_map[counter].0 as usize, y * sprite_map[counter].1 as usize, &sprite_map[counter], buffer, window_width);
        }
        counter -= 1;
    }
}

/// Loads sprites from a sprite map image file into memory.
///
/// This function opens the image file specified by `sprite_map_path`, extracts individual
/// sprites based on `sprite_width` and `sprite_height`, and stores each sprite in a buffer.
///
/// # Parameters
/// - `sprite_map_path`: A string slice containing the path to the sprite map image file.
/// - `sprite_width`: The width of each individual sprite in pixels.
/// - `sprite_height`: The height of each individual sprite in pixels.
///
/// # Returns
/// A vector containing tuples of sprite dimensions and pixel data.
pub fn load_sprites_from_map(sprite_map_path: &str, sprite_width: u32, sprite_height: u32) -> Vec<(u32, u32, Vec<u32>)> {
    // Load the sprite map image
    let sprite_map = image::open(sprite_map_path).expect(&format!("Failed to open sprite map at {}", sprite_map_path));
    let (map_width, map_height) = sprite_map.dimensions();

    println!("Sprite map loaded from {}", sprite_map_path);
    println!("Sprite map dimensions: {}x{}", map_width, map_height);

    // Calculate the number of sprites in each dimension
    let sprites_x = map_width / sprite_width;
    let sprites_y = map_height / sprite_height;

    println!("Sprites x: {}", sprites_x);
    println!("Sprites y: {}", sprites_y);

    // Extract individual sprites and store them in a buffer
    let mut sprites = Vec::new();
    for y in 0..sprites_y {
        for x in 0..sprites_x {
            println!("Extracting sprite at ({}, {})", x, y);
            let sprite = sprite_map.crop_imm(x * sprite_width, y * sprite_height, sprite_width, sprite_height);
            let buffer = img_to_buffer(&sprite);
            println!("Sprite extracted: {}x{}, buffer length: {}", sprite_width, sprite_height, buffer.len());
            sprites.push((sprite_width, sprite_height, buffer));
        }
    }

    println!("Total sprites extracted: {}\n", sprites.len());

    // Return the vector of sprites
    sprites
}

/// Draws a sprite onto the window buffer at the specified coordinates, with alpha blending.
///
/// # Parameters
/// - `x`: The x-coordinate where the sprite will be drawn.
/// - `y`: The y-coordinate where the sprite will be drawn.
/// - `sprite`: A tuple containing the sprite's width, height, and pixel data. The pixel data is a vector of `u32` values representing RGBA colors.
/// - `window_buffer`: A mutable slice of `u32` representing the pixels of the window buffer. Each `u32` value represents an RGBA color.
/// - `window_width`: The width of the window in pixels.
///
/// This function uses alpha blending to combine the sprite's pixels with the corresponding pixels in the window buffer. Only non-transparent pixels in the sprite are drawn.
///
/// # Alpha Blending
/// Alpha blending is a process used in computer graphics to combine a foreground image with a background image, resulting in a composite image.
/// The alpha value determines the transparency level of the pixel:
/// - An alpha value of 0 means the pixel is completely transparent.
/// - An alpha value of 255 (0xFF) means the pixel is completely opaque.
///
/// The formula for alpha blending is:
/// ```
/// blended_color = (foreground_color * alpha + background_color * (255 - alpha)) / 255
/// ```
///
/// # ARGB Color Palette
/// Each `u32` value in the pixel data represents a color in ARGB format:
/// - The highest 8 bits represent the alpha (transparency) channel.
/// - The next 8 bits represent the red channel.
/// - The next 8 bits represent the green channel.
/// - The lowest 8 bits represent the blue channel.
///
/// For example, a color represented as `0x80FF00FF` means:
/// - Alpha: 0x80 (128 in decimal, semi-transparent)
/// - Red: 0xFF (255 in decimal, full intensity)
/// - Green: 0x00 (0 in decimal, no intensity)
/// - Blue: 0xFF (255 in decimal, full intensity)
///
/// # Example Usage
/// ```rust
/// let sprite = (16, 16, vec![0x80FF00FF; 256]); // A semi-transparent magenta 16x16 sprite
/// let mut window_buffer = vec![0xFFFFFFFF; 800 * 600]; // A white 800x600 window buffer
/// draw_sprite(10, 10, &sprite, &mut window_buffer, 800);
/// ```
pub fn draw_sprite(x: usize, y: usize, sprite: &(u32, u32, Vec<u32>), window_buffer: &mut [u32], window_width: usize) {
    let (sprite_width, sprite_height, sprite_data) = sprite;

    for row in 0..*sprite_height as usize {
        for col in 0..*sprite_width as usize {
            let sprite_pixel_index = row * (*sprite_width as usize) + col;
            let window_pixel_index = (y + row) * window_width + (x + col);

            if window_pixel_index < window_buffer.len() {
                let sprite_pixel = sprite_data[sprite_pixel_index];
                let sprite_alpha = (sprite_pixel >> 24) & 0xFF; // Extract alpha channel from sprite pixel
                let sprite_rgb = sprite_pixel & 0x00FFFFFF; // Extract RGB channels from sprite pixel

                if sprite_alpha > 0 { // Only blend if the pixel is not fully transparent
                    let window_pixel = window_buffer[window_pixel_index];
                    let window_rgb = window_pixel & 0x00FFFFFF; // Extract RGB channels from window buffer pixel

                    // Calculate blended color using alpha blending formula
                    let blended_r = ((sprite_rgb >> 16) & 0xFF) * sprite_alpha / 255 + ((window_rgb >> 16) & 0xFF) * (255 - sprite_alpha) / 255;
                    let blended_g = ((sprite_rgb >> 8) & 0xFF) * sprite_alpha / 255 + ((window_rgb >> 8) & 0xFF) * (255 - sprite_alpha) / 255;
                    let blended_b = (sprite_rgb & 0xFF) * sprite_alpha / 255 + (window_rgb & 0xFF) * (255 - sprite_alpha) / 255;

                    // Combine blended color with full alpha
                    let blended_pixel = 0xFF000000 | (blended_r & 0xFF) << 16 | (blended_g & 0xFF) << 8 | (blended_b & 0xFF);

                    // Assign the blended pixel to the window buffer
                    window_buffer[window_pixel_index] = blended_pixel;
                }
            }
        }
    }
}

/// Returns a vector of tuples representing key mappings.
///
/// Each tuple contains the following elements:
/// - `Key`: The key that is pressed.
/// - `Note`: The musical note associated with the key.
/// - `usize`: The position of the key on the keyboard.
/// - `usize`: The sprite index for the note.
///
/// The function maps keys on the keyboard to musical notes and provides
/// information on their positions and corresponding sprite indices.
pub fn get_key_mappings() -> Vec<(Key, Note, usize, usize)> {
    vec![
        (Key::Q, Note::C, 0, NOTE_C),
        (Key::Key2, Note::CSharp, 0, NOTE_C_SHARP),
        (Key::W, Note::D, 1, NOTE_D),
        (Key::Key3, Note::DSharp, 1, NOTE_D_SHARP),
        (Key::E, Note::E, 2, NOTE_E),
        (Key::R, Note::F, 3, NOTE_F),
        (Key::Key5, Note::FSharp, 3, NOTE_F_SHARP),
        (Key::T, Note::G, 4, NOTE_G),
        (Key::Key6, Note::GSharp, 4, NOTE_G_SHARP),
        (Key::Y, Note::A, 5, NOTE_A),
        (Key::Key7, Note::ASharp, 5, NOTE_A_SHARP),
        (Key::U, Note::B, 6, NOTE_B),
    ]
}

/// Returns the position of the given musical note on the keyboard.
///
/// # Arguments
///
/// * `note` - A reference to the `Note` whose position is to be found.
///
/// # Returns
///
/// * `Some(usize)` - The position of the note on the keyboard if it exists.
/// * `None` - If the note is not found in the key mappings.
///
/// This function iterates through the key mappings and returns the position
/// associated with the given note.
pub fn get_key_position(note: &Note) -> Option<usize> {
    for (_, mapped_note, position, _) in get_key_mappings() {
        if mapped_note == *note {
            return Some(position);
        }
    }
    None
}

/// Returns the sprite index for the given musical note.
///
/// # Arguments
///
/// * `note` - A reference to the `Note` whose sprite index is to be found.
///
/// # Returns
///
/// * `Some(usize)` - The sprite index for the note if it exists.
/// * `None` - If the note is not found in the key mappings.
///
/// This function iterates through the key mappings and returns the sprite index
/// associated with the given note.
pub fn get_note_sprite_index(note: &Note) -> Option<usize> {
    for (_, mapped_note, _, sprite_index) in get_key_mappings() {
        if mapped_note == *note {
            return Some(sprite_index);
        }
    }
    None
}

/// Creates a map for tangent positions and their corresponding note sprite indices.
///
/// # Returns
/// A `HashMap` where the keys are positions on the keyboard and the values are note sprite indices
/// for the corresponding tangent (sharp) keys.
pub fn create_tangent_map() -> HashMap<i32, usize> {
    let tangent_map: HashMap<i32, usize> = [
        (1, NOTE_C_SHARP),   // Between keys C and D
        (2, NOTE_D_SHARP),   // Between keys D and E
        (4, NOTE_F_SHARP),   // Between keys F and G
        (5, NOTE_G_SHARP),   // Between keys G and A
        (6, NOTE_A_SHARP),   // Between keys A and B
    ].iter().cloned().collect();
    tangent_map
}

/// Draws the sprite for a pressed key at the specified position on the virtual keyboard.
///
/// # Parameters
/// - `sprites`: A reference to the `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
/// - `key_position`: The position of the key on the keyboard.
///
/// This function draws the pressed key sprite at the specified key position.
pub fn draw_pressed_key_sprite(sprites: &Sprites, window_buffer: &mut Vec<u32>, key_position: usize) {
    draw_sprite(key_position * TILE_WIDTH, 7 * TILE_HEIGHT, &sprites.keys[KEY_PRESSED], window_buffer, WINDOW_WIDTH);
}

/// Draws the text sprite on the virtual keyboard.
///
/// # Parameters
/// - `sprites`: A reference to the `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
///
/// This function draws the text sprite at a fixed position on the virtual keyboard.
pub fn draw_text_sprite(sprites: &Sprites, window_buffer: &mut Vec<u32>) {
    draw_sprite(8 * TILE_WIDTH, 9 * TILE_HEIGHT, &sprites.text[0], window_buffer, WINDOW_WIDTH);
}

/// Draws the current window with the provided pixel buffer.
///
/// This function updates the window with the contents of the pixel buffer,
/// presenting the current visual state of the synthesizer.
///
/// # Parameters
/// - `window`: Mutable reference to the `Window` object where the visuals are displayed.
/// - `window_buffer`: Mutable reference to a vector of `u32` representing the pixel data to be displayed.
///
/// # Panics
/// This function panics if updating the window with the buffer fails. This usually happens if there's an issue
/// with the window backend or if the buffer dimensions do not match the window dimensions.
///
/// # Example
/// ```rust
/// # use minifb::Window;
/// # const WINDOW_WIDTH: usize = 800;
/// # const WINDOW_HEIGHT: usize = 600;
/// # let mut window = Window::new("Synthesizer", WINDOW_WIDTH, WINDOW_HEIGHT, Default::default()).unwrap();
/// # let mut window_buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
/// draw_state(&mut window, &mut window_buffer);
/// ```
pub fn draw_buffer(window: &mut Window, window_buffer: &mut Vec<u32>) {
    window.update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
}

/// Draws the current waveform sprite based on the synthesizer's state.
///
/// # Parameters
/// - `state`: A reference to the `SynthesizerState` struct representing the current state of the synthesizer.
/// - `sprites`: A reference to the `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
///
/// This function draws the waveform sprite corresponding to the current waveform index in the synthesizer's state.
pub fn draw_current_waveform_sprite(state: &SynthesizerState, sprites: &Sprites, window_buffer: &mut Vec<u32>) {
    draw_sprite(7 * TILE_WIDTH, 9 * TILE_HEIGHT, &sprites.waveforms[state.get_current_waveform_sprite_index()], window_buffer, WINDOW_WIDTH);
}

/// Draws the current octave sprite based on the synthesizer's state.
///
/// # Parameters
/// - `state`: A reference to the `SynthesizerState` struct representing the current state of the synthesizer.
/// - `sprites`: A reference to the `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
///
/// This function draws the octave sprite corresponding to the current octave value in the synthesizer's state.
pub fn draw_current_octave_sprite(state: &SynthesizerState, sprites: &Sprites, window_buffer: &mut Vec<u32>) {
    draw_sprite(9 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.numbers[state.get_current_octave() as usize -1], window_buffer, WINDOW_WIDTH);
}

/// Draws the note sprite for the given note sprite index.
///
/// # Parameters
/// - `sprites`: A reference to the `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
/// - `note_sprite_index`: The index of the note sprite to be drawn.
///
/// This function draws the note sprite at a fixed position on the virtual keyboard.
pub fn draw_note_sprite(sprites: &Sprites, window_buffer: &mut Vec<u32>, note_sprite_index: usize) {
    draw_sprite(8 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.notes[note_sprite_index], window_buffer, WINDOW_WIDTH);
}

/// Draws all idle tangents (sharp keys) on the virtual keyboard.
///
/// # Parameters
/// - `sprites`: A reference to the `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
/// - `tangent_map`: A hashmap mapping positions to the corresponding tangent note sprite indices.
///
/// This function iterates over the `tangent_map` keys and draws the idle tangent sprite
/// at each position. The x-coordinate for each tangent is calculated based on its position
/// and the dimensions of the tangents and keys.
pub fn draw_idle_tangents(sprites: &Sprites, window_buffer: &mut Vec<u32>, tangent_map: &HashMap<i32, usize>) {
    for &pos in tangent_map.keys() {
        let x = pos * (TILE_WIDTH as i32) - (TANGENT_WIDTH as i32 / 2);
        draw_sprite(usize::try_from(x).unwrap(), 7 * TILE_HEIGHT, &sprites.tangents[TANGENT_IDLE], window_buffer, WINDOW_WIDTH);
    }
}

/// Draws all idle keys on the virtual keyboard.
///
/// # Parameters
/// - `sprites`: A reference to the `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
///
/// This function iterates over the range of key positions (0 to 6) and draws the idle key sprite
/// at each position.
pub fn draw_idle_keys(sprites: &Sprites, window_buffer: &mut Vec<u32>) {
    for i in 0..7 {
        draw_sprite(i * TILE_WIDTH, 7 * TILE_HEIGHT, &sprites.keys[KEY_IDLE], window_buffer, WINDOW_WIDTH);
    }
}

/// Draws the tangents (sharp keys) on the virtual keyboard.
///
/// # Parameters
/// - `note_sprite_index`: The index of the sprite representing the current note being pressed.
/// - `tangent_map`: A hashmap mapping positions to the corresponding tangent note sprite indices.
/// - `sprites`: The `Sprites` struct containing all the sprite images.
/// - `window_buffer`: A mutable reference to the buffer representing the window's pixels.
///
/// This function iterates over the `tangent_map` to determine which tangent sprites to draw. If the
/// `note_sprite_index` matches the tangent, it draws the pressed tangent sprite; otherwise, it draws
/// the idle tangent sprite. The x-coordinate for each tangent is calculated based on its position
/// and the dimensions of the tangents and keys.
pub fn draw_tangents(note_sprite_index: usize, tangent_map: &HashMap<i32, usize>, sprites: &Sprites, window_buffer: &mut Vec<u32>) {
    for (&pos, &tangent) in tangent_map {
        let tangent_sprite_index = if note_sprite_index == tangent {
            TANGENT_PRESSED
        } else {
            TANGENT_IDLE
        };

        // Calculate the x-coordinate of the tangent's center position.
        let x = pos * TILE_WIDTH as i32 - TANGENT_WIDTH as i32 / 2;

        draw_sprite(usize::try_from(x).unwrap(), 7 * TILE_HEIGHT, &sprites.tangents[tangent_sprite_index], window_buffer, WINDOW_WIDTH);
    }
}


