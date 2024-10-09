use hound::{self, WavWriter};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::io::{Cursor, Seek, Write};

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

fn write_freq<W>(writer: &mut WavWriter<W>, freq: f32, duration: f32)
where
    W: Write + Seek,
{
    let sample_count = (duration * SAMPLE_RATE).ceil() as i32;
    for t in (0..sample_count).map(|x| x as f32 / SAMPLE_RATE) {
        let sample = (t * freq * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

/// Writes given morse code into a `hound:WavWriter`
///
/// # Arguments
///
/// * `pause_between` -> The duration in miliseconds.
/// - `char_time` -> The pause between characters, e.g .- *wait* -..
/// - `words_time` -> The pause between words, or equivalently at '/' characters.
pub fn write_morse<W>(
    writer: &mut WavWriter<W>,
    morse: &str,
    pause_between_char_time: f32,
    pause_between_words_time: f32,
) where
    W: Write + Seek,
{
    for c in morse.chars() {
        if c == '.' {
            write_freq(writer, MORSE_FREQ, UNIT_TIME);
            write_freq(writer, 0.0, 0.5 * UNIT_TIME); // Silence after dot
        } else if c == '-' {
            write_freq(writer, MORSE_FREQ, 3.0 * UNIT_TIME);
            write_freq(writer, 0.0, 0.5 * UNIT_TIME); // Silence after dash
        } else if c == ' ' {
            // Pause between Characters
            write_freq(writer, 0.0, pause_between_char_time);
        } else if c == '/' {
            // Pause between Words
            write_freq(writer, 0.0, 2.0 * pause_between_words_time);
        } else {
            panic!("The morse code given is not valid!");
        }
    }
    write_freq(writer, 0.0, UNIT_TIME); // End padding
}

/// Writes given morse code into a wav file
///
/// # Arguments
///
/// * `pause_between` -> The duration in miliseconds.
/// - `char_time` -> The pause between characters, e.g .- *wait* -..
/// - `words_time` -> The pause between words, or equivalently at '/' characters.
pub fn write_morse_to_file(
    filename: &str,
    morse: &str,
    pause_between_char_time: f32,
    pause_between_words_time: f32,
) {
    // Convert from MS;
    let pause_between_char_time = pause_between_char_time / 1000.0;
    let pause_between_words_time = pause_between_words_time / 1000.0;

    let mut writer = hound::WavWriter::create(filename, SPEC).unwrap();
    write_morse(
        &mut writer,
        morse,
        pause_between_char_time,
        pause_between_words_time,
    );

    writer.finalize().unwrap();
}

/// Writes given morse code into a buffer that is returned
///
/// # Arguments
///
/// * `pause_between` -> The duration in miliseconds.
/// - `char_time` -> The pause between characters, e.g .- *wait* -..
/// - `words_time` -> The pause between words, or equivalently at '/' characters.
pub fn write_morse_in_memory(
    morse: &str,
    pause_between_char_time: f32,
    pause_between_words_time: f32,
) -> Vec<u8> {
    let mut buffer = Vec::new(); // Create a buffer to hold the WAV data
    {
        let mut writer = hound::WavWriter::new(Cursor::new(&mut buffer), SPEC).unwrap();
        write_morse(
            &mut writer,
            morse,
            pause_between_char_time,
            pause_between_words_time,
        );
    }

    buffer
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
