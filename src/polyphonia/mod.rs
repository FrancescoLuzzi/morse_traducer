pub const SAMPLE_RATE: u32 = 44100;
pub const AMPLITUDE: f32 = i16::MAX as f32;

pub enum Volume {
    Silent,
    Low,
    Medium,
    High,
    Custom(f32),
}

impl Volume {
    fn scaling(&self) -> f32 {
        match self {
            Volume::Silent => 0.0,
            Volume::Low => 0.3,
            Volume::Medium => 0.5,
            Volume::High => 0.8,
            Volume::Custom(volume) => volume.clamp(0.01, 1.0),
        }
    }
}

pub struct Note {
    frequency: f32,
}

pub mod notable_notes {
    use super::Note;
    pub const A4: Note = Note {
        frequency: 440.0_f32,
    };
    pub const E0: Note = Note {
        frequency: 20.60_f32,
    };
    pub const G0: Note = Note {
        frequency: 24.50_f32,
    };
    pub const C0: Note = Note {
        frequency: 16.35_f32,
    };
    pub const C4_SH: Note = Note {
        frequency: 277.18_f32,
    };
}

impl Note {
    pub fn combine(notes: &[Self], secs: f32, volume: &Volume) -> Vec<i16> {
        let nsamples = (secs * SAMPLE_RATE as f32) as u32;
        let mut buf: Vec<i16> = vec![0; nsamples as usize];
        let notes_number = notes.len() as i16;
        for values in notes.iter().map(|note| note.audio_wave(secs, volume)) {
            for (new_val, val) in buf.iter_mut().zip(&values) {
                *new_val += val / notes_number;
            }
        }
        buf
    }

    pub fn audio_wave(&self, secs: f32, volume: &Volume) -> Vec<i16> {
        let nsamples = (secs * SAMPLE_RATE as f32) as u32;
        let mut buf: Vec<i16> = Vec::new();
        for t in 0..nsamples {
            let s = match *volume {
                Volume::Silent => 0_i16,
                _ => {
                    let w = 2.0 * std::f32::consts::PI * self.frequency * t as f32;
                    let s = f32::sin(w / (SAMPLE_RATE as f32));
                    f32::floor(AMPLITUDE * (volume.scaling() * s)) as i16
                }
            };
            buf.push(s);
        }
        buf
    }
}
