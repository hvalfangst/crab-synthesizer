use console::Term;
use rodio::{OutputStream, Sink};
use crate::synths::wave_table::{populate_wave_table, Octave, WavetableOscillator, Note};
use crate::state::event_loop::execute_event_loop;
mod synths;mod state;

fn main() {
    // Size of the wavetable used by the WavetableOscillator
    let wave_table_size = 64;

    // Create an empty vector to store waveform samples
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    // Populate the wavetable with samples
    populate_wave_table(wave_table_size.clone(), &mut wave_table);

    // Set the initial octave value to 4
    let octave = Octave { value: 4 };

    // Create a WavetableOscillator with a sample rate of 44100 hz and our populated wavetable
    let mut oscillator = WavetableOscillator::new(44100, wave_table);

    // Set the initial frequency of the oscillator to the A note in octave 4 (440 hz)
    oscillator.set_frequency(Note::A.frequency(octave));

    // Set the initial filter parameters for the low-pass filter
    oscillator.set_filter_params(0.1, 0.1);

    // Initialize a console terminal for user interaction
    let term = Term::stdout();

    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Execute the main event loop, which handles user input and associated sound generation
    execute_event_loop(octave, &mut oscillator, term, sink);
}
