pub enum Volume {
    Silent,
    Low,
    Medium,
    High,
    Custom(f32),
}

impl Volume {
    pub fn scaling(&self) -> f32 {
        match self {
            Volume::Silent => 0.0,
            Volume::Low => 0.3,
            Volume::Medium => 0.5,
            Volume::High => 0.8,
            Volume::Custom(volume) => volume.clamp(0.01, 1.0),
        }
    }
}
