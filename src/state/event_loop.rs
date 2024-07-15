use std::thread;
use std::time::{Duration, Instant};
use minifb::{Key as key, KeyRepeat, Window, WindowOptions};
use rodio::{Sink, source::Source};

use crate::{
    graphics::constants::*,
    graphics::sprites::*,
    music_theory::{
        note::Note,
        OCTAVE_LOWER_BOUND,
        OCTAVE_UPPER_BOUND,
    },
    waveforms::{
        AMPLITUDE,
        DURATION,
        sine_wave::SineWave,
        square_wave::SquareWave,
        Waveform,
    },
    state::{FRAME_DURATION, SynthesizerState},
};

/// Starts the event loop for the synthesizer application, handling user input and rendering visuals.
///
/// This function initializes a window, listens for user input, updates visual and audio states
/// based on key presses, and renders the updated visuals to the window.
///
/// # Parameters
/// - `state`: Mutable reference to `SynthesizerState`, which manages the current state of the synthesizer.
/// - `sink`: Mutable reference to `Sink`, the audio sink responsible for playing sound.
/// - `sprites`: Reference to `Sprites`, containing all graphical assets used for rendering visuals.
///
/// # Event Loop Logic
/// - Initializes a window with specific dimensions and title.
/// - Continuously checks for user input and updates synthesizer state accordingly.
/// - Updates the visual representation of the synthesizer based on the current state.
/// - Renders the updated visual buffer onto the window.
/// - Maintains a frame rate of approximately 60 frames per second by calculating necessary sleep time.
pub fn start_event_loop(state: &mut SynthesizerState, sink: &mut Sink, sprites: &Sprites) {
    // Calculate grid dimensions based on tile size
    let grid_width = WINDOW_WIDTH / TILE_WIDTH;
    let grid_height = WINDOW_HEIGHT / TILE_HEIGHT;

    // Create a window with error handling
    let mut window = Window::new(
        "Rust Synthesizer 0.25",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e); // Panic if window creation fails
    });

    // Initialize window buffer to store pixel data
    let mut window_buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while window.is_open() && !window.is_key_down(key::Escape) {
        let start = Instant::now(); // Record start time for frame timing

        // Handle user key presses to update synthesizer state and play sound
        handle_key_presses(state, &mut window, sink);

        // Update the pixel buffer with the current state visuals
        update_buffer_with_state(state, sprites, &mut window_buffer, grid_width, grid_height);

        // Draw the current buffer onto the window
        draw_buffer(&mut window, &mut window_buffer);

        // Maintain a frame rate of approximately 60 fps
        let elapsed = start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }
    }
}

/// Handles key presses for musical notes, waveform toggling, and octave adjustments.
///
/// This function checks for specific key presses and updates the synthesizer state accordingly.
///
/// # Parameters
/// - `state`: Mutable reference to the synthesizer state which holds current octave, waveform, and pressed key.
/// - `window`: Mutable reference to the window object used to detect key presses.
/// - `sink`: Mutable reference to the audio sink where musical notes are played.
///
/// # Key Handling Logic
/// - It iterates over predefined key mappings and triggers musical note generation when a corresponding key is pressed.
/// - Toggles between SINE and SQUARE waveform when the 'F' key is pressed.
/// - Increases the octave when 'F2' key is pressed and the current octave is below the upper bound.
/// - Decreases the octave when 'F1' key is pressed and the current octave is above the lower bound.
///
/// # Notes
/// - The function uses the `handle_musical_note` function to produce sound for the pressed musical note.
/// - State changes (waveform toggle, octave adjustments) directly modify the provided `state` reference.
pub fn handle_key_presses(state: &mut SynthesizerState, window: &mut Window, sink: &mut Sink) {
    // Check for musical note key presses
    for (key, note, _, _) in get_key_mappings() {
        if window.is_key_pressed(key, KeyRepeat::No) {
            handle_musical_note(state.get_current_octave(), sink, state.get_current_waveform(), note);
            state.pressed_key = Some((key, note));
            return;
        }
    }

    // Toggle the waveform between SINE and SQUARE when 'F' key is pressed
    if window.is_key_pressed(key::F, KeyRepeat::No) {
        state.toggle_waveform();
    }

    // Increase the octave when 'F2' key is pressed and the current octave is below the upper bound
    if window.is_key_pressed(key::F2, KeyRepeat::No) && state.get_current_octave() < OCTAVE_UPPER_BOUND {
        state.increase_octave();
    }

    // Decrease the octave when 'F1' key is pressed and the current octave is above the lower bound
    if window.is_key_pressed(key::F1, KeyRepeat::No) && state.get_current_octave() > OCTAVE_LOWER_BOUND {
        state.decrease_octave();
    }
}


