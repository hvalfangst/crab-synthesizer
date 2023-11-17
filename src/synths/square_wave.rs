use rodio::Source;
use std::time::Duration;
use crate::synths::sine_wave::calculate_sine;

const MONO: u16 = 1;

#[derive(Clone, Debug)]
pub struct SquareWave {
    freq: f32,
    num_sample: usize,
}

impl SquareWave {
    pub fn new(freq: f32) -> SquareWave {
        SquareWave { freq, num_sample: 0 }
    }
}

/// Implementation of the `Iterator` trait for the `SquareWave`.
impl Iterator for SquareWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        // increment sample counter by 1
        self.num_sample = self.num_sample.wrapping_add(1);

        // Generates a sine wave
        let sine_wave: f32 = calculate_sine(self.freq.clone(), self.num_sample.clone());

        // Utilize a sign function to normalize our sine wave to [1.0, -1.0 or 0.0]
        let square_wave: f32 = sgn(sine_wave);

        Some(square_wave)
    }
}

/// Implementation of the `Source` trait for the `SquareWave`.
impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        MONO
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// Returns the sign of the given floating-point number.
///
/// The signum function returns:
/// - 1.0 if the number is positive,
/// - -1.0 if the number is negative,
/// - 0.0 if the number is zero.
///
/// # Arguments
///
/// * `x` - The floating-point number for which to determine the sign.
fn sgn(x: f32) -> f32 {
    x.signum()
}