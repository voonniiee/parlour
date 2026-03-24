use std::{fs::OpenOptions, path::Path, sync::Arc};

use realfft::{RealFftPlanner, RealToComplex, num_complex::Complex};
use symphonia::{
    core::{
        audio::SampleBuffer,
        codecs::{CodecRegistry, Decoder, DecoderOptions},
        formats::{FormatOptions, FormatReader},
        io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};

pub struct Sound {
    pub data: Vec<f32>,

    pub frequency_rep: Vec<Vec<Complex<f32>>>,
}

impl Sound {
    pub fn from_file(path: impl AsRef<Path>, dft_window_len: usize) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            //TODO: NEVER FUCKING USE UNWRAP
            .unwrap();

        let codecs = get_codecs();
        let mut reader = Self::do_symphonia_stuff(Box::new(file));

        let track = reader.tracks()[0].clone();

        let mut decoder = codecs
            .make(&track.codec_params, &DecoderOptions::default())
            .unwrap();

        let mut data = Vec::new();

        while let Ok(packet) = reader.next_packet() {
            let decoded = decoder.decode(&packet).unwrap();

            let mut buffer = SampleBuffer::new(packet.dur, decoded.spec().clone());

            buffer.copy_planar_ref(decoded);

            data.extend_from_slice(buffer.samples());
        }

        let frequency_rep = Self::stft(&data, dft_window_len);

        Self {
            data,
            frequency_rep,
        }
    }

    fn stft(data: &Vec<f32>, dft_window_len: usize) -> Vec<Vec<Complex<f32>>> {
        let fft = RealFftPlanner::<f32>::new().plan_fft_forward(dft_window_len);

        let mut scratch = fft.make_scratch_vec();
        let mut output = fft.make_output_vec();

        let mut out = Vec::new();

        for chunk in data.chunks(dft_window_len) {
            let mut chunk = chunk.to_vec();
            chunk.resize(dft_window_len, 0f32);

            fft.process_with_scratch(&mut chunk, &mut output, &mut scratch)
                .unwrap();

            let scaling = 1f32 / (dft_window_len as f32).sqrt();

            // i *think* this is what you're supposed to do?? not sure.
            for val in &mut output {
                *val *= scaling;
            }

            out.push(output.clone());
        }

        out
    }

    //TODO: make a lot of this symphonia stuff persistent (maybe).
    fn do_symphonia_stuff(source: Box<dyn MediaSource>) -> Box<dyn FormatReader> {
        let probe = get_probe();

        //TODO: figure out if tuning source stream options matters much
        let stream = MediaSourceStream::new(source, MediaSourceStreamOptions::default());

        let format = probe
            .format(
                &Hint::new(),
                stream,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            )
            .unwrap();

        format.format
    }
}
