pub mod sine_wave;
pub mod square_wave;
pub mod saw_wave;

#[derive(Debug, Clone)]
pub enum Waveform {
    SINE,
    SQUARE,
    SAW
}