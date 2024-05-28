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
        if let Err(e) = self.repl() {
            panic!("{e:#?}");
        }
        println!("Goodbye. \r");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        loop {
            if let Event::Key(key_event) = read()? {
                println!("{key_event:?} \r");
                if let KeyCode::Char(c) = key_event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}
