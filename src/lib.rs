use hound::{self, WavWriter};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::{fs, i16, io};

const SAMPLE_RATE: f32 = 44100.0;
const BITS_PER_SAMPLE: u16 = 16;
const MORSE_FREQ: f32 = 600.0;
const UNIT_TIME: f32 = 100.0 / 1000.0; // IN MS

const SPEC: hound::WavSpec = hound::WavSpec {
    channels: 1,
    sample_rate: SAMPLE_RATE as u32,
    bits_per_sample: BITS_PER_SAMPLE,
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

pub fn to_morse(text: &str) -> String {
    let char_to_morse: HashMap<char, &str> = HashMap::from([
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('0', "-----"),
    ]);

    let code = text.to_lowercase();
    let mut ret = String::new();

    let chars: Vec<char> = code.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = if i < chars.len() - 1 {
            Some(chars[i + 1])
        } else {
            None
        };

        if c == ' ' {
            if let (Some(p), Some(n)) = (prev, next) {
                if char_to_morse.contains_key(&p) && char_to_morse.contains_key(&n) {
                    ret += "/ ";
                }
            }
        } else if let Some(encoded) = char_to_morse.get(&c) {
            ret += encoded;
            ret += " ";
        }
    }

    ret.trim_end().to_string()
}

pub fn write_morse(filename: &str, morse: &str) {
    let mut writer = hound::WavWriter::create(filename, SPEC).unwrap();
    for c in morse.chars() {
        // We give a margin of 0.5 * UNIT_TIME between dots and dashes
        // give 1.5 * UNIT_TIME between characters
        // and give 2 * UNIT_TIME between words
        if c == '.' {
            write_freq(&mut writer, MORSE_FREQ, UNIT_TIME);
            write_freq(&mut writer, 0.0, 0.5 * UNIT_TIME);
        } else if c == '-' {
            write_freq(&mut writer, MORSE_FREQ, 3.0 * UNIT_TIME);
            write_freq(&mut writer, 0.0, 0.5 * UNIT_TIME);
        } else if c == ' ' {
            write_freq(&mut writer, 0.0, 1.5 * UNIT_TIME);
        } else if c == '/' {
            write_freq(&mut writer, 0.0, 2.0 * UNIT_TIME);
        } else {
            panic!("The morse code given is not valid!");
        }
    }
    write_freq(&mut writer, 0.0, UNIT_TIME); // End padding
}

pub fn from_morse(morse: &str) -> String {
    let morse_to_char: HashMap<&str, char> = HashMap::from([
        (".-", 'a'),
        ("-...", 'b'),
        ("-.-.", 'c'),
        ("-..", 'd'),
        (".", 'e'),
        ("..-.", 'f'),
        ("--.", 'g'),
        ("....", 'h'),
        ("..", 'i'),
        (".---", 'j'),
        ("-.-", 'k'),
        (".-..", 'l'),
        ("--", 'm'),
        ("-.", 'n'),
        ("---", 'o'),
        (".--.", 'p'),
        ("--.-", 'q'),
        (".-.", 'r'),
        ("...", 's'),
        ("-", 't'),
        ("..-", 'u'),
        ("...-", 'v'),
        (".--", 'w'),
        ("-..-", 'x'),
        ("-.--", 'y'),
        ("--..", 'z'),
        (".----", '1'),
        ("..---", '2'),
        ("...--", '3'),
        ("....-", '4'),
        (".....", '5'),
        ("-....", '6'),
        ("--...", '7'),
        ("---..", '8'),
        ("----.", '9'),
        ("-----", '0'),
        ("/", ' '),
    ]);

    let mut look = String::new();
    let mut ret = String::new();

    for c in morse.chars() {
        if c == ' ' {
            if let Some(decoded) = morse_to_char.get(&look[..]) {
                ret.push(*decoded);
                look = "".to_string();
            }
        } else {
            look.push(c);
        }
    }

    if !look.is_empty() {
        if let Some(decoded) = morse_to_char.get(&look[..]) {
            ret.push(*decoded);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_morse() {
        let encoded = to_morse("This iS a Message 01 written in morse!");
        assert_eq!(encoded, "- .... .. ... / .. ... / .- / -- . ... ... .- --. . / ----- .---- / .-- .-. .. - - . -. / .. -. / -- --- .-. ... .")
    }

    #[test]
    fn test_weird_input_to_morse() {
        let encoded = to_morse("!@(@*$)(!*@)($*)!@ hey this is good char now )!(@*)][][]][]");
        assert_eq!(encoded, ".... . -.-- / - .... .. ... / .. ... / --. --- --- -.. / -.-. .... .- .-. / -. --- .--")
    }

    #[test]
    fn test_from_morse() {
        let decoded = from_morse("-.. .- -. .. ... .... / --. .... .- .. ... .- -. / .--. ..- - . .-. .- / .- .... -- .- -.. ..");
        assert_eq!(decoded, "danish ghaisan putera ahmadi");
    }
}
