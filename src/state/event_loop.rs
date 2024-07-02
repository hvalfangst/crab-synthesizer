use std::thread;
use std::time::Duration;
use console::{Key, Term};
use minifb::{Key as key2, KeyRepeat, Window, WindowOptions};
use image::GenericImageView;
use rodio::{Sink, source::Source};
use crate::{music_theory::{
    note::Note,
    note::Octave
}, waveforms::{
    Waveform,
    sine_wave::SineWave,
    square_wave::SquareWave,
    DURATION, AMPLITUDE
}, graphics::*, Keyboard};
use crate::music_theory::{OCTAVE_LOWER_BOUND, OCTAVE_UPPER_BOUND};

pub fn execute_event_loop(octave: &mut Octave, keyboard: &mut Keyboard, sink: &mut Sink) {

    // Load tile images into buffer
    let tiles = load_tiles();

    // Load letter images into buffer
    let letters = load_letters();

    // Load waveform images into buffer
    let waveforms = load_waveforms();

    // Load octave images into buffer
    let numbers = load_octaves();

    // Set grid parameters
    let grid_width = WINDOW_WIDTH / TILE_WIDTH;  // 10 tiles
    let grid_height = WINDOW_HEIGHT / TILE_HEIGHT; // 10 tiles

    // Used to keep track of which waveform image to display
    let mut waveform_image_index: usize = 0;

    // Create a window
    let mut window = Window::new(
        "Rust Synthesizer 0.1",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Create a buffer for the entire window
    let mut window_buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // Paint all tiles in the grid blue
    fill_grid_with_tile(&mut window_buffer, &tiles[BLUE].2, grid_width, grid_height);

    // Display the grid and update specific tiles based on key press
    while window.is_open() && !window.is_key_down(key2::Escape) {
        if window.is_key_pressed(key2::Q, KeyRepeat::No) {
            draw_tiles(0, 7, LETTER_A, &tiles, &letters, &waveforms, &numbers, &grid_width, &grid_height, &mut waveform_image_index,  usize::try_from(octave.value - 1).unwrap(), &mut window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(), Note::A);
        } else if window.is_key_pressed(key2::W, KeyRepeat::No) {
            draw_tiles(1, 7, LETTER_B, &tiles, &letters, &waveforms, &numbers, &grid_width, &grid_height, &mut waveform_image_index,  usize::try_from(octave.value - 1).unwrap(), &mut window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(),  Note::B);
        } else if window.is_key_pressed(key2::E, KeyRepeat::No) {
            draw_tiles(2, 7, LETTER_C, &tiles, &letters, &waveforms, &numbers, &grid_width, &grid_height, &mut waveform_image_index,  usize::try_from(octave.value - 1).unwrap(), &mut window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(), Note::C);
        } else if window.is_key_pressed(key2::R, KeyRepeat::No) {
            draw_tiles(3, 7, LETTER_D, &tiles, &letters, &waveforms, &numbers, &grid_width, &grid_height, &mut waveform_image_index,  usize::try_from(octave.value - 1).unwrap(), &mut window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(), Note::D);
        } else if window.is_key_pressed(key2::T, KeyRepeat::No) {
            draw_tiles(4, 7, LETTER_E, &tiles, &letters, &waveforms, &numbers, &grid_width, &grid_height, &mut waveform_image_index,  usize::try_from(octave.value - 1).unwrap(), &mut window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(),  Note::E);
        } else if window.is_key_pressed(key2::Y, KeyRepeat::No) {
            draw_tiles(5, 7, LETTER_F, &tiles, &letters, &waveforms, &numbers, &grid_width, &grid_height, &mut waveform_image_index,  usize::try_from(octave.value - 1).unwrap(), &mut window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(),  Note::F);
        } else if window.is_key_pressed(key2::U, KeyRepeat::No) {
            draw_tiles(6, 7, LETTER_G, &tiles, &letters, &waveforms, &numbers, &grid_width, &grid_height, &mut waveform_image_index,  usize::try_from(octave.value - 1).unwrap(), &mut window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(),  Note::G);
        } else if window.is_key_pressed(key2::F, KeyRepeat::No) {
            // Modify current active waveform and its associated image index
            keyboard.toggle_waveform();
            let waveform = keyboard.get_current_waveform();
            waveform_image_index = if waveform == Waveform::SQUARE { 1 } else { 0 };

            // Paint the new waveform tile
            update_specific_tile(&mut window_buffer, &waveforms[waveform_image_index].2, 5, 0);
        } else if window.is_key_pressed(key2::F2, KeyRepeat::No) {
            if octave.value < OCTAVE_UPPER_BOUND {
                handle_increase_octave(octave, keyboard);
                update_specific_tile(&mut window_buffer,  &numbers[usize::try_from(octave.value - 1).unwrap()].2, 9, 0);
            }
        } else if window.is_key_pressed(key2::F1, KeyRepeat::No) {
            if octave.value >  OCTAVE_LOWER_BOUND {
                handle_decrease_octave(octave, keyboard);
                update_specific_tile(&mut window_buffer,  &numbers[usize::try_from(octave.value - 1).unwrap()].2, 9, 0);
            }
        }
            window.update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
        }
    }

    fn handle_increase_octave(octave: &mut Octave, keyboard: &mut Keyboard) {
            let new_octave = &octave.value + 1;
            octave.value = new_octave;
            keyboard.set_current_octave(&octave.value);
    }

    fn handle_decrease_octave(octave: &mut Octave, keyboard: &mut Keyboard) {
            let new_octave = &octave.value - 1;
            octave.value = new_octave;
            keyboard.set_current_octave(&octave.value);
    }

    fn handle_musical_note(octave: &mut Octave, sink: &mut Sink, current_waveform: Waveform, note: Note) {

        // Initialize Synth implementation based on current Waveform
        let synth = match current_waveform {
            Waveform::SQUARE => {
                let mut square_wave = SquareWave::new(note.frequency(octave));
                Box::new(square_wave) as Box<dyn Source<Item=f32> + 'static + Send>
            }
            _ => {
                let mut sine_wave = SineWave::new(note.frequency(octave));
                Box::new(sine_wave) as Box<dyn Source<Item=f32> + 'static + Send>
            }
        };

        // Create Source from our Synth
        let source = synth.take_duration(Duration::from_secs_f32(DURATION)).amplify(AMPLITUDE);

        // Append the sound source to the audio sink for playback
        let _result = sink.append(source);
    }