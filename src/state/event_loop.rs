use std::thread;
use std::time::Duration;
use console::{Key, Term};
use rodio::{Sink, source::Source};
use crate::synths::wave_table::{Octave, WavetableOscillator, Note};

/// Executes the main event loop, which handles user input and sound generation.
///
/// # Arguments
///
/// * `octave` - The current octave.
/// * `oscillator` - The wavetable oscillator responsible for generating audio samples.
/// * `term` - The console terminal for user input.
/// * `sink` - The audio sink for playback.
pub fn execute_event_loop(mut octave: Octave, oscillator: &mut WavetableOscillator, term: Term, sink: Sink) {
    loop {
        // Read a key from the terminal
        let key = term.read_key().unwrap();

        // Match the pressed key to musical notes and perform actions accordingly
        match key {
            Key::Char('q') | Key::Char('Q')
            | Key::Char('w') | Key::Char('W')
            | Key::Char('e') | Key::Char('E')
            | Key::Char('r') | Key::Char('R')
            | Key::Char('t') | Key::Char('T')
            | Key::Char('y') | Key::Char('Y')
            | Key::Char('u') | Key::Char('U') => {
                let note = match key {
                    // Map keys to musical notes
                    Key::Char('q') | Key::Char('Q') => Note::A,
                    Key::Char('w') | Key::Char('W') => Note::B,
                    Key::Char('e') | Key::Char('E') => Note::C,
                    Key::Char('r') | Key::Char('R') => Note::D,
                    Key::Char('t') | Key::Char('T') => Note::E,
                    Key::Char('y') | Key::Char('Y') => Note::F,
                    Key::Char('u') | Key::Char('U') => Note::G,
                    _ => panic!("Unexpected key"),
                };

                // Print the pressed note and current octave for debugging purposes
                println!("Note {:?}, Octave {:?}", note, octave.clone().value);

                // Set the oscillator frequency to correspond to the pressed note in the current octave
                oscillator.set_frequency(note.frequency(octave.clone()));

                // Generate random filter parameters and assign these to the oscillator if filter is active
                if oscillator.is_filter_active() {
                    // Generate random values for the low-pass filter parameters
                    let new_filter_cutoff = rand::random::<f32>();
                    let new_filter_resonance = rand::random::<f32>();

                    // Print the modified filter parameters for debugging purposes
                    println!(
                        "Filter parameters modified - Cutoff: {:.2}, Resonance: {:.2}",
                        new_filter_cutoff, new_filter_resonance
                    );

                    // Set the oscillator filter parameters with our random values
                    oscillator.set_filter_params(new_filter_cutoff, new_filter_resonance);
                }

                // Clone the oscillator for playback and create a sound source
                let cloned_oscillator = oscillator.clone();
                let sound_source = cloned_oscillator.take_duration(Duration::from_secs_f32(0.25));

                // Append the sound source to the audio sink for playback
                let _result = sink.append(sound_source);
            }
            Key::Char('o') | Key::Char('O') => {
                // Reduce the octave value and print the updated value for debugging purposes
                let new_octave = octave.value.clone() - 1;
                println!("Octave has been reduced from {:?} to {:?}", octave.value, new_octave);
                octave.value = new_octave;
            }
            Key::Char('p') | Key::Char('P') => {
                // Increase the octave value and print the updated value for debugging purposes
                let new_octave = octave.value.clone() + 1;
                println!("Octave has been increased from {:?} to {:?}", octave.value, new_octave);
                octave.value = new_octave;
            }
            Key::Char('f') | Key::Char('F') => {
                oscillator.modify_filter();
            }
            Key::Char('z') | Key::Char('Z') => {
                // Quit the program
                println!("Quitting...");
                break;
            }
            _ => {
                // Print a message for invalid keys
                println!("Invalid key. Press 'QWERTY' to play, 'O/P' to modify octave. F to modify filter and 'Z' to quit.");
            }
        }

        // Pause the thread to mitigate CPU overload
        thread::sleep(Duration::from_millis(50));
    }
}