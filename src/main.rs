use std::thread;
use std::time::Duration;
use console::{Key, Term};
use rodio::{OutputStream, Sink};
use rodio::source::Source;

#[derive(Clone, Copy)]
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

        // Calculate the frequency for the specified octave by adjusting the base frequency with a factor determined by raising 2.0 to the power of (state.octave - 4).
        let octave_adjusted_frequency = base_frequency * 2.0_f32.powi(state.octave - 4);
        println!("Octave adjusted frequency {:?}", octave_adjusted_frequency);
        octave_adjusted_frequency
    }
}

#[derive(Clone)]
struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    filter_cutoff: f32,
    filter_resonance: f32,
    filtered_value: f32,
}


impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            filter_cutoff: 0.0,
            filter_resonance: 0.0,
            filtered_value: 0.0
        };
    }

    fn set_filter_params(&mut self, cutoff: f32, resonance: f32) {
        self.filter_cutoff = cutoff;
        self.filter_resonance = resonance;
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate.clone() as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment.clone();
        self.index %= self.wave_table.len() as f32;

        self.filtered_value = (1.0 - self.filter_cutoff.clone()) * self.filtered_value.clone()
            + self.filter_cutoff.clone() * sample;

        return self.filtered_value.clone();
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index.clone() as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();
        let next_index_weight = self.index.clone() - truncated_index.clone() as f32;
        let truncated_index_weight = 1.0 - next_index_weight;
        return truncated_index_weight * self.wave_table[truncated_index.clone()].clone() + next_index_weight.clone() * self.wave_table[next_index].clone();
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate.clone();
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn main() {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);


    for n in 0..wave_table_size {
        let mut sample = 0.0;

        for a in 1..=5 {
            let detune_factor = 0.25;
            let frequency = 220.0 * (a as f32).exp2() * (1.0 + detune_factor * (a.clone() as f32 - 1.0));
            let amplitude = 1.0 / a.clone() as f32;
            sample += amplitude * (2.0 * frequency * (n.clone() as f32) / wave_table_size.clone() as f32).sin();
        }

        wave_table.push(sample);
    }

    let mut state = State{octave: 4};
    let mut oscillator = WavetableOscillator::new(44100, wave_table);

    oscillator.set_frequency(Note::A.frequency(state));
    oscillator.set_filter_params(0.1, 0.1);

    let term = Term::stdout();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
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

               println!("Note {:?}, Octave {:?}", note, state.clone().octave);
                oscillator.set_frequency(note.frequency(state.clone()));
                let new_filter_cutoff = rand::random::<f32>(); // Random value between 0.0 and 1.0
                let new_filter_resonance = rand::random::<f32>();

                println!(
                    "Filter parameters modified - Cutoff: {:.2}, Resonance: {:.2}",
                    new_filter_cutoff, new_filter_resonance
                );

                oscillator.set_filter_params(new_filter_cutoff, new_filter_resonance);

                let cloned_oscillator = oscillator.clone();



                let sound_source = cloned_oscillator.take_duration(Duration::from_secs_f32(0.25)).convert_samples::<f32>();



                let _result = sink.append(sound_source);
            }
            Key::Char('o') | Key::Char('O') => {
                let new_octave = state.octave.clone() -1;
                println!("Octave has been reduced from {:?} to {:?}", state.octave, new_octave);
                state.octave = new_octave;
            }
            Key::Char('p') | Key::Char('P') => {
                let new_octave = state.octave.clone() +1;
                println!("Octave has been increased from {:?} to {:?}", state.octave, new_octave);
                state.octave = new_octave;
            }
            Key::Char('z') | Key::Char('Z') => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid key. Press 'QWERTY' to play, 'O/P' to modify octave and  'Z' to quit.");
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}
