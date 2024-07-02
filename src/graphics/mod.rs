use image::GenericImageView;

pub const WINDOW_WIDTH: usize = 640;
pub const WINDOW_HEIGHT: usize = 480;

pub const TILE_WIDTH: usize = 64;
pub const TILE_HEIGHT: usize = 48;

pub const GRAY: usize = 2;
pub const WHITE: usize = 1;
pub const BLUE: usize = 0;

pub const LETTER_A: usize = 0;
pub const LETTER_B: usize = 1;
pub const LETTER_C: usize = 2;
pub const LETTER_D: usize = 3;
pub const LETTER_E: usize = 4;
pub const LETTER_F: usize = 5;
pub const LETTER_G: usize = 6;

// Helper function to convert image to buffer
pub fn img_to_buffer(img: &image::DynamicImage) -> Vec<u32> {
    img.to_rgba8().pixels().map(|p| {
        let channels = p.0;
        ((channels[3] as u32) << 24) // Alpha channel
            | ((channels[0] as u32) << 16) // Red channel
            | ((channels[1] as u32) << 8)  // Green channel
            | (channels[2] as u32)         // BLUE channel
    }).collect()
}

// Function to update a specific tile in the buffer
pub fn update_specific_tile(buffer: &mut [u32], tile_buffer: &[u32], tile_x: usize, tile_y: usize) {
    let x_offset = tile_x * TILE_WIDTH;
    let y_offset = tile_y * TILE_HEIGHT;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let window_idx = (y + y_offset) * WINDOW_WIDTH + (x + x_offset);
            let tile_idx = y * TILE_WIDTH + x;
            buffer[window_idx] = tile_buffer[tile_idx];
        }
    }
}

// Function to fill the entire grid with a specific tile
pub fn fill_grid_with_tile(buffer: &mut [u32], tile_buffer: &[u32], grid_width: usize, grid_height: usize) {
    for y in 0..grid_height {
        for x in 0..grid_width {
            update_specific_tile(buffer, tile_buffer, x, y);
        }
    }
}

pub fn load_tiles() -> Vec<(u32, u32, Vec<u32>)> {
    let tiles: Vec<(u32, u32, Vec<u32>)> = (0..3)
        .map(|i| {
            let img = image::open(format!("assets/tile_{}.png", i)).expect(&format!("Failed to open tile_{}", i));
            let (width, height) = img.dimensions();
            let buffer = img_to_buffer(&img);
            (width, height, buffer)
        })
        .collect();
    tiles
}

pub fn load_letters() -> Vec<(u32, u32, Vec<u32>)> {
    let letters: Vec<(u32, u32, Vec<u32>)> = ["a", "b", "c", "d", "e", "f", "g"]
        .iter()
        .map(|&letter| {
            let img = image::open(format!("assets/letter_{}.png", letter)).expect(&format!("Failed to open letter {}", letter));
            let (width, height) = img.dimensions();
            let buffer = img_to_buffer(&img);
            (width, height, buffer)
        })
        .collect();
    letters
}

pub fn load_waveforms() -> Vec<(u32, u32, Vec<u32>)> {
    let waveforms: Vec<(u32, u32, Vec<u32>)> = ["sine", "square", "saw"]
        .iter()
        .map(|&waveform| {
            let img = image::open(format!("assets/{}.png", waveform)).expect(&format!("Failed to open waveform {}", waveform));
            let (width, height) = img.dimensions();
            let buffer = img_to_buffer(&img);
            (width, height, buffer)
        })
        .collect();
    waveforms
}

pub fn load_octaves() -> Vec<(u32, u32, Vec<u32>)> {
    let numbers: Vec<(u32, u32, Vec<u32>)> = (1..6)
        .map(|i| {
            let img = image::open(format!("assets/{}.png", i)).expect(&format!("Failed to open tile_{}", i));
            let (width, height) = img.dimensions();
            let buffer = img_to_buffer(&img);
            (width, height, buffer)
        })
        .collect();
    numbers
}


pub fn draw_tiles(gray_start: usize, white_end: usize, letter: usize, tiles: &Vec<(u32, u32, Vec<u32>)>, letters: &Vec<(u32, u32, Vec<u32>)>, waveforms: &Vec<(u32, u32, Vec<u32>)>, numbers: &Vec<(u32, u32, Vec<u32>)>, grid_width: &usize, grid_height: &usize, waveform_index: &mut usize, octave_index: usize, mut window_buffer: &mut Vec<u32>) {

    // Paint all tiles in the grid blue
    fill_grid_with_tile(&mut window_buffer, &tiles[BLUE].2, *grid_width, *grid_height);

    // Draw note
    update_specific_tile(&mut window_buffer, &letters[letter].2, 8, 0);

    // Draw octave
    update_specific_tile(&mut window_buffer, &numbers[octave_index].2, 9, 0);

    // Draw waveform
    update_specific_tile(&mut window_buffer, &waveforms[*waveform_index].2, 5, 0);

    // Potentially draw white keys leading up to gray keys
    for i in 0..gray_start {
        update_specific_tile(window_buffer, &tiles[WHITE].2, i, 9);
        update_specific_tile(window_buffer, &tiles[WHITE].2, i, 8);
        update_specific_tile(window_buffer, &tiles[WHITE].2, i, 7);
    }

    // Draw gray keys
    update_specific_tile(&mut window_buffer, &tiles[GRAY].2, gray_start, 9);
    update_specific_tile(&mut window_buffer, &tiles[GRAY].2, gray_start, 8);
    update_specific_tile(&mut window_buffer, &tiles[GRAY].2, gray_start, 7);

    // Potentially draw white keys following gray keys
    for i in gray_start+1..white_end {
        update_specific_tile(&mut window_buffer, &tiles[WHITE].2, i, 9);
        update_specific_tile(&mut window_buffer, &tiles[WHITE].2, i, 8);
        update_specific_tile(&mut window_buffer, &tiles[WHITE].2, i, 7);
    }
}

