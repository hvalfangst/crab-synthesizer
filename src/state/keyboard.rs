use crate::music_theory::{OCTAVE_LOWER_BOUND, OCTAVE_UPPER_BOUND};
use crate::waveforms::Waveform;

/// Represents a musical keyboard with functionality for handling octaves and waveforms.
pub struct Keyboard {
    current_octave: i32,
    current_waveform: Waveform,
}

impl Keyboard {
    /// Creates a new `Keyboard` instance with default values.
    pub fn new() -> Self {
        Self {
            current_octave: 4,
            current_waveform: Waveform::SINE,
        }
    }

    /// Increases the octave by one step, ensuring it does not exceed the upper bound.
    pub fn increase_octave(&mut self) {
        if self.current_octave < OCTAVE_UPPER_BOUND {
            self.current_octave += 1;
        }
    }

    /// Decreases the octave by one step, ensuring it does not go below the lower bound.
    pub fn decrease_octave(&mut self) {
        if self.current_octave > OCTAVE_LOWER_BOUND {
            self.current_octave -= 1;
        }
    }

    /// Returns the current octave value.
    pub fn get_current_octave(&self) -> i32 {
        self.current_octave
    }

    /// Toggles the waveform between SINE and SQUARE.
    pub fn toggle_waveform(&mut self) {
        self.current_waveform = match self.current_waveform {
            Waveform::SINE => Waveform::SQUARE,
            Waveform::SQUARE => Waveform::SINE,
        };
    }

    /// Returns the current waveform.
    pub fn get_current_waveform(&self) -> Waveform {
        self.current_waveform
    }
}
