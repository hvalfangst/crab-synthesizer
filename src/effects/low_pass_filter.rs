
#[derive(Debug)]
pub struct LowPassFilter {
    pub filter_active: bool, // Whether or not the filter has been activated
    pub filter_cutoff: f32, // Cutoff frequency for the low-pass filter
    pub filter_resonance: f32,  // Resonance parameter for the low-pass filter
    pub filtered_value: f32,  // Output of the low-pass filter
}

impl LowPassFilter {
    pub fn new() -> LowPassFilter {
        LowPassFilter {
            filter_active: false,
            filter_cutoff: 0.0,
            filter_resonance: 0.0,
            filtered_value: 0.0
        }
    }

    pub fn filter_active(&mut self) -> bool {
        self.filter_active
    }

    pub fn modify_filter(&mut self) {
        if self.filter_active == false {
            println!("Low-pass filter has been activated");
            self.filter_active = true;
        } else {
            println!("Low-pass filter has been deactivated");
            self.filter_active = false;
        }
    }

    pub fn change_cutoff(&mut self, cutoff: f32) {
        self.filter_cutoff = cutoff;
    }

    pub fn change_resonance(&mut self, resonance: f32) {
        self.filter_resonance = resonance;
    }

    pub fn low_pass_filter(&self) -> f32 {
        ( (1.0 - self.filter_cutoff) * self.filtered_value) + self.filter_cutoff
    }
}