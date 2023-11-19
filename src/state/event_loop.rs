use std::thread;
use std::time::Duration;
use console::{Key, Term};
use rodio::{Sink, source::Source};
use crate::Octave;
use crate::synths::{sine_wave::SineWave, square_wave::SquareWave, saw_wave::SawWave, note::Note, Waveform};

const FREQ_NOT_SET: f32 = 666.6;
const DURATION: f32 = 0.25;
const AMPLITUDE: f32 = 0.20;

/// Executes the main event loop, which handles user input and sound generation.
///
/// # Arguments
///
/// * `octave` - The current octave.
/// * `oscillator` - The wavetable oscillator responsible for generating audio samples.
/// * `term` - The console terminal for user input.
/// * `sink` - The audio sink for playback.
pub fn execute_event_loop(mut octave: Octave, term: Term, sink: Sink) {
    let mut current_waveform: Option<Waveform> = None;

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

                // Print the pressed note.rs and current octave for debugging purposes
                println!("Note {:?}, Octave {:?}", note, octave.clone().value);

                // Initialize Synth based on currently Enum
                let synth = match current_waveform {
                    Some(Waveform::SQUARE) => {
                        let square_wave = SquareWave::new(note.clone().frequency(octave));
                        Box::new(square_wave) as Box<dyn Source<Item = f32> + 'static + Send>
                    }
                    Some(Waveform::SAW) => {
                        let saw_wave = SawWave::new(note.clone().frequency(octave));
                        Box::new(saw_wave) as Box<dyn Source<Item = f32> + 'static + Send>
                    }
                    _ => {
                        let sine_wave = SineWave::new(note.clone().frequency(octave));
                        Box::new(sine_wave) as Box<dyn Source<Item = f32> + 'static + Send>
                    }
                };

                // Create Source from our Synth
                let source = synth.take_duration(Duration::from_secs_f32(DURATION)).amplify(AMPLITUDE);

                // Append the sound source to the audio sink for playback
                let _result = sink.append(source);
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
                current_waveform = match current_waveform {
                    Some(Waveform::SINE) => Some(Waveform::SQUARE),
                    Some(Waveform::SQUARE) => Some(Waveform::SAW),
                    _ => Some(Waveform::SINE)
                };
                println!("Current Waveform was changed to {:?}", current_waveform)
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