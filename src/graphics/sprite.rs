use std::collections::HashMap;

use image::GenericImageView;

use crate::graphics::{img_to_buffer, KEY_IDLE, KEY_PRESSED, NOTE_A_SHARP, NOTE_C_SHARP, NOTE_D_SHARP, NOTE_F_SHARP, NOTE_G_SHARP, TANGENT_IDLE, TANGENT_PRESSED, TANGENT_WIDTH, TILE_HEIGHT, TILE_WIDTH, WINDOW_WIDTH};

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

pub fn fill_background(buffer: &mut [u32], sprite_map: &Vec<(u32, u32, Vec<u32>)>, grid_width: usize, grid_height: usize, window_width: usize) {

    let mut counter = sprite_map.len()-1;

    for y in 0..grid_height {
        for x in 0..grid_width {
            draw_sprite(x * sprite_map[counter].0 as usize, y * sprite_map[counter].1 as usize, &sprite_map[counter], buffer, window_width);
        }
        counter -= 1;
    }

}

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

    // Return said buffer
    sprites
}

pub fn draw_sprites(
    pressed_key_position: usize,
    note_sprite_index: usize,
    sprites: &Sprites,
    grid_width: &usize,
    grid_height: &usize,
    waveform_index: &mut usize,
    octave_index: usize,
    mut window_buffer: &mut Vec<u32>
) {

    // Paint background
    fill_background(&mut window_buffer, &sprites.background, *grid_width, *grid_height, WINDOW_WIDTH);

    // Draw note
    draw_sprite(8 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.notes[note_sprite_index], &mut window_buffer, WINDOW_WIDTH);

    // Draw octave
    draw_sprite(9 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.numbers[octave_index], &mut window_buffer, WINDOW_WIDTH);

    // Draw waveform
    draw_sprite(7 * TILE_WIDTH, 9 * TILE_HEIGHT, &sprites.waveforms[*waveform_index], &mut window_buffer, WINDOW_WIDTH);

    // Wave text
    draw_sprite(8 * TILE_WIDTH, 9 * TILE_HEIGHT, &sprites.text[0], &mut window_buffer, WINDOW_WIDTH);

    // Potentially draw idle white keys followed by the pressed colored key
    for i in 0..pressed_key_position {
        draw_sprite(i * TILE_WIDTH, 7 * TILE_HEIGHT, &sprites.keys[KEY_IDLE], &mut window_buffer, WINDOW_WIDTH);
    }

    // Sharp keys are associated with tangent press and as thus there will be no ordinary key press drawn
    match note_sprite_index {
        NOTE_C_SHARP | NOTE_D_SHARP | NOTE_F_SHARP | NOTE_G_SHARP | NOTE_A_SHARP =>
            {
                draw_sprite(pressed_key_position * TILE_WIDTH, 7 * TILE_HEIGHT, &sprites.keys[KEY_IDLE], &mut window_buffer, WINDOW_WIDTH);
            }
        _ => {
            draw_sprite(pressed_key_position * TILE_WIDTH, 7 * TILE_HEIGHT, &sprites.keys[KEY_PRESSED], &mut window_buffer, WINDOW_WIDTH);
        }
    }

    // Potentially draw idle white keys following the pressed colored key
    for i in (pressed_key_position + 1)..7 {
        draw_sprite(i * TILE_WIDTH, 7 * TILE_HEIGHT, &sprites.keys[0], &mut window_buffer, WINDOW_WIDTH);
    }

    // Create a map for tangent positions and their corresponding letter constants
    let tangent_map: HashMap<i32, usize> = [
        (1, NOTE_C_SHARP),   // Between keys C and D
        (2, NOTE_D_SHARP),  // Between keys D and E
        (4, NOTE_F_SHARP),  // Between keys F and G
        (5, NOTE_G_SHARP), // Between keys G and A
        (6, NOTE_A_SHARP),  // Between keys A and B
    ].iter().cloned().collect();

    // Unpressed/pressed tangent key
    let mut tangent_sprite_index = TANGENT_IDLE;

    for &pos in tangent_map.keys() { // Draw tangents according to the indices specified in above map
        if let Some(&tangent) = tangent_map.get(&pos) {

            if note_sprite_index == tangent { // Current note is a sharp (ie A#) and therefore a tangent should be drawn
                tangent_sprite_index = TANGENT_PRESSED;
            } else {
                tangent_sprite_index = TANGENT_IDLE;
            }

            // Calculate the x-coordinate of the tangent's center position.
            // The tangent is centered between the current key and the next key.
            // Therefore, we start with the position of the current key (`pos * TILE_WIDTH`)
            // and adjust by subtracting half of the tangent's width (`TANGENT_WIDTH / 2`)
            // to center it precisely between the two keys.
            let x = pos * (TILE_WIDTH as i32) - (TANGENT_WIDTH as i32 / 2i32);

            draw_sprite(usize::try_from(x).unwrap(), 7 * TILE_HEIGHT, &sprites.tangents[tangent_sprite_index], &mut window_buffer, WINDOW_WIDTH);

            // Reset index to the default tangent sprite index
            tangent_sprite_index = TANGENT_IDLE;
        }
    }
}

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


