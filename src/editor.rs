use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        loop {
            match read() {
                Ok(Event::Key(key_event)) => {
                    println!("{key_event:?} \r");
                    if let KeyCode::Char(c) = key_event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => println!("Error : {err}"),
                _ => (),
            }
        }

        disable_raw_mode().unwrap();
    }
}
