use std::default::default;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::time::Instant;
use crate::input_event::InputEvent;
use crate::config_loader::CfgKeybind;

pub struct ListenConvert{
    input_receiver: Receiver<InputEvent>,
    keybind_sender: Sender<CfgKeybind>,
}

impl ListenConvert {
    pub fn new(input_receiver: Receiver<InputEvent>,
               keybind_sender: Sender<CfgKeybind>) -> ListenConvert{
        ListenConvert{
            input_receiver,
            keybind_sender
        }
    }

    pub fn start(&mut self){

        let mut has_started = false;
        let mut time_start: Instant = Default::default();
        let mut keybind =  CfgKeybind{
            sub_keybinds: vec![],
            name: "default".to_string(),
            timer_threshold: 0
        };
        // i will store all event in a vec
        // and will proceed to parse all data in a function after
        let mut event_vec: Vec<InputEvent> = vec![];

        loop{
            let last_input = loop {
                //! TODO! the timeout polling needs to be somewhat standard to one point in time.
                break match
                self.input_receiver.recv_timeout(std::time::Duration::from_millis(75)) {
                    Ok(x) => {x}
                    Err(..) => {
                        continue
                    }
                };
            };
            //if we received the first key event we start the count
            if ! has_started{
                has_started = true;
                time_start = Instant::now();
            }
            event_vec.push(last_input);

            //and to one point we will send
            keybind.timer_threshold = time_start.elapsed().as_millis() as u64;
            self.keybind_sender.send(keybind.clone()).expect("error couldn't send message");
        }
    }

    fn event_parser(event_vec: Vec<InputEvent> )-> CfgKeybind{
    }
}