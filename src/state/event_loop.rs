use std::time::Duration;

use minifb::{Key as key, KeyRepeat, Window, WindowOptions};
use rodio::{Sink, source::Source};

use crate::{graphics::*, Keyboard, music_theory::{
    note::Note,
    note::Octave,
    OCTAVE_UPPER_BOUND,
    OCTAVE_LOWER_BOUND,
}, waveforms::{
    AMPLITUDE,
    DURATION,
    sine_wave::SineWave,
    square_wave::SquareWave, Waveform,
}};
use crate::graphics::sprite::{draw_sprite, draw_sprites, Sprites};

pub fn start_event_loop(octave: &mut Octave, keyboard: &mut Keyboard, sink: &mut Sink, sprites: &Sprites) {
    let grid_width = WINDOW_WIDTH / TILE_WIDTH;
    let grid_height = WINDOW_HEIGHT / TILE_HEIGHT;

    // Default is Sine wave
    let mut waveform_sprite_index: usize = WAVEFORM_SINE;

    // Create a window
    let mut window = Window::new(
        "Rust Synthesizer 0.25",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut window_buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // As long as the window is open and the escape key is not pressed; subscribe to user input
    while window.is_open() && !window.is_key_down(key::Escape) {
        handle_key_presses(
            &mut window,            // Reference to the window to handle key events
            octave,                 // Current octave
            keyboard,               // Keyboard state
            sink,                   // Audio sink to play sounds
            &sprites,               // Sprites to draw on the screen
            &mut waveform_sprite_index, // Current waveform sprite index
            &mut window_buffer,     // Buffer to hold the window's pixel data
            grid_width,             // Width of the grid
            grid_height             // Height of the grid
        );

        // Update the window with the contents of the window buffer
        window.update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}

fn handle_key_presses(
    window: &mut Window,
    octave: &mut Octave,
    keyboard: &mut Keyboard,
    sink: &mut Sink,
    sprites: &Sprites,
    waveform_image_index: &mut usize,
    window_buffer: &mut Vec<u32>,
    grid_width: usize,
    grid_height: usize,
) {
    let key_mappings = vec![
        (key::Q, Note::C, 0, NOTE_C),
        (key::Key2, Note::CSharp, 0, NOTE_C_SHARP),
        (key::W, Note::D, 1, NOTE_D),
        (key::Key3, Note::DSharp, 1, NOTE_D_SHARP),
        (key::E, Note::E, 2, NOTE_E),
        (key::R, Note::F, 3, NOTE_F),
        (key::Key5, Note::FSharp, 3, NOTE_F_SHARP),
        (key::T, Note::G, 4, NOTE_G),
        (key::Key6, Note::GSharp, 4, NOTE_G_SHARP),
        (key::Y, Note::A, 5, NOTE_A),
        (key::Key7, Note::ASharp, 5, NOTE_A_SHARP),
        (key::U, Note::B, 6, NOTE_B),
    ];

    for (key, note, colored_key_index, note_sprite_index) in key_mappings {
        if window.is_key_pressed(key, KeyRepeat::No) {
            draw_sprites(colored_key_index, note_sprite_index, &sprites, &grid_width, &grid_height, waveform_image_index, octave.value as usize, window_buffer);
            handle_musical_note(octave, sink, keyboard.get_current_waveform(), note);
            return;
        }
    }

    if window.is_key_pressed(key::F, KeyRepeat::No) {
        // Toggle the waveform between SINE and SQUARE when 'F' key is pressed
        keyboard.toggle_waveform();
        // Update the waveform image index based on the current waveform
        *waveform_image_index = if keyboard.get_current_waveform() == Waveform::SQUARE { WAVEFORM_SQUARE } else { WAVEFORM_SINE };

        // Paint background so that the sprite may merge in foreground
        draw_sprite(7 * TILE_WIDTH, 9 * TILE_HEIGHT, &sprites.background[7], window_buffer, WINDOW_WIDTH);

        // Draw waveform
        draw_sprite(7 * TILE_WIDTH, 9 * TILE_HEIGHT, &sprites.waveforms[*waveform_image_index], window_buffer, WINDOW_WIDTH);

        // Wave text
        draw_sprite(8 * TILE_WIDTH, 9 * TILE_HEIGHT, &sprites.text[0], window_buffer, WINDOW_WIDTH);
    }

    if window.is_key_pressed(key::F2, KeyRepeat::No) && octave.value < OCTAVE_UPPER_BOUND {
        // Increase the octave when 'F2' key is pressed and the current octave is below the upper bound
        keyboard.increase_octave();
        // Update the octave value to match the keyboard's current octave
        octave.value = keyboard.get_current_octave();
        // Paint background so that the sprite may merge in foreground
        draw_sprite(9 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.background[16], window_buffer, WINDOW_WIDTH);
        // Draw the new octave sprite at the specified position on top of the background sprite (important as it is fully transparent/alpha)
        draw_sprite(9 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.numbers[octave.value as usize - 1], window_buffer, WINDOW_WIDTH);
    }

    if window.is_key_pressed(key::F1, KeyRepeat::No) && octave.value > OCTAVE_LOWER_BOUND {
        // Decrease the octave when 'F1' key is pressed and the current octave is above the lower bound
        keyboard.decrease_octave();
        // Update the octave value to match the keyboard's current octave
        octave.value = keyboard.get_current_octave();
        // Paint background so that the sprite may merge in foreground
        draw_sprite(9 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.background[16], window_buffer, WINDOW_WIDTH);
        // Draw the new octave sprite at the specified position on top of the background sprite (important as it is fully transparent/alpha)
        draw_sprite(9 * TILE_WIDTH, 0 * TILE_HEIGHT, &sprites.numbers[octave.value as usize - 1], window_buffer, WINDOW_WIDTH);
    }
}
fn handle_musical_note(octave: &mut Octave, sink: &mut Sink, current_waveform: Waveform, note: Note) {

    // Initialize Synth implementation based on Waveform enum
    let synth = match current_waveform {
        Waveform::SQUARE => {
            let square_wave = SquareWave::new(note.frequency(octave));
            Box::new(square_wave) as Box<dyn Source<Item=f32> + 'static + Send>
        }
        _ => {
            let sine_wave = SineWave::new(note.frequency(octave));
            Box::new(sine_wave) as Box<dyn Source<Item=f32> + 'static + Send>
        }
    };

    // Create Source from our Synth
    let source = synth.take_duration(Duration::from_secs_f32(DURATION)).amplify(AMPLITUDE);

    // Append the sound source to the audio sink for playback
    let _result = sink.append(source);
}