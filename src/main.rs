use morse_translator::{to_morse, write_morse};

fn main() {
    let text = to_morse("danish ghaisan");

    write_morse("danish.wav", &text);
}
