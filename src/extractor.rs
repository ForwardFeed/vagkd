use std::fs::File;
use std::io::Read;
use crate::input_event::Input_Event;


pub struct Extractor{
    file: File,
}

impl Extractor {
    pub fn new(event_file: String) -> Extractor {
        let file = open_event_file(event_file);
        Extractor{
            file,
        }
    }

    pub fn last_keyboard_event(&mut self) -> Input_Event {
        //loop expected to be 3 time max because some events are just to tell there is an event
        // incoming or some non-necessary stuff to the program
        let mut buffer: [u8;24] = [0; 24];
        loop{
            self.file.read(&mut buffer[..]).unwrap();
            match Input_Event::from_byte(&buffer) {
                Ok(x) => return x,
                Err(()) => continue
            }

        }
    }


}

fn open_event_file(event_file: String) -> File {

    let file = File::open(event_file);
    match file {
        Ok(file) => return file,
        Err(e) => panic!("Error while opening keyboard event file : {}", e),
    };
}