use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub struct InputEvent {
    pub timestamp: Instant,
    pub key_type: u16,
    pub key_code: u16,
    pub key_value: i32
}

impl InputEvent {
    pub fn from_byte(buffer: &[u8;24]) -> Result<InputEvent, ()> {
        let key_type: u16 =
            u16::from_be_bytes([buffer[17], buffer[16]]);
        let key_code: u16 =
            u16::from_be_bytes([buffer[19], buffer[18]]);
        let key_value: i32 =
            i32::from_be_bytes([buffer[23],buffer[22], buffer[21], buffer[20]]);

        // this part is just to win some computation steps, since 1/3 of the events are sync event
        // and 1/3 others are EV_MSC which i haven't found a way to exploit them
        return if (key_type | key_code) == 0 {

            //SYN_EVENT
            Err(())
        } else if (key_type | key_code) == 4 {
            //EV_MSC
            Err(())
        } else {
            Ok(InputEvent {
                timestamp: Instant::now(),
                key_type,
                key_code,
                key_value
            })
        }
    }


}
