use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub struct Input_Event{
    pub timestamp: std::time::Duration,
    pub key_type: u16,
    pub key_code: u16,
    pub key_value: i32
}

impl Input_Event {
    pub fn from_byte(buffer: &[u8;24]) -> Result<Input_Event, ()> {
        let time_sec: u32 =
            u32::from_be_bytes([buffer[3],buffer[2], buffer[1], buffer[0]]);
        let time_microsec: u32 =
            u32::from_be_bytes([buffer[11],buffer[10], buffer[9], buffer[8]]);
        let key_type: u16 =
            u16::from_be_bytes([buffer[17], buffer[16]]);
        let key_code: u16 =
            u16::from_be_bytes([buffer[19], buffer[18]]);
        let key_value: i32 =
            i32::from_be_bytes([buffer[23],buffer[22], buffer[21], buffer[20]]);

        let timestamp = Duration::from_secs(time_sec as u64) +
            Duration::from_micros(time_microsec as u64);
        let x = Input_Event{
            timestamp,
            key_type,
            key_code,
            key_value
        };
        return match x.is_valuable() {
            true => Ok(x),
            false => Err(()),
        }
    }
    // this function is just to win some computation steps, since 1/3 of the events are sync event
    // and 1/3 others are EV_MSC which i haven't found a way to exploit them
     fn is_valuable(&self) -> bool{
        return if (self.key_type | self.key_code) == 0 {
            //SYN_EVENT
            false
        } else if (self.key_type | self.key_code) == 4 {
            //EV_MSC
            false
        } else {
            true
        }
    }
}
