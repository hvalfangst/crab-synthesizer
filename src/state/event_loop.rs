use std::thread;
use std::time::Duration;
use console::{Key, Term};
use rodio::{Sink, source::Source};
use crate::{
    music_theory::{
        note::Note,
        note::Octave
    },
    waveforms::{
        Waveform,
        sine_wave::SineWave,
        square_wave::SquareWave,
        saw_wave::SawWave
    }
};

const DURATION: f32 = 0.25;
const AMPLITUDE: f32 = 0.20;
const FILTER_CUTOFF_UPPER_BOUND: f32 = 0.91;
const FILTER_CUTOFF_LOWER_BOUND: f32 = 0.11;


/// Executes the main event loop, which handles user input and sound generation.
///
/// # Arguments
///
/// * `octave` - The current octave.
/// * `oscillator` - The wavetable oscillator responsible for generating audio samples.
/// * `term` - The console terminal for user input.
/// * `sink` - The audio sink for playback.
pub fn execute_event_loop(octave: &mut Octave, term: Term, sink: Sink) {
    let mut current_waveform: Option<Waveform> = None;
    let mut filter_active = false;
    let mut filter_cutoff: f32 = 0.0;

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
                println!("Note {:?}, Octave {:?}", note, octave.value);

                // Initialize Synth based on currently Enum
                let synth = match current_waveform {
                    Some(Waveform::SQUARE) => {
                        let mut square_wave = SquareWave::new(note.frequency(octave));
                        if filter_active {
                            square_wave.filter.modify_filter();
                            square_wave.filter.filter_cutoff = filter_cutoff;
                        }
                        Box::new(square_wave) as Box<dyn Source<Item = f32> + 'static + Send>
                    }
                    Some(Waveform::SAW) => {
                        let mut saw_wave = SawWave::new(note.frequency(octave));
                        if filter_active {
                            saw_wave.filter.modify_filter();
                            saw_wave.filter.filter_cutoff = filter_cutoff;
                        }
                        Box::new(saw_wave) as Box<dyn Source<Item = f32> + 'static + Send>
                    }
                    _ => {
                        let mut sine_wave = SineWave::new(note.frequency(octave));
                        if filter_active {
                            sine_wave.filter.modify_filter();
                            sine_wave.filter.filter_cutoff = filter_cutoff;
                        }
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
                let new_octave = &octave.value - 1;
                println!("Octave has been reduced from {:?} to {:?}", octave.value, new_octave);
                octave.value = new_octave;
            }
            Key::Char('p') | Key::Char('P') => {
                // Increase the octave value and print the updated value for debugging purposes
                let new_octave = &octave.value + 1;
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
            Key::Char('1') => {
                if filter_cutoff < FILTER_CUTOFF_UPPER_BOUND {
                    filter_cutoff += 0.1;
                    println!("Filter cutoff has been increased to {:?}", filter_cutoff)
                } else {
                    println!("Filter cutoff is too high: {:?}", filter_cutoff)
                }
            }
            Key::Char('2') => {
                if filter_cutoff > FILTER_CUTOFF_LOWER_BOUND {
                    filter_cutoff -= 0.1;
                    println!("Filter cutoff has been reduced to {:?}", filter_cutoff)
                } else {
                    println!("Filter cutoff is too low: {:?}", filter_cutoff)
                }
            }
            Key::Char('3') => {
                filter_active = true;
                println!("Low pass filter has been activated")
            }
            Key::Char('z') | Key::Char('Z') => {
                // Quit the program
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Menu:");
                println!("  Q - Play Note A");
                println!("  W - Play Note B");
                println!("  E - Play Note C");
                println!("  R - Play Note D");
                println!("  T - Play Note E");
                println!("  Y - Play Note F");
                println!("  U - Play Note G");
                println!("  O - Decrease Octave");
                println!("  P - Increase Octave");
                println!("  F - Change Synth");
                println!("  3 - Activate Low Pass Filter");
                println!("  1 - Increase Filter Cutoff");
                println!("  2 - Decrease Filter Cutoff");
                println!("  Z - Quit");
            }
        }

        // Pause the thread to mitigate CPU overload
        thread::sleep(Duration::from_millis(50));
    }
}