use hound::{self, WavWriter};
use std::f32::consts::PI;
use std::{fs, i16, io};

const SAMPLE_RATE: f32 = 44100.0;
const MORSE_FREQ: f32 = 600.0;
const UNIT_TIME: f32 = 100.0 / 1000.0; // IN MS
const CHAR_TIME: f32 = 50.0 / 1000.0;

const SPEC: hound::WavSpec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
};

fn write_freq(writer: &mut WavWriter<io::BufWriter<fs::File>>, freq: f32, duration: f32) {
    let sample_count = (duration * SAMPLE_RATE).ceil() as i32;
    for t in (0..sample_count).map(|x| x as f32 / SAMPLE_RATE) {
        let sample = (t * freq * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

fn write_morse(filename: &str, morse: &str) {
    let mut writer = hound::WavWriter::create(filename, SPEC).unwrap();

    //for t in (0..sample_count).map(|x| x as f32 / SAMPLE_RATE) {
    //    let sample = (t * MORSE_FREQ * 2.0 * PI).sin();
    //    let amplitude = i16::MAX as f32;
    //    writer.write_sample((sample * amplitude) as i16).unwrap();
    //}

    for c in morse.chars() {
        // We give a margin of CHAR_TIME between dots and dashes
        // and we give 1.5 * UNIT_TIME between characters
        if c == '.' {
            write_freq(&mut writer, MORSE_FREQ, UNIT_TIME);
            write_freq(&mut writer, 0.0, CHAR_TIME);
        } else if c == '-' {
            write_freq(&mut writer, MORSE_FREQ, 3.0 * UNIT_TIME);
            write_freq(&mut writer, 0.0, CHAR_TIME);
        } else if c == ' ' {
            write_freq(&mut writer, 0.0, 1.5 * UNIT_TIME);
        } else {
            panic!("The morse code given is not valid!");
        }
    }
    write_freq(&mut writer, 0.0, UNIT_TIME); // End padding
}

fn main() {
    // -.. .- -. .. ... ...

    write_morse("morse_danish.wav", "-.. .- -. .. ... ....");
    //write_freq(&mut writer, 350.0, 1.0);
    //write_freq(&mut writer, 650.0, 1.0);
}
