use std::ops::Fn;

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

fn oscillator(w: f32, amplitute: f32) -> f32 {
    amplitute * f32::sin(w)
}

fn natural_oscillator<F>(
    frequency: f32,
    time_start: f32,
    time_curr: f32,
    amplitude_calculator: F,
    amplitude_modulator: F,
) -> f32
where
    F: Fn(f32) -> f32,
{
    let time_delta = time_curr - time_start;
    oscillator(
        frequency * amplitude_modulator(time_delta),
        amplitude_calculator(time_delta),
    )
}

fn get_w(frequency: f32, time: f32, sample_rate: f32) -> f32 {
    2.0 * std::f32::consts::PI * frequency * time / sample_rate
}

impl Note {
    pub fn combine(notes: &[Self], secs: f32, volume: &Volume) -> Vec<i16> {
        let nsamples = secs * SAMPLE_RATE as f32;
        (0..nsamples as u32)
            .map(|t| {
                notes.iter().map(move |note| match volume {
                    Volume::Silent => 0_f32,
                    _ => oscillator(get_w(note.frequency, t as f32, SAMPLE_RATE as f32), 1_f32),
                })
            })
            .map(|step| f32::floor(AMPLITUDE * volume.scaling() * step.sum::<f32>()) as i16)
            .collect::<Vec<i16>>()
    }

    pub fn audio_wave(&self, secs: f32, volume: &Volume) -> Vec<i16> {
        let nsamples = secs * SAMPLE_RATE as f32;
        (0..nsamples as u32)
            .map(|t| match *volume {
                Volume::Silent => 0_i16,
                _ => f32::floor(oscillator(
                    get_w(self.frequency, t as f32, SAMPLE_RATE as f32),
                    AMPLITUDE * volume.scaling(),
                )) as i16,
            })
            .collect()
    }
}
