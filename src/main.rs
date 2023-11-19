use console::Term;
use rodio::{OutputStream, Sink};
use crate::state::event_loop::execute_event_loop;
use crate::synths::note::Octave;

mod synths;mod state;

fn main() {
    // Set the initial octave value to 4
    let octave = Octave { value: 4 };

    // Initialize a console terminal for user interaction
    let term = Term::stdout();

    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Execute the main event loop, which handles user input and associated sound generation
    execute_event_loop(octave, term, sink);
}
