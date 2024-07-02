use std::time::Duration;
use rodio::Source;
use crate:: {
    waveforms::{MONO, AMPLITUDE, SAMPLE_RATE}
};

#[derive(Debug)]
pub struct AdsrEnvelope {
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,
    elapsed_time: f32,
    is_released: bool,
}

impl Iterator for AdsrEnvelope {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        // Implement ADSR envelope logic here
        // Return None when the envelope is complete

        // Placeholder implementation (replace with your logic)
        Some(1.0) // Default to full amplitude for now
    }
}

/// Implementation of the [Source] trait for the [AdsrEnvelope]
impl Source for AdsrEnvelope {
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

impl AdsrEnvelope {
    pub fn new(attack_time: f32, decay_time: f32, sustain_level: f32, release_time: f32) -> Self {
        AdsrEnvelope {
            attack_time,
            decay_time,
            sustain_level,
            release_time,
            elapsed_time: 0.0,
            is_released: false,
        }
    }

    pub fn get_amplitude(&mut self, time: f32) -> f32 {
        self.elapsed_time += time;

        if self.is_released {
            // Release stage
            if self.elapsed_time < self.release_time {
                // Linear release
                1.0 - (self.elapsed_time / self.release_time)
            } else {
                0.0
            }
        } else {
            // Attack stage
            if self.elapsed_time < self.attack_time {
                // Linear attack
                self.elapsed_time / self.attack_time
            }
            // Decay and sustain stage
            else if self.elapsed_time < (self.attack_time + self.decay_time) {
                // Linear decay
                1.0 - (self.elapsed_time - self.attack_time) / self.decay_time * (1.0 - self.sustain_level)
            } else {
                // Sustain
                self.sustain_level
            }
        }
    }

    pub fn release(&mut self) {
        self.is_released = true;
        self.elapsed_time = 0.0;
    }

    pub fn reset(&mut self) {
        self.elapsed_time = 0.0;
        self.is_released = false;
    }
}