use itertools::Itertools;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::sync::Arc;
use tinyaudio::prelude::{run_output_device, OutputDeviceParameters};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn play_sine_wave() {
    let sample_rate = 44100;
    let seconds = 2;

    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate,
        channel_sample_count: sample_rate * seconds,
    };

    let sf2 = include_bytes!("synth.sf2");
    let sound_font = Arc::new(SoundFont::new(&mut sf2.as_ref()).unwrap());

    let settings = SynthesizerSettings::new(sample_rate as i32);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count / 2];
    let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count / 2];
    synthesizer.note_on(0, 60, 100);
    synthesizer.note_on(0, 63, 100);
    synthesizer.note_on(0, 67, 100);
    synthesizer.render(&mut left[..], &mut right[..]);
    let mut left2: Vec<f32> = vec![0_f32; params.channel_sample_count / 2];
    let mut right2: Vec<f32> = vec![0_f32; params.channel_sample_count / 2];
    synthesizer.note_off(0, 63);
    synthesizer.note_on(0, 64, 100);
    synthesizer.render(&mut left2[..], &mut right2[..]);

    let device = run_output_device(params, {
        move |data| {
            left.iter()
                .interleave(right.iter())
                .chain(left2.iter().interleave(right2.iter()))
                .zip_eq(data.iter_mut())
                .for_each(|(v, d)| {
                    *d = *v;
                });
        }
    })
    .unwrap();

    Box::leak(device);
}
