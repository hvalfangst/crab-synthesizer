use std::thread;
use std::time::Duration;
use console::{Key, Term};
use rodio::{OutputStream, Sink};
use rodio::source::{Amplify, SineWave, Source, TakeDuration};

#[derive(Clone)]
pub struct State {
    pub octave: i32
}

#[derive(Clone)]
pub struct WaveForm {
    pub frequency: f32,
    pub duration: f32,
    pub amplitude: f32
}

#[derive(Debug, Clone)]
enum Note {
    A, B, C, D, E, F, G,
}

impl Note {
    fn frequency(self, state: State) -> f32 {
        let base_frequency = match self {
            Note::A => 440.0,
            Note::B => 493.88,
            Note::C => 523.25,
            Note::D => 587.33,
            Note::E => 659.25,
            Note::F => 698.46,
            Note::G => 783.99,
        };

        // Calculate the frequency for the specified octave by adjusting the base frequency
        // with a factor determined by raising 2.0 to the power of (state.octave - 4).
        base_frequency * 2.0_f32.powi(state.octave - 4)
    }
}

fn get_sine_wave(note: Note, duration: f32, amplitude: f32, state: State) -> Amplify<TakeDuration<SineWave>> {
    SineWave::new(note.frequency(state))
        .take_duration(Duration::from_secs_f32(duration))
        .amplify(amplitude)
}

fn main() {
    let mut state = State{octave: 4};
    let waveform = WaveForm{frequency: 440.0, duration: 0.25, amplitude: 0.20};

    let term = Term::stdout();

    // Create an output stream and a handle to control the stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create a sink in a separate thread to play audio
    let sink = Sink::try_new(&stream_handle).unwrap();

    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Char('q') | Key::Char('Q')
            | Key::Char('w') | Key::Char('W')
            | Key::Char('e') | Key::Char('E')
            | Key::Char('r') | Key::Char('R')
            | Key::Char('t') | Key::Char('T')
            | Key::Char('y') | Key::Char('Y')
            | Key::Char('u') | Key::Char('U') => {
                let note = match key {
                    Key::Char('q') | Key::Char('Q') => Note::A,
                    Key::Char('w') | Key::Char('W') => Note::B,
                    Key::Char('e') | Key::Char('E') => Note::C,
                    Key::Char('r') | Key::Char('R') => Note::D,
                    Key::Char('t') | Key::Char('T') => Note::E,
                    Key::Char('y') | Key::Char('Y') => Note::F,
                    Key::Char('u') | Key::Char('U') => Note::G,
                    _ => panic!("Unexpected key"),
                };

                println!("Note {:?}, Octave {:?}", note, state.octave);
                sink.append(get_sine_wave(note, waveform.clone().duration, waveform.clone().amplitude, state.clone()));
            }
            Key::Char('o') | Key::Char('O') => {
                let new_octave = state.octave -1;
                println!("Octave has been reduced from {:?} to {:?}", state.octave, new_octave);
                state.octave = new_octave;
            }
            Key::Char('p') | Key::Char('P') => {
                let new_octave = state.octave +1;
                println!("Octave has been increased from {:?} to {:?}", state.octave, new_octave);
                state.octave = new_octave;
            }
            Key::Char('z') | Key::Char('Z') => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid key. Press 'Z' to play the sound. Press 'Q' to quit.");
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
