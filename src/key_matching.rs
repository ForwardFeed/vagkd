
//this trait will in function of the object make the correct matching it's a bit hard to understand even to myself tbh
pub trait KeyMatching{
    fn key_matching(&self, key_code: u16, key_value: u32) -> bool;
}


struct Simple{
    key_code: u16,
    key_value: u32,
}

//simply compare the config with what kernel say, really simple
impl KeyMatching for Simple{
    fn key_matching(&self, key_code: u16, key_value: u32) -> bool {
        if self.key_code == key_code && self.key_value == key_value{
            return true
        }
        return false
    }
}

//to explain in short, this function
//in our config file we have humans understandable key_states such as release, press, hold and tons of other options
//so what i do is to transform thoses Strings into key value, it's more of a key_code_function to be franc
//i think that for the sake of the develloper it's better to make matching with 1 refering to a certain function/type than words
pub(crate) fn trans_key_state_to_key_value(cfg_key_state: String)->u32{//u32 because since it's related to the 4bytes keyvalue of the kernel input event system
    //note i rather u32 than i32 because i don't like to deal with negative numbers and in the end it's just 4bytes

    //This where you can link a key_state to a key value
    //for the simple struct example, the function will simply match the key code sent by the kernel and the one converted
    //but for other keymatching more complex. 2 will mean check if it's pressed for more than well 2 seconds or milliseconds or log 2 milliseconds
    return match cfg_key_state.as_str() {
        "press" => 1,
        ">" => 1,
        "release" => 0,
        "<" => 0,
        "hold" => 2,
        "_" => 2,
        _ => 1,
    }
}

pub(crate) fn trans_key_state_to_key_function(cfg_key_state: String)->u8{//i have a mental illness that makes me liking unsigned chars, if people managed to get soft block by this u8 i'm impressed
    return match cfg_key_state.as_str() {
        "press" => 1,
        ">" => 1,
        "release" => 1,
        "<" => 1,
        "hold" => 1,
        "_" => 1,
        _ => 1,
    }
}


//if you want to add new keymatching function you can put uses of the implementation here
pub fn new(cfg_key_code: u16, cfg_key_state: String) -> Box<dyn KeyMatching>{
    let cfg_key_value = trans_key_state_to_key_value(cfg_key_state.clone());
    let code= trans_key_state_to_key_function(cfg_key_state);
    return match code{
        1 => Box::new(Simple{key_code: cfg_key_code, key_value: cfg_key_value}),
        _ => Box::new(Simple{key_code: cfg_key_code, key_value: cfg_key_value}),//better make a panic tbh
    }
}