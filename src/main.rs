use std::io::Read;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();

    for b in std::io::stdin().bytes() {
        match b {
            Ok(u8_value) => {
                let char_value = u8_value as char;

                if char_value.is_control() {
                    println!("Binary {0:08b}, ASCII: {0:#3} \r", u8_value);
                } else {
                    println!(
                        "Binary {0:08b}, ASCII: {0:#3} Character {1:#?}\r",
                        u8_value, char_value
                    );
                }

                if char_value == 'q' {
                    break;
                }
            }
            Err(err) => println!("Error : {}", err),
        }
    }
    disable_raw_mode().unwrap();
}
