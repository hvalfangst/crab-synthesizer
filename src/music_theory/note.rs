
#[derive(Clone, Copy)]
pub struct Octave {
    pub value: i32
}

/// Enumerates musical notes A, B, C, D, E, F, and G.
#[derive(Debug, Clone)]
pub enum Note {
    A, B, C, D, E, F, G,
}

impl Note {
    /// Computes the frequency of the note.rs based on the following: [frequency * (2^(octave-4))].
    ///
    /// # Arguments
    ///
    /// * `octave` - The current octave.
    ///
    /// # Returns
    ///
    /// The adjusted frequency of the note.rs based on the current octave.
    pub fn frequency(&self, octave: &Octave) -> f32 {
        let base_frequency = match self {
            Note::A => 440.0,
            Note::B => 493.88,
            Note::C => 523.25,
            Note::D => 587.33,
            Note::E => 659.25,
            Note::F => 698.46,
            Note::G => 783.99,
        };

        // Adjust the base frequency based on the current octave setting
        let octave_adjusted_frequency = base_frequency * 2.0_f32.powi(octave.value - 4);

        // Print the octave-adjusted frequency for debugging purposes
        println!("Octave adjusted frequency: {:?}", octave_adjusted_frequency);

        octave_adjusted_frequency
    }
}