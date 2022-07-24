use std::fs::File;
use std::io::Read;
use std::time::Duration;
use crate::input_event::InputEvent;
use std::{thread, time};
use std::sync::mpsc::{channel, Receiver, Sender, RecvTimeoutError};

struct KeyboardEventReader {
    file: File,
    sender: Sender<InputEvent>,
}

impl KeyboardEventReader {
    pub fn start(&mut self){
        loop{
            let mut buffer: [u8; 24] = [0; 24];
            loop {
                match self.file.read(&mut buffer[..]).unwrap() {
                    24 => { break; }
                    _ => {}
                }
            }
             match InputEvent::from_byte(&buffer) {
                Ok(x) => {
                    if x.key_value == 2 {
                        continue // ignore when the kernel say the key is being held
                        //it confuse my code more than anything else
                    } else {
                        //go value let's communicate it
                        //we loop until we can communicate
                        loop {
                            match self.sender.send(x){
                                Ok(_) => {break}
                                Err(_) => {continue}
                            }
                        }
                    }
                },
                Err(()) => continue
            }
        }
    }
}


pub struct Extractor{
    _ev_thread_handle: thread::JoinHandle<()>,
    receiver: Receiver<InputEvent>,

}

impl Extractor {

    pub fn new(event_file: String) -> Extractor {
        let file = open_event_file(event_file);
        let (sender, receiver) = channel();
        let mut event_reader = KeyboardEventReader {
            file,
            sender,
        };
        let ev_thread_handle= thread::spawn(move ||{
            event_reader.start();
        });
        Extractor {
            _ev_thread_handle: ev_thread_handle,
            receiver
        }
    }

    pub fn last_keyboard_event(&mut self) -> InputEvent {
        //loop expected to be 3 time max because some events are just to tell there is an event
        // incoming or some non-necessary stuff to the program
        loop {
            loop {
                match self.receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(last_event) => {
                        return last_event
                    },
                    Err(RecvTimeoutError::Timeout) => {
                        return InputEvent {
                            timestamp: time::Instant::now(),
                            key_type: 0,
                            key_code: 0,
                            key_value: 0
                        }
                    },
                    Err(RecvTimeoutError::Disconnected) => panic!("reading file thread crashed"),
                }
            }
        };
    }
}

fn open_event_file(event_file: String) -> File {
    let file = File::open(event_file.clone());
    match file {
        Ok(file) => return file,
        Err(e) => panic!("Error while opening {} : {}",event_file, e),
    };
}

// a special KeyboardEventReader for the generate config needs
pub struct ConfigKeyboardEventReader {
    event_file: String
}

impl ConfigKeyboardEventReader {
    pub fn new(event_file: String)-> ConfigKeyboardEventReader {
        ConfigKeyboardEventReader {
            event_file,
        }
    }

    pub fn last_keycode_pressed(&mut self) -> u16{
        let mut buffer: [u8; 24] = [0; 24];
        let mut file = open_event_file(self.event_file.clone());
        loop {
            loop {
                match file.read(&mut buffer[..]).unwrap() {
                    24 => { break; }
                    _ => {}
                }
            }
            match InputEvent::from_byte(&buffer) {
                Ok(x) => {
                    return x.key_code;
                },
                Err(()) => continue
            }
        }
    }
}