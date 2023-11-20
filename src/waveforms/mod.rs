pub mod sine_wave;
pub mod square_wave;
pub mod saw_wave;

const MONO: u16 = 1;
const SAMPLE_RATE: f32 = 48000.0;
pub const AMPLITUDE: f32 = 0.20;
pub const DURATION: f32 = 0.25;

#[derive(Debug, Clone)]
pub enum Waveform {
    SINE,
    SQUARE,
    SAW
}