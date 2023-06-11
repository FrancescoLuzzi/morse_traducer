use std::fs::OpenOptions;
use std::io::{self, Write};

pub const SAMPLE_RATE: u32 = 44100;

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
            Volume::Custom(volume) => volume.clamp(0.01, 1.0) as f32,
        }
    }
}

pub struct Note {
    frequency: f32,
}

pub mod notable_notes {
    use crate::wav::Note;
    pub const A4: Note = Note { frequency: 440_f32 };
    pub const E0: Note = Note {
        frequency: 20.60_f32,
    };
    pub const G0: Note = Note {
        frequency: 24.50_f32,
    };
    pub const C0: Note = Note {
        frequency: 16.35_f32,
    };
}

impl Note {
    pub fn combine(notes: &[Self], secs: usize, volume: &Volume) -> Vec<u8> {
        let nsamples = secs * SAMPLE_RATE as usize;
        let mut buf: Vec<u8> = Vec::new();
        let notes_number = notes.len() as u8;
        for _ in 0..nsamples {
            buf.push(0);
        }
        for values in notes.iter().map(|note| note.audio_wave(secs, volume)) {
            for (new_val, val) in buf.iter_mut().zip(values) {
                *new_val += val / notes_number;
            }
        }
        buf
    }

    pub fn audio_wave(&self, secs: usize, volume: &Volume) -> Vec<u8> {
        let nsamples = secs * SAMPLE_RATE as usize;
        let mut buf: Vec<u8> = Vec::new();
        for t in 0..nsamples {
            let s = match *volume {
                Volume::Silent => 0_16,
                _ => {
                    let w = 2.0 * std::f32::consts::PI * self.frequency * t as f32;
                    let s = f32::sin(w / (SAMPLE_RATE as f32));
                    f32::floor(u16::MAX as f32 * (volume.scaling() * s)) as i16
                }
            };
            buf.extend_from_slice(&make_bytes::<u16>(s as u16));
        }
        buf
    }
}

fn make_bytes<T>(number: T) -> Vec<u8>
where
    T: Into<u32>,
{
    let number: u32 = number.into();
    let mut b: Vec<u8> = Vec::new();
    for i in 0..std::mem::size_of::<T>() {
        b.push(((number >> (8 * i)) & 0xff) as u8);
    }
    b
}

fn write_wav(data: Vec<u8>, writer: &mut Box<dyn Write>) -> io::Result<()> {
    let nsamples = data.len();
    writer.write_all(b"RIFF")?;
    let rsize = make_bytes::<u32>(20 + nsamples as u32); // added 20 for the rest of the header
    writer.write_all(&rsize)?; // WAVE chunk size

    // WAVE chunk
    writer.write_all(b"WAVE")?;

    // fmt chunk
    writer.write_all(b"fmt ")?;
    writer.write_all(&make_bytes::<u32>(16))?; // fmt chunk size
    writer.write_all(&make_bytes::<u16>(1))?; // format code (PCM)
    writer.write_all(&make_bytes::<u16>(1))?; // number of channels
    writer.write_all(&make_bytes::<u32>(SAMPLE_RATE))?; // sample rate
    writer.write_all(&make_bytes::<u32>(SAMPLE_RATE))?; // data rate
    writer.write_all(&make_bytes::<u16>(1))?; // block size
    writer.write_all(&make_bytes::<u16>(16))?; // bits per sample

    // data chunk
    writer.write_all(b"data")?;
    writer.write_all(&make_bytes::<u32>(nsamples as u32))?; // data chunk size

    writer.write_all(&data)?;

    writer.flush()
}

#[test]
fn test_file() {
    fn get_writer(arg: &str) -> Box<dyn Write> {
        match arg {
            "-" => Box::new(io::stdout().lock()),
            x if x.is_empty() => Box::new(io::stdout().lock()),
            file_name => Box::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(file_name)
                    .unwrap(),
            ),
        }
    }
    let mut data_content = Vec::new();
    data_content.extend_from_slice(&notable_notes::A4.audio_wave(3, &Volume::Medium));
    data_content.extend_from_slice(&notable_notes::A4.audio_wave(3, &Volume::Silent));
    data_content.extend_from_slice(&notable_notes::G0.audio_wave(3, &Volume::Low));
    // need to generate data in i16 then pass it to
    // data_content.extend_from_slice(&Note::combine(
    //     &[notable_notes::C0, notable_notes::G0, notable_notes::E0],
    //     3,
    //     &Volume::High,
    // ));
    write_wav(data_content, &mut get_writer("file.wav")).unwrap();
}
