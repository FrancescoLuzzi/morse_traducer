use super::{get_w, oscillator, volume::Volume, MAX_AMPLITUDE, SAMPLE_RATE};

pub struct Note(pub f32);

impl Note {
    pub fn get_frequency(&self) -> f32 {
        self.0
    }

    pub fn combine(notes: &[Self], secs: f32, volume: &Volume) -> Vec<i16> {
        let nsamples = secs * SAMPLE_RATE as f32;
        (0..nsamples as u32)
            .map(|t| {
                notes.iter().map(move |note| match volume {
                    Volume::Silent => 0_f32,
                    _ => oscillator(
                        get_w(note.get_frequency(), t as f32, SAMPLE_RATE as f32),
                        1_f32,
                    ),
                })
            })
            .map(|step| f32::floor(MAX_AMPLITUDE * volume.scaling() * step.sum::<f32>()) as i16)
            .collect::<Vec<i16>>()
    }

    pub fn audio_wave(&self, secs: f32, volume: &Volume) -> Vec<i16> {
        let nsamples = secs * SAMPLE_RATE as f32;
        (0..nsamples as u32)
            .map(|t| match *volume {
                Volume::Silent => 0_i16,
                _ => f32::floor(oscillator(
                    get_w(self.get_frequency(), t as f32, SAMPLE_RATE as f32),
                    MAX_AMPLITUDE * volume.scaling(),
                )) as i16,
            })
            .collect()
    }
}