# morse_rs

A simple Morse code translator written in Rust, with the ability to write Morse code to WAV files.

## Features

- Translate text to Morse code and vice versa
- Generate WAV files from Morse code sequences

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
morse_rs = "0.1.2"
```

## Usage

Converts a string to morse then writes the encoded message into a wav file.

```rust
use morse_rs::{to_morse, write_morse};

fn main() {
    let my_message = "attack at noon";
    let morse = to_morse(my_message);

    write_morse("my_message.wav", &morse);
}
```

We can also write the morse into a buffer in memory

```rust
use morse_rs::{to_morse, write_morse};

fn main() {
    let my_message = "attack at noon";
    let morse = to_morse_inmemory(my_message);

    write_morse("my_message.wav", &morse);
}
```

## TODO
- Decode morse from sound
- Add proper rust like error return types
