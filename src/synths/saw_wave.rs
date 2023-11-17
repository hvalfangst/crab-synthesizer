use std::f32::consts::PI;
use rodio::Source;
use std::time::Duration;
use crate::synths::sine_wave::calculate_sine;

const MONO: u16 = 1;

#[derive(Clone, Debug)]
pub struct SawWave {
    freq: f32,
    num_sample: usize,
}

impl SawWave {
    pub fn new(freq: f32) -> SawWave {
        SawWave { freq, num_sample: 0 }
    }
}

/// Implementation of the `Iterator` trait for the `SawWave`.
impl Iterator for SawWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        // increment sample counter by 1
        self.num_sample = self.num_sample.wrapping_add(1);

        /// y(t) = 2A/π * arctan( tan ( (πft/a) )
        let time: f32 = self.num_sample.clone() as f32 / 48000.0;
        let first_portion: f32 = 2.0 * 0.20 / PI;
        let last_portion: f32 = (PI * self.freq.clone() * time) / 0.20;
        let tan_last_portion: f32 = last_portion.tan();
        let atan_tan_last_portion: f32 = tan_last_portion.atan();
        let saw_wave: f32 = first_portion * atan_tan_last_portion;
        Some(saw_wave)
    }
}

/// Implementation of the `Source` trait for the `SawWave`.
impl Source for SawWave {
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