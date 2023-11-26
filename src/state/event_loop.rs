use std::thread;
use std::time::Duration;
use console::{Key, Term};
use rodio::{Sink, source::Source};
use crate::{music_theory::{
    note::Note,
    note::Octave
}, waveforms::{
    Waveform,
    sine_wave::SineWave,
    square_wave::SquareWave,
    saw_wave::SawWave,
    DURATION, AMPLITUDE
}, effects::{
    FILTER_CUTOFF_UPPER_BOUND, FILTER_CUTOFF_LOWER_BOUND,
    FILTER_RESONANCE_UPPER_BOUND, FILTER_RESONANCE_LOWER_BOUND
}, Keyboard};

/// Executes the main event loop, which handles user input and sound generation.
///
/// # Arguments
///
/// * `octave` - The current octave.
/// * `oscillator` - The wavetable oscillator responsible for generating audio samples.
/// * `term` - The console terminal for user input.
/// * `sink` - The audio sink for playback.
pub fn execute_event_loop(octave: &mut Octave, term: &mut Term, keyboard: &mut Keyboard, sink: &mut Sink) {
    let mut current_waveform: Option<Waveform> = None;
    let mut filter_active = false;
    let mut filter_cutoff: f32 = 0.0;
    let mut filter_resonance: f32 = 0.0;

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
                handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, key);
            }
            Key::Char('o') | Key::Char('O') => {
                handle_reduce_octave(octave, term, keyboard);
            }
            Key::Char('p') | Key::Char('P') => {
                handle_increase_octave(octave, term, keyboard);
            }
            Key::Char('f') | Key::Char('F') => {
                handle_toggle_waveforms(term, keyboard, current_waveform);
            }
            Key::Char('1') => {
                handle_increase_filter_cutoff(&mut filter_cutoff);
            }
            Key::Char('2') => {
                handle_reduce_filter_cutoff(&mut filter_cutoff);
            }
            Key::Char('3') => {
                handle_activate_filter(&mut filter_active);
            }
            Key::Char('4') => {
                handle_increase_filter_resonance(&mut filter_resonance);
            }
            Key::Char('5') => {
                handle_reduce_filter_resonance(&mut filter_resonance);
            }
            Key::Char('z') | Key::Char('Z') => {
                break;
            }
            _ => {}
        }

        // Pause the thread to mitigate CPU overload
        thread::sleep(Duration::from_millis(10));
    }
}

fn handle_activate_filter(filter_active: &mut bool) {
    *filter_active = true;
    println!("Low pass filter has been activated")
}

fn handle_increase_filter_cutoff(filter_cutoff: &mut f32) {
    if *filter_cutoff < FILTER_CUTOFF_UPPER_BOUND {
        *filter_cutoff += 0.1;
        println!("Filter cutoff has been increased to {:?}", filter_cutoff)
    } else {
        println!("Filter cutoff is too high: {:?}", filter_cutoff)
    }
}

fn handle_reduce_filter_cutoff(filter_cutoff: &mut f32) {
    if *filter_cutoff > FILTER_CUTOFF_LOWER_BOUND {
        *filter_cutoff -= 0.1;
        println!("Filter cutoff has been reduced to {:?}", filter_cutoff)
    } else {
        println!("Filter cutoff is too low: {:?}", filter_cutoff)
    }
}

fn handle_increase_filter_resonance(filter_resonance: &mut f32) {
    if *filter_resonance < FILTER_RESONANCE_UPPER_BOUND {
        *filter_resonance += 0.1;
        println!("Filter resonance has been increased to {:?}", filter_resonance)
    } else {
        println!("Filter resonance is too high: {:?}", filter_resonance)
    }
}

fn handle_reduce_filter_resonance(filter_resonance: &mut f32) {
    if *filter_resonance > FILTER_RESONANCE_LOWER_BOUND {
        *filter_resonance -= 0.1;
        println!("Filter resonance has been reduced to {:?}", filter_resonance)
    } else {
        println!("Filter resonance is too low: {:?}", filter_resonance)
    }
}

fn handle_toggle_waveforms(term: &mut Term, keyboard: &mut Keyboard, mut current_waveform: Option<Waveform>) {
    current_waveform = match current_waveform {
        Some(Waveform::SINE) => Some(Waveform::SQUARE),
        Some(Waveform::SQUARE) => Some(Waveform::SAW),
        _ => Some(Waveform::SINE)
    };

    keyboard.set_current_waveform(current_waveform.as_ref().unwrap());
    term.clear_screen().expect("handle_toggle_waveforms has panicked!");
    keyboard.draw(term);
}

fn handle_increase_octave(octave: &mut Octave, term: &mut Term, keyboard: &mut Keyboard) {
    let new_octave = &octave.value + 1;
    octave.value = new_octave;
    keyboard.set_current_octave(&octave.value);
    term.clear_screen().expect("handle_increase_octave has panicked!");
    keyboard.draw(term);
}

fn handle_reduce_octave(octave: &mut Octave, term: &mut Term, keyboard: &mut Keyboard) {
    let new_octave = &octave.value - 1;
    octave.value = new_octave;
    keyboard.set_current_octave(&octave.value);
    term.clear_screen().expect("handle_reduce_octave has panicked!");
    keyboard.draw(term);
}

fn handle_musical_notes(octave: &mut Octave, term: &mut Term, keyboard: &mut Keyboard, sink: &mut Sink,
                        current_waveform: &mut Option<Waveform>, filter_active: &mut bool,
                        filter_cutoff: &mut f32, filter_resonance: &mut f32, key: Key) {
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

    // Draw the initial keyboard layout
    term.clear_screen().expect("handle_musical_notes has panicked!");

    // Simulate handling a key press (replace this with your actual key press handling logic)
    keyboard.handle_key_press(key);

    // Draw the updated keyboard layout after the key press
    keyboard.draw(term);

    // Initialize Synth based on currently Enum
    let synth = match current_waveform {
        Some(Waveform::SQUARE) => {
            let mut square_wave = SquareWave::new(note.frequency(octave));
            if *filter_active {
                square_wave.filter.modify_filter();
                square_wave.filter.change_cutoff(*filter_cutoff);
                square_wave.filter.change_resonance(*filter_resonance);
            }
            Box::new(square_wave) as Box<dyn Source<Item=f32> + 'static + Send>
        }
        Some(Waveform::SAW) => {
            let mut saw_wave = SawWave::new(note.frequency(octave));
            if *filter_active {
                saw_wave.filter.modify_filter();
                saw_wave.filter.change_cutoff(*filter_cutoff);
                saw_wave.filter.change_resonance(*filter_resonance);
            }
            Box::new(saw_wave) as Box<dyn Source<Item=f32> + 'static + Send>
        }
        _ => {
            let mut sine_wave = SineWave::new(note.frequency(octave));
            if *filter_active {
                sine_wave.filter.modify_filter();
                sine_wave.filter.change_cutoff(*filter_cutoff);
                sine_wave.filter.change_resonance(*filter_resonance);
            }
            Box::new(sine_wave) as Box<dyn Source<Item=f32> + 'static + Send>
        }
    };

    // Create Source from our Synth
    let source = synth.take_duration(Duration::from_secs_f32(DURATION)).amplify(AMPLITUDE);

    // Append the sound source to the audio sink for playback
    let _result = sink.append(source);
}