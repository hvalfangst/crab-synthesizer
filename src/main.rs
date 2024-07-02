use console::Term;
use rodio::{OutputStream, Sink};
use crate::{
    music_theory::note::Octave,
    state::{
        event_loop::execute_event_loop,
        keyboard::Keyboard
    },
};

mod waveforms;mod state;mod music_theory;mod graphics;

fn main() {
    // Set the initial octave value to 4
    let mut octave = Octave { value: 4 };

    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();

    // Draw the keyboard
    let mut keyboard = Keyboard::new();

    // Execute the main event loop, which handles user input and associated sound generation
    execute_event_loop(&mut octave, &mut keyboard, &mut sink);
}