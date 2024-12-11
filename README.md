# morse_rs

A simple Morse code translator written in Rust, with the ability to write Morse code to WAV files.

## Features

- Translate text to Morse code and vice versa
- Generate WAV files from Morse code sequences

## Usage

Converts a string to morse then writes the encoded message into a wav file, specifying the pause times.

```rust
use morse_rs::{to_morse, write_morse_to_file};

fn main() {
    let my_message = "attack at noon";
    let morse = to_morse(my_message);

    write_morse_to_file("my_message.wav", &morse, 150.0, 200.0);
}

```

We can also write the morse into a buffer in memory, doing so allows the use of transferring data with WebAssembly.

```rust
use morse_rs::{to_morse, write_morse_in_memory};

#[wasm_bindgen]
pub fn generate_morse_sound(s: String) -> Vec<u8> {
    let encoded = to_morse(&s);
    let sound = write_morse_in_memory(encoded, 150.0, 200.0);

    sound
}

```

## TODO
- Decode morse from sound
- Solve the Riemann Hypothesis

