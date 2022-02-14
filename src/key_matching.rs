
//this trait will in function of the object make the correct matching it's a bit hard to understand even to myself tbh
pub trait KeyMatching{
    fn key_matching(&mut self, key_code: u16, key_value: u32) -> bool;
    fn reset(&mut self);
}


struct Simple{
    key_code: u16,
    key_value: u32,
}

impl Simple {
    fn new(key_code: u16, keybind_type: String) -> Simple{
        let key_bind_str = keybind_type.as_str();
        return match key_bind_str{
             "press" => Simple{key_code, key_value: 1},
             ">" => Simple{key_code, key_value: 1},
             "release" => Simple{key_code, key_value: 0},
             "<" => Simple{key_code, key_value: 0},
             "hold" => Simple{key_code, key_value: 2},
             "_" => Simple{key_code, key_value: 2},
            _ => panic!("Wrong keybind_type got : {} ", keybind_type)
        };
    }
}

//simply compare the config with what kernel say, really simple
impl KeyMatching for Simple{
    fn key_matching(&mut self, key_code: u16, key_value: u32) -> bool {
        if self.key_code == key_code && self.key_value == key_value{
            return true
        }
        return false
    }

    fn reset(&mut self) {
        //nothing to do
    }
}
/*
This Struct will be be for press that have to be X time long before matching
The idea is when a 1 is matched it init the
 */

use std::time::{Duration, Instant};
use crate::config_loader::KeyStates;

struct LongPress{
    key_code: u16,
    press_duration: Duration,
    start_timer: Instant,
}

/*
i don't use the keybind_type for now maybe in the future?
 */

impl LongPress {
    fn new(key_code: u16, keybind_type: String, press_duration: u64) ->LongPress{

        let key_bind_str = keybind_type.as_str();
        match key_bind_str{
            "longpress" => (),
            "long-press" => (),
            "long_press" => (),
            _ => panic!("Wrong keybind_type expected \"longpress\" got : {} ", keybind_type)
        };

        LongPress {
            key_code,
            press_duration: Duration::from_millis(press_duration),
            start_timer: Instant::now()
        }
    }
}

impl KeyMatching for LongPress{
    fn key_matching(&mut self, key_code: u16, key_value: u32) -> bool {
        if self.key_code == key_code{
            //we press down the keyboard key
            if key_value == 1{
                self.start_timer=Instant::now();
            }
            /* it's else 2 a kernel hold down or a release
               Here is a tricky part, this system cannot hold very precise time length
               because at each "kernel tic" this function will be invoked and kernel tics aren't that frequent.
               But to fix this i would need a total rewrite
             */
            else{
                let now = Instant::now();
                // just check if the time elapsed between the first press and the second press is more than the time we wanted
                if now.duration_since(self.start_timer) > self.press_duration {
                    return true;
                }
                //the press wasn't long enough
                return false;
            }
        }
        return false
    }

    fn reset(&mut self) {
        self.start_timer = Instant::now();
    }
}

/*
This structure will handle keymatching for spam pressing of one key during a specific time
 */
struct SpamPress{
    key_code: u16,
    spam_press_time_span: Duration,
    start_timer: Instant,
    cfg_count_press: u16,
    current_count_press: u16,
}

impl SpamPress{

    /*
    i don't use the keybind_type for now maybe in the future?
     */
    fn new(key_code: u16, keybind_type: String, spam_press_time_span: u64, repetition: u16 ) ->SpamPress{

        let key_bind_str = keybind_type.as_str();
        match  key_bind_str {
            "spampress" => (),
            "spam-press" => (),
            "spam_press" => (),
            _ => panic!("Wrong keybind_type expected \"spampress\"  got : {} ", keybind_type)
        }

        SpamPress {
            key_code,
            spam_press_time_span: Duration::from_millis(spam_press_time_span),
            start_timer: Instant::now(),
            cfg_count_press: repetition,
            current_count_press: 0,
        }
    }
}

impl KeyMatching for SpamPress {
    fn key_matching(&mut self, key_code: u16, key_value: u32) -> bool {
        if self.key_code == key_code{
            //we press down the keyboard key
            if key_value == 1{
                //If it's the first count down start the timer
                if self.current_count_press == 0 {
                    self.start_timer = Instant::now();
                    //start the increment
                    self.current_count_press+=1;
                    //println!("init: {}", self.cfg_count_press.clone())
                }
                else {
                    //simply increment
                    self.current_count_press+=1;
                    //println!("current: {}", self.current_count_press.clone());
                }
            }
            //when we finally release we check if the count has been reached or not
            if key_value==0{
                //Before anything just calc if we pressed into enough time?
                let now = Instant::now();
                // just check if the time elapsed between the first press and the second press is more than the time we wanted
                if now.duration_since(self.start_timer) > self.spam_press_time_span {
                    //reset time baby?
                    self.current_count_press = 0;
                    return false;
                }
                //did we reached the number of count?
                //the >= here is to prevent a bug if someone put zero press in the config file
                if self.current_count_press >= self.cfg_count_press{
                    //yes we did, we just released the last one
                    self.current_count_press = 0;
                    return true;
                }
                return false;
            }
            //if the key_value is 2 we don't care
        }
        return false
    }

    fn reset(&mut self) {
        //nothing to do
    }
}

/*

 */
pub fn new(cfg_key_code: u16, cfg_key_state: KeyStates) -> Box<dyn KeyMatching>{

    match cfg_key_state {
        KeyStates::Simple{keybind_type} => Box::new(Simple::new(cfg_key_code, keybind_type)),
        KeyStates::LongPress{keybind_type,press_duration} => Box::new(LongPress::new(cfg_key_code,keybind_type, press_duration)),
        KeyStates::SpamPress{keybind_type, spam_press_time_span, repetition} => Box::new(SpamPress::new(cfg_key_code,keybind_type,spam_press_time_span ,repetition)),
    }

}