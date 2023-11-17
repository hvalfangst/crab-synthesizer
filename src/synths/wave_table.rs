use std::time::Duration;
use rodio::source::Source;

#[derive(Clone, Copy)]
pub struct Octave {
    pub value: i32,
}

#[derive(Clone)]
pub struct WaveForm {
    pub frequency: f32,
    pub duration: f32,
    pub amplitude: f32,
}

/// Enumerates musical notes A, B, C, D, E, F, and G.
#[derive(Debug, Clone)]
pub enum Note {
    A, B, C, D, E, F, G,
}

impl Note {
    /// Computes the frequency of the note based on the following: [frequency * (2^(octave-4))].
    ///
    /// # Arguments
    ///
    /// * `octave` - The current octave.
    ///
    /// # Returns
    ///
    /// The adjusted frequency of the note based on the current octave.
    pub fn frequency(self, octave: Octave) -> f32 {
        // Base frequencies for each note
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

#[derive(Clone)]
pub struct WavetableOscillator {
    sample_rate: u32,  // Sample rate of the audio system
    wave_table: Vec<f32>, // The wavetable containing the waveform samples
    index: f32,  // Current position in the wavetable
    index_increment: f32,  // Increment for the wavetable position based on frequency
    filter_cutoff: f32, // Cutoff frequency for the low-pass filter
    filter_resonance: f32,  // Resonance parameter for the low-pass filter
    filtered_value: f32,  // Output of the low-pass filter
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            filter_cutoff: 0.0,
            filter_resonance: 0.0,
            filtered_value: 0.0,
        }
    }

    /// Sets the parameters for the low-pass filter.
    pub fn set_filter_params(&mut self, cutoff: f32, resonance: f32) {
        self.filter_cutoff = cutoff;
        self.filter_resonance = resonance;
    }

    /// Sets the frequency of the oscillator.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate.clone() as f32;
    }

    /// Gets the next sample from the oscillator.
    pub fn get_sample(&mut self) -> f32 {
        let sample = self.linear_interpolation();
        self.index += self.index_increment.clone();
        self.index %= self.wave_table.len() as f32;

        // Apply low-pass filter
        self.filtered_value =
            (1.0 - self.filter_cutoff.clone()) * self.filtered_value.clone() + self.filter_cutoff.clone() * sample;

        self.filtered_value.clone()
    }

    /// Linear interpolation between two adjacent samples in the wavetable.
    fn linear_interpolation(&self) -> f32 {
        // Convert the current index to an integer (truncating the decimal part)
        let truncated_index = self.index.clone() as usize;

        // Calculate the index of the next sample in the wavetable (wrapping around if necessary)
        let next_index = (truncated_index + 1) % self.wave_table.len();

        // Calculate the weight for the next index based on the decimal part of the current index
        let next_index_weight = self.index.clone() - truncated_index.clone() as f32;

        // Calculate the weight for the truncated index
        let truncated_index_weight = 1.0 - next_index_weight;

        // Linear interpolation formula
        let interpolated_value = truncated_index_weight * self.wave_table[truncated_index.clone()].clone()
            + next_index_weight.clone() * self.wave_table[next_index].clone();

        // Example:

        // vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0] (6 elements)

        // Assuming the current self.index is '2.3':

        //  truncated_index becomes 2
        //  next_index becomes 3 due to ((2 + 1) % 6)

        // That is; we wish to insert a value between '0.4' (index 2) and '0.6' (index 3)

        // next_index_weight becomes 0.3
        // truncated_index_weight becomes 0.7 due to (1.0 - 0.3)

        // The linear interpolation formula then calculates the interpolated value:
        //      (truncated_index_weight *  wave_table[truncated_index]) + (next_index_weight * wavetable[next_index])
        //      (0.7                    *  0.4    =   0.28            ) + (0.3               * 0.6     =    0.18    ) => (0.28 + 0.18) = 0.46
        interpolated_value
    }

}

/// Implementation of the `Iterator` trait for the `WavetableOscillator`. This allows instances of `WavetableOscillator` to be used as iterators, generating a sequence of audio samples.
impl Iterator for WavetableOscillator {

    /// The type of item produced by the iterator (`f32` in this case, representing audio samples).
    type Item = f32;

    /// Returns the next audio sample from the `WavetableOscillator`.
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

/// Implementation of the `Source` trait for the `WavetableOscillator`. This allows instances of `WavetableOscillator` to be used as a source for audio playback.
impl Source for WavetableOscillator {

    /// Returns the length of the current frame, if available.
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    /// Returns the number of channels in the audio source. A value of `1` indicates mono audio.
    fn channels(&self) -> u16 {
        1
    }

    ///  Returns the sample rate of the audio source. This information is necessary for compatibility with audio playback systems.
    fn sample_rate(&self) -> u32 {
        self.sample_rate.clone()
    }

    /// Returns the total duration of the audio source, if available.
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}


/// Populates a given `wave_table` with samples for a wavetable oscillator.
///
/// # Arguments
///
/// * `wave_table_size` - The size of the wavetable to be populated.
/// * `wave_table` - A mutable reference to the vector that will store the generated samples.
pub fn populate_wave_table(wave_table_size: usize, wave_table: &mut Vec<f32>) {
    // Iterate over each sample index in the wavetable
    for n in 0..wave_table_size {
        let mut sample = 0.0;

        // Iterate over harmonics from 1 to 5
        for harmonics in 1..=5 {
            // Detune factor to introduce slight detuning for each harmonic
            let detune_factor = 0.25;

            // Calculate frequency and amplitude for the current harmonic
            let frequency = 220.0 * (harmonics as f32).exp2() * (1.0 + detune_factor * (harmonics.clone() as f32 - 1.0));
            let amplitude = 1.0 / harmonics.clone() as f32;

            // Generate the sample using a sine wave for the current harmonic
            sample += amplitude * (2.0 * frequency * (n.clone() as f32) / wave_table_size.clone() as f32).sin();
        }

        // Add the generated sample to the wavetable
        wave_table.push(sample);
    }
}