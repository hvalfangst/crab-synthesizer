use console::{Key, style, StyledObject, Term};
use crate::music_theory::note::Note;
use crate::waveforms::Waveform;

pub struct Keyboard {
    current_octave: i32,
    current_waveform: Waveform
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            current_octave: 4,
            current_waveform: Waveform::SINE
        }
    }

    pub fn set_current_octave(&mut self, octave: &i32) {
        self.current_octave = *octave
    }

    pub fn set_current_waveform(&mut self, waveform: &Waveform) {
        self.current_waveform = *waveform
    }

    pub fn toggle_waveform(&mut self) {
        self.current_waveform = match self.current_waveform {
            Waveform::SINE => Waveform::SQUARE,
            Waveform::SQUARE => Waveform::SINE
        };
    }

    pub fn get_current_waveform(&mut self) -> Waveform {
        return self.current_waveform
    }
}
