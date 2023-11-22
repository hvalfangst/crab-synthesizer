use console::{Key, style, Term};

pub struct Keyboard {
    keys_pressed: [bool; 7],
    current_octave: i32
}

impl Keyboard {
    // Create a new Keyboard instance with all keys initially not pressed
    pub(crate) fn new() -> Self {
        Self {
            keys_pressed: [false; 7],
            current_octave: 4
        }
    }

    // Draw the keyboard layout with styles based on key presses
    pub fn draw(&mut self, term: &mut Term) {
        // Labels for the keys
        let key_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];

        // Get styles for each key based on its label and pressed state
        let key_styles = key_labels
            .iter()
            .enumerate()
            .zip(self.keys_pressed.clone().iter())
            .map(|((index, &label), &pressed)| {
                // If the key is pressed, set its pressed state to false and style it accordingly
                if pressed {
                    self.keys_pressed[index] = false;
                    style(label).green().on_black()
                } else {
                    // If the key is not pressed, style it normally
                    style(label)
                }
            })
            .collect::<Vec<_>>();

        // Draw the keyboard layout
        term.write_line("+---+---+---+---+---+---+---+").unwrap();
        term.write_line(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | Octave: {}",
            key_styles[0],
            key_styles[1],
            key_styles[2],
            key_styles[3],
            key_styles[4],
            key_styles[5],
            key_styles[6],
            self.current_octave
        ))
            .unwrap();
        term.write_line("+---+---+---+---+---+---+---+").unwrap();
    }

    // Handle key presses and update the corresponding key state
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
        self.current_octave = *octave;
    }
}
