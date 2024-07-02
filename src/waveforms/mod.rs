use std::fmt;

pub mod sine_wave;
pub mod square_wave;

const MONO: u16 = 1;
const SAMPLE_RATE: f32 = 48000.0;
pub const AMPLITUDE: f32 = 0.20;
pub const DURATION: f32 = 0.19;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Waveform {
    SINE,
    SQUARE
}

/// Implements the [Display] trait for [WaveForm]
impl fmt::Display for Waveform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Waveform::SINE => write!(f, "Sine"),
            Waveform::SQUARE => write!(f, "Square"),
            _ => write!(f, "Saw")
        }
    }
}