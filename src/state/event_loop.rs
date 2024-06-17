use std::thread;
use std::time::Duration;
use console::{Key, Term};
use minifb::{Key as key2, Window, WindowOptions};
use image::GenericImageView;
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

    // Load the first image
    let img1 = image::open("assets/sine_key_0.png").expect("Failed to open key_0");
    let (width1, height1) = img1.dimensions();
    let buffer1: Vec<u32> = img_to_buffer(&img1);

    // Load the second image
    let img2 = image::open("assets/sine_key_1.png").expect("Failed to open key_1");
    let (width2, height2) = img2.dimensions();
    let buffer2: Vec<u32> = img_to_buffer(&img2);

    // Load the third image
    let img3 = image::open("assets/sine_key_2.png").expect("Failed to open key_2");
    let (width3, height3) = img3.dimensions();
    let buffer3: Vec<u32> = img_to_buffer(&img3);

    // Load the third image
    let img4 = image::open("assets/sine_key_3.png").expect("Failed to open key_3");
    let (width4, height4) = img4.dimensions();
    let buffer4: Vec<u32> = img_to_buffer(&img4);

    // Load the third image
    let img5 = image::open("assets/sine_key_4.png").expect("Failed to open key_4");
    let (width5, height5) = img5.dimensions();
    let buffer5: Vec<u32> = img_to_buffer(&img5);

    // Load the third image
    let img6 = image::open("assets/sine_key_5.png").expect("Failed to open key_5");
    let (width6, height6) = img6.dimensions();
    let buffer6: Vec<u32> = img_to_buffer(&img6);

    let img7 = image::open("assets/sine_key_6.png").expect("Failed to open key_6");
    let (width7, height7) = img7.dimensions();
    let buffer7: Vec<u32> = img_to_buffer(&img7);

    let img8 = image::open("assets/sine_key_7.png").expect("Failed to open key_7");
    let (width8, height8) = img8.dimensions();
    let buffer8: Vec<u32> = img_to_buffer(&img8);

    // Create a window
    let mut window = Window::new(
        "Image Display",
        width1 as usize,
        height1 as usize,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut current_buffer = &buffer1;

    // Display the images alternately based on key press
    while window.is_open() && !window.is_key_down(key2::Escape) {
        if window.is_key_pressed(key2::Q, minifb::KeyRepeat::No) {
            current_buffer = &buffer2;
            handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, Key::Char('q'));
        } else if window.is_key_pressed(key2::W, minifb::KeyRepeat::No) {
            current_buffer = &buffer3;
            handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, Key::Char('w'));
        } else if window.is_key_pressed(key2::E, minifb::KeyRepeat::No) {
            current_buffer = &buffer4;
            handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, Key::Char('e'));
        } else if window.is_key_pressed(key2::R, minifb::KeyRepeat::No) {
            current_buffer = &buffer5;
            handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, Key::Char('r'));
        } else if window.is_key_pressed(key2::T, minifb::KeyRepeat::No) {
            current_buffer = &buffer6;
            handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, Key::Char('t'));
        } else if window.is_key_pressed(key2::Y, minifb::KeyRepeat::No) {
            current_buffer = &buffer7;
            handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, Key::Char('y'));
        } else if window.is_key_pressed(key2::U, minifb::KeyRepeat::No) {
            current_buffer = &buffer8;
            handle_musical_notes(octave, term, keyboard, sink, &mut current_waveform, &mut filter_active, &mut filter_cutoff, &mut filter_resonance, Key::Char('u'));
        } else if window.is_key_pressed(key2::F, minifb::KeyRepeat::No) {
            current_buffer = &buffer6;
            handle_toggle_waveforms(term, keyboard, &mut current_waveform);
        }
        window.update_with_buffer(current_buffer, width1 as usize, height1 as usize).unwrap();
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

fn handle_toggle_waveforms(term: &mut Term, keyboard: &mut Keyboard, current_waveform: &mut Option<Waveform>) {
    *current_waveform = match current_waveform {
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

    // Simulate handling a key press
    //keyboard.handle_key_press(key);

    // Draw the updated keyboard layout after the key press
    //keyboard.draw(term);

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

fn img_to_buffer(img: &image::DynamicImage) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::with_capacity((img.width() * img.height()) as usize);
    for pixel in img.to_rgba8().pixels() {
        let rgba = pixel.0;
        let value = ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[2] as u32);
        buffer.push(value);
    }
    buffer
}