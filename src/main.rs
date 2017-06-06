use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                println!("{}", generate_morse_string(buffer));
            }
        }
        Err(error) => println!("error: {}", error),
    }
}

/// Returns a string of morse code from a string of ascii characters
///
/// # Arguments
/// * `msg` - A message that is to be translated to morse code
///
/// # Example
/// ```
/// generate_morse_string("This is a test string");
/// ```
fn generate_morse_string(msg: String) -> String {
    let mut morse = String::new();

    for x in msg.chars() {
        morse.push_str(morse_code(x));
    }

    morse
}

/// Returns the Internation Standard morse code equivalent of a character
/// # Arguments
/// * `character` - A ascii character
/// # Example
/// ```
/// morse_code('a');
/// ```
fn morse_code<'a>(character: char) -> &'a str {
    match character.to_lowercase().next().unwrap() {
        'a' => "._",
        'b' => "_...",
        'c' => "_._",
        'd' => "_..",
        'e' => ".",
        'f' => ".._.",
        'g' => "__.",
        'h' => "....",
        'i' => "..",
        'j' => ".___",
        'k' => "_._",
        'l' => "._..",
        'm' => "__",
        'n' => "_.",
        'o' => "___",
        'p' => ".__.",
        'q' => "__._",
        'r' => "._.",
        's' => "...",
        't' => "_",
        'u' => ".._",
        'v' => "..._",
        'w' => ".__",
        'x' => "_.._",
        'y' => "_.__",
        'z' => "__..",

        '0' => "_____",
        '1' => ".____",
        '2' => "..___",
        '3' => "...__",
        '4' => "...._",
        '5' => ".....",
        '6' => "_....",
        '7' => "__...",
        '8' => "___..",
        '9' => "____.",

        ' ' => "...",
        _ => ""
    }
}
