use std::f32::consts::PI;
use rodio::Source;
use std::time::Duration;
use crate::synths::low_pass_filter::LowPassFilter;
use crate::synths::sine_wave::calculate_sine;


const MONO: u16 = 1;
const SAMPLE_RATE: f32 = 48000.0;
const AMPLITUDE: f32 = 0.20;

#[derive(Clone)]
pub struct SawWave {
    freq: f32,
    num_sample: usize,
    filter: LowPassFilter
}

impl SawWave {
    pub fn new(freq: f32) -> SawWave {
        SawWave { freq, num_sample: 0, filter: LowPassFilter::new() }
    }
}

/// Implementation of the `Iterator` trait for the `SawWave`.
impl Iterator for SawWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        // increment sample counter by 1
        self.num_sample = self.num_sample.wrapping_add(1);

        /// y(t) = 2A/π * arctan( tan ( (πft/a) )
        let time: f32 = self.num_sample.clone() as f32 / SAMPLE_RATE;
        let first_portion: f32 = 2.0 * AMPLITUDE / PI;
        let last_portion: f32 = (PI * self.freq.clone() * time) / AMPLITUDE;
        let tan_last_portion: f32 = last_portion.tan();
        let atan_tan_last_portion: f32 = tan_last_portion.atan();
        let saw_wave: f32 = first_portion * atan_tan_last_portion;

        // Only apply low-pass filter if it is indeed active
        if self.filter.filter_active {
            self.filter.filtered_value = saw_wave * self.filter.low_pass_filter();
            Some(self.filter.filtered_value.clone())
        } else {
            Some(saw_wave)
        }
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
        SAMPLE_RATE as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}