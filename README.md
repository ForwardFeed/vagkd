# vagk
Very Advanced Global Keybind Daemon (for Linux)

### What is this?
This is a keybinder or a macro daemon for linux .It reacts from keys pressed on the keyboard (I call this keybinding) and with different
behaviors or "keys patterns" as I will say from now.
For example holding a key for a set amount of time or spamming a key in a set amount of time.\
Multiples key pattern can be set for one keybind to be proc. But to save speed i've chosen to ignore the order 
of the keys pressed.\
So if you do space + Return, you can do Return + space it will work too.
### features:
- Unlike pre-existing linux keybinders this one is based on the linux kernel and should work on any display server
(hello wayland users). downside is it requires some privileges.
- You can configure it using a RON config file 
  - `-i --input` to set the config file to use
- Matching keybindings will be printed to stdout.


The config file is in .json extension, i took JSON for the GUI [[ insert link ]], and you should not be manipulating this with a text editor, there is some tools you should use [[ insert link ]].

### ⚠️The following is not up to date⚠️
### Let's Decompose the config file to understand
the `event_path` is the absolute path for finding the keyboard file, you can find it using the HelperFindKeyboard.sh
or by using this bash command :`grep -E  'Handlers|EV=' /proc/bus/input/devices |  grep -B1 'EV=120013' | grep -Eo 'event[0-9]+`

```json
general_parameters: (
event_path: "/dev/input/event3",
),
```

This is an array of Keybinds, Each keybind will hold multiples subkeybinds object (see below)\
- `name` is the string of characters that will be printed in the stdout once all subkeybind will have matched.\
- `timer_threshold` is a span of time in milliseconds which is the max amount of time between 
the first and last keybind to match, if the time elapsed is larger than the timer_threshold then nothing will match\
Note that there is a 100~ millescond error margin due to reasonable polling time (it could be in the settings one day)

```json
keybinds: [
        (
        /// there is a subkeybind object here see next 
        "name": "simple click",
        "timer_threshold": 1500,
        )
```
Then the subkeybind objects that are contained by the previous object\
- `key_code` it's a code corresponding to a key, see below for more help
- `key_state` it's an object with detail of the matching pattern
- `keybind_type` Understand it by looking at the exemple i've //commented explanations
```json
sub_keybinds: [
    (
        "key_code": 73, //For me it's my KeyPad_8 but it's because my keyboard is this way
        "key_state": (
            keybind_type: "press", //simple, it's when you press the key
        )
    ),
    (
        key_code: 72, // my KeyPad_7 key
        key_state: (
            keybind_type: "spampress", // spam the key to create the match of the keybind
            spam_press_time_span: 2000, // in millesconds, the time between the first and last key hit must not exceed
            repetition: 4, //the number of time you have to hit
        )
    ),
    (
        key_code: 71, // my KeyPad_9 key
        key_state: (
            keybind_type: "longpress", //press a hit and do not relax it
            press_duration: 750, // how long have
        )
    ),
]
```
To find the keycode the best is just to just use the --generate command utility

### TODO list
- [ ] making a tutorial on how to use it
- [x] GUI
- [ ] modify config with the CLI
- [ ] more **advanced** matching pattern
- [ ] ordered keybinds 
- [ ] make a wrapper for others languages such as python for ease of use