/// Handles playing a musical note with a specified octave, waveform, and duration.
///
/// This function initializes a synthesizer based on the provided waveform and plays the corresponding musical note
/// with the given octave, using a specified audio sink for playback.
///
/// # Parameters
/// - `octave`: A mutable reference to the current octave of the synthesizer.
/// - `sink`: A mutable reference to the audio sink where the sound will be played.
/// - `current_waveform`: The waveform enum representing the type of waveform to use for synthesizing the sound.
/// - `note`: The musical note (pitch) to be played.
///
/// # Example
/// ```rust
/// use std::time::Duration;
/// use rodio::{Sink, Source};
/// use your_module::{handle_musical_note, Octave, Waveform, Note};
///
/// let mut octave = Octave::default(); // Initialize octave
/// let mut sink = Sink::new(); // Create audio sink
/// let current_waveform = Waveform::SQUARE; // Choose waveform
/// let note = Note::C; // Choose note
///
/// // Play the note with the given parameters
/// handle_musical_note(&mut octave, &mut sink, current_waveform, note);
/// ```
fn handle_musical_note(octave: i32, sink: &mut Sink, current_waveform: Waveform, note: Note) {

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

/// Draws the current state of the synthesizer on the window buffer.
///
/// This function first paints the background, then draws all idle keys and tangents.
/// If a key is pressed (`state.pressed_key` is Some), it draws the corresponding note,
/// octave, waveform, text, and potentially the pressed key sprite.
///
/// # Parameters
/// - `state`: Reference to the current `SynthesizerState` containing the state of the synthesizer.
/// - `sprites`: Reference to the `Sprites` struct containing all sprite data needed for drawing.
/// - `window_buffer`: Mutable reference to the window buffer where pixels are drawn.
/// - `grid_width`: Width of the grid in tiles.
/// - `grid_height`: Height of the grid in tiles.
fn update_buffer_with_state(state: &SynthesizerState, sprites: &Sprites, window_buffer: &mut Vec<u32>, grid_width: usize, grid_height: usize) {
    // Draw background
    fill_background(window_buffer, &sprites.background, grid_width, grid_height, WINDOW_WIDTH);

    // Draw all idle keys first
    draw_idle_keys(sprites, window_buffer);

    // Create a map for tangent positions and their corresponding note constants
    let tangent_map = create_tangent_map();

    // Draw all tangents in their idle state first
    draw_idle_tangents(sprites, window_buffer, &tangent_map);

    // Check if a key is pressed
    if let Some((_, note)) = &state.pressed_key {
        // Get sprite index associated with the note to be drawn (A, C# etc.)
        let note_sprite_index = get_note_sprite_index(note).unwrap_or_default();

        // Get key position on the keyboard (0 would be the first key, 7 the last etc.)
        let key_position = get_key_position(note).unwrap_or(0);

        // Draw sprites
        draw_note_sprite(sprites, window_buffer, note_sprite_index);
        draw_current_octave_sprite(state, sprites, window_buffer);
        draw_current_waveform_sprite(state, sprites, window_buffer);
        draw_text_sprite(sprites, window_buffer);

        // Draw pressed key sprite if the note is not a sharp
        if matches!(note, Note::A | Note::B | Note::C | Note::D | Note::E | Note::F | Note::G) {
            draw_pressed_key_sprite(sprites, window_buffer, key_position);
        }

        // Draw idle and pressed tangents
        draw_tangents(note_sprite_index, &tangent_map, sprites, window_buffer);
    }
}