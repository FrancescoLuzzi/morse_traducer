pub enum Amplitude {
    Silent,
    Low,
    Medium,
    High,
    Custom(f32),
}

impl Amplitude {
    pub fn scaling(&self) -> f32 {
        match self {
            Amplitude::Silent => 0.0,
            Amplitude::Low => 0.3,
            Amplitude::Medium => 0.5,
            Amplitude::High => 0.8,
            Amplitude::Custom(volume) => volume.clamp(0.01, 1.0),
        }
    }
}
