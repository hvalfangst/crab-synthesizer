use std::time::Duration;
use rodio::Source;
use crate::synths::saw_wave::SawWave;
use crate::synths::sine_wave::SineWave;
use crate::synths::square_wave::SquareWave;

pub mod note;
pub mod sine_wave;
pub mod square_wave;
pub mod saw_wave;
pub mod low_pass_filter;

#[derive(Debug, Clone)]
pub enum Waveform {
    SINE,
    SQUARE,
    SAW
}