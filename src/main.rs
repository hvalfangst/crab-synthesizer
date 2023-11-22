use std::io::Write;
use console::{Key, style, Term};
use rodio::{OutputStream, Sink};
use crate::{
    music_theory::note::Octave,
    state::event_loop::execute_event_loop,
};

mod waveforms;mod state;mod music_theory; mod effects;

pub struct Keyboard {
    q_key_pressed: bool,
    w_key_pressed: bool,
    e_key_pressed: bool,
    r_key_pressed: bool,
    t_key_pressed: bool,
    y_key_pressed: bool,
    u_key_pressed: bool
}

impl Keyboard {
    fn new() -> Self {
        Self {
            q_key_pressed: false,
            w_key_pressed: false,
            e_key_pressed: false,
            r_key_pressed: false,
            t_key_pressed: false,
            y_key_pressed: false,
            u_key_pressed: false
        }
    }

    fn draw(&mut self, term: &mut Term) {
        let mut q_key_style = style("A");
        let mut w_key_style = style("B");
        let mut e_key_style = style("C");
        let mut r_key_style = style("D");
        let mut t_key_style = style("E");
        let mut y_key_style = style("F");
        let mut u_key_style = style("G");

        if self.q_key_pressed {
            q_key_style = style("A").green().on_black();
            self.q_key_pressed = false
        } else if self.w_key_pressed {
            w_key_style = style("B").green().on_black();
            self.w_key_pressed = false
        } else if self.e_key_pressed {
            e_key_style = style("C").green().on_black();
            self.e_key_pressed = false
        } else if self.r_key_pressed {
            r_key_style = style("D").green().on_black();
            self.r_key_pressed = false
        } else if self.t_key_pressed {
            t_key_style = style("E").green().on_black();
            self.t_key_pressed = false
        } else if self.y_key_pressed {
            y_key_style = style("F").green().on_black();
            self.y_key_pressed = false
        } else if self.u_key_pressed {
            u_key_style = style("G").green().on_black();
            self.u_key_pressed = false
        }

        // Draw the keyboard layout
        term.write_line("+---+---+---+---+---+---+---+").unwrap();
        term.write_line(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            q_key_style, w_key_style, e_key_style, r_key_style, t_key_style, y_key_style, u_key_style
        ))
            .unwrap();
        term.write_line("+---+---+---+---+---+---+---+").unwrap();
    }

    fn handle_key_press(&mut self, key: Key) {
        match key {
            Key::Char('q') => self.q_key_pressed = true,
            Key::Char('w') => self.w_key_pressed = true,
            Key::Char('e') => self.e_key_pressed = true,
            Key::Char('r') => self.r_key_pressed = true,
            Key::Char('t') => self.t_key_pressed = true,
            Key::Char('y') => self.y_key_pressed = true,
            Key::Char('u') => self.u_key_pressed = true,
            _ => {}
        }
    }
}

fn main() {
    // Set the initial octave value to 4
    let mut octave = Octave { value: 4 };

    // Initialize a console terminal for user interaction
    let term = &mut Term::stdout();

    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Draw the keyboard
    let mut keyboard = Keyboard::new();

    // Execute the main event loop, which handles user input and associated sound generation
    execute_event_loop(&mut octave, term, &mut keyboard, sink);
}