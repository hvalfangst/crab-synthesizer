use console::{Key, style, StyledObject, Term};
use crate::music_theory::note::{get_all_notes, Note};
use crate::waveforms::Waveform;

pub struct Keyboard {
    keys_pressed: [bool; 7],
    current_octave: i32,
    current_waveform: Waveform
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 7],
            current_octave: 4,
            current_waveform: Waveform::SINE
        }
    }

    /// Draw the keyboard layout with styles based on key presses
    pub fn draw(&mut self, term: &mut Term) {
        let notes = get_all_notes();
        let key_styles = notes
            .iter()
            .enumerate()
            .zip(self.keys_pressed.clone().iter())
            .map(|((index, &ref label), &pressed)| {
                if pressed {
                    self.keys_pressed[index] = false;
                    style(label).green().on_black()
                } else {
                    style(label)
                }
            })
            .collect::<Vec<_>>();

        self.draw_keyboard_layout(term, &key_styles);
    }


    /// Draws the keyboard layout on the console with styling based on key presses.
    ///
    /// # Arguments
    ///
    /// * `term` - A mutable reference to the console `Term` for output.
    /// * `key_styles` - A vector of `StyledObject` representing the styling of each key.
    ///
    fn draw_keyboard_layout(&mut self, term: &mut Term, key_styles: &Vec<StyledObject<&Note>>) {
        term.write_line("+---+---+---+---+---+---+---+").unwrap();
        term.write_line(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | Octave: {}, Waveform: {}",
            key_styles[0],
            key_styles[1],
            key_styles[2],
            key_styles[3],
            key_styles[4],
            key_styles[5],
            key_styles[6],
            self.current_octave,
            self.current_waveform
        ))
            .unwrap();
        term.write_line("+---+---+---+---+---+---+---+").unwrap();
    }

    /// Handles key presses and updates the corresponding key state in the keyboard.
    ///
    /// # Arguments
    ///
    /// * `key` - The `Key` enum representing the pressed key.
    pub fn handle_key_press(&mut self, key: Key) {
        match key {
            Key::Char('q') => self.keys_pressed[0] = true,
            Key::Char('w') => self.keys_pressed[1] = true,
            Key::Char('e') => self.keys_pressed[2] = true,
            Key::Char('r') => self.keys_pressed[3] = true,
            Key::Char('t') => self.keys_pressed[4] = true,
            Key::Char('y') => self.keys_pressed[5] = true,
            Key::Char('u') => self.keys_pressed[6] = true,
            _ => {} // Ignore other key presses
        }
    }

    pub fn set_current_octave(&mut self, octave: &i32) {
        self.current_octave = *octave
    }

    pub fn set_current_waveform(&mut self, waveform: &Waveform) {
        self.current_waveform = *waveform
    }
}
