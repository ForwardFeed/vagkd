# vagkd
Vagely Advanced Global Keybind Daemon (for Linux)

### What is this?
This is a keybind daemon for linux. It reacts from keys pressed on the keyboard, **globally**, on **any** Display server(X11/Wayland) and in a **more advanced/original** way.


### features:
- **Wide Linux operability:** Unlike pre-existing linux keybinders this one is based on the linux kernel and should work on any display server
(hello wayland users). downside is it requires some privileges.

- **Global keybinds :** The main reason why this program needs to have priviledges is because it works on any window.

- **More control over key patterns:** the vagk project was created to allow original way to make global keybinds and some parameters are available

- **Somewhat advanced pattern matching:** What I call pattern matching is the key interacting process humans do on the keyboard to generate a reaction from the software:
- - *Simple key pressing/release:* key up or key down.
- - *Long key pressing:* down for X amount of time.
- - *Spam key pressing:* in a span of X time a key must have been pressed X number of time

- **Combination of pattern matching:** It's recommended to combine multiple keybinds, so for example my Numpad_0 needs to be held down for 400ms which is enough to not being a simple typing and in a spam of 750ms to put down my Numpad_7 key, then the program reacts. 

- **Unordered pattern matching:** Multiples key pattern can be set for one keybind to be proc. But to save speed i've chosen to ignore the order 
of the keys pressed.\
So if you do space + Return, you can do Return + space it will work too.

- **Output to stdout:** Matching keybindings will be printed to stdout.

### How to use it:

To interact with the project as a human [vagk-tools](https://github.com/ForwardFeed/vagk-tools), its most important part being the [GUI](https://github.com/ForwardFeed/vagk-tools/tree/main/gui). Which helps creating configuration file

There isn't a way to use it as a library so far.
##### Build the project
The program is a binary,so to get it:  
```sh
#Clone the project
git clone https://github.com/ForwardFeed/vagkd
cd vagk
cargo build
# 
```

##### Run the project
vagkd need a config file being loaded to works. The GUI creates a config file in json, per default "macro-config.json"
```sh
#default way:
#this will take a configuration file called macro-config.json in the local directory
cargo run
#specific configuration way:
#the user need to specify the path of a vagkd configuration file (that should have been made with the gui)
cargo run -- --input /path/to/the/config.json
cargo run -- -i /path/to/the/config.json
```
both way will fail if no configuration file is found/given


### TODO list
- [ ] making a tutorial on how to use it
- [x] GUI
- [ ] more **advanced** matching pattern
- [ ] ordered keybinds 
- [ ] make vagk as a lib
- [ ] make a wrapper for others languages such as python for ease of use
