use std::io::{self, Write};

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

pub fn write_wav(data: Vec<i16>, sample_rate: u32, writer: &mut dyn Write) -> io::Result<()> {
    let nsamples = data.len() * 2;
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
    writer.write_all(&make_bytes::<u32>(sample_rate))?; // sample rate
    writer.write_all(&make_bytes::<u32>(sample_rate))?; // data rate
    writer.write_all(&make_bytes::<u16>(2))?; // block size
    writer.write_all(&make_bytes::<u16>(16))?; // bits per sample

    // data chunk
    writer.write_all(b"data")?;
    writer.write_all(&make_bytes::<u32>(nsamples as u32))?; // data chunk size
    for half_word in data {
        writer.write_all(&make_bytes(half_word as u16))?;
    }

    writer.flush()
}

#[test]
fn test_file() {
    use crate::polyphonia::{notable_notes, Note, Amplitude, SAMPLE_RATE};
    use std::fs::OpenOptions;
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
    data_content.extend_from_slice(&notable_notes::A4.audio_wave(3.0, &Amplitude::Medium));
    data_content.extend_from_slice(&notable_notes::A4.audio_wave(3.0, &Amplitude::Silent));
    data_content.extend_from_slice(&notable_notes::A4.audio_wave(3.0, &Amplitude::Low));
    data_content.extend_from_slice(&Note::combine(
        &[notable_notes::A4, notable_notes::C4_SH, notable_notes::E0],
        3.0,
        &Amplitude::Medium,
    ));
    write_wav(
        data_content,
        SAMPLE_RATE,
        &mut get_writer("file_static.wav"),
    )
    .unwrap();
}
