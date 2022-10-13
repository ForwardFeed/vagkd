# vagkd
Very Advanced Global Keybind Daemon (for Linux)

### What is this?
This is a keybind daemon for linux. It reacts from keys pressed on the keyboard (I call this keybinding) and with different
behaviors or "keys patterns" as I will say from now.
For example holding a key for a set amount of time or spamming a key in a set amount of time.\
Multiples key pattern can be set for one keybind to be proc. But to save speed i've chosen to ignore the order 
of the keys pressed.\
So if you do space + Return, you can do Return + space it will work too.
### features:
- **Wide Linux operability:** Unlike pre-existing linux keybinders this one is based on the linux kernel and should work on any display server
(hello wayland users). downside is it requires some privileges.

- **More control over key patterns:** the vagk project was created to allow original way to make global keybinds.

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
