# vagk
For (Linux) Very Advanced Global Keybind (daemon) 

### What is this?
This is a keybinder or a macro daemon for linux . it reacts from keys you pressed on your keyboard and with different
behavior,
For example holding a key for a set amount of time or spamming a key in a set amount of time.\
Multiples key patterns can be set for one keybind to be proc. But to save speed i've chosen to ignore the order 
of the keys pressed.\
So if you do space + Return, you can do Return + space it will work.
### features:
- Unlike pre-existing linux keybinders this one is based on the linux kernel and should work on any display server
(hello wayland users). downside is it requires some privileges since it behave like a keylogger.
- You can configure it using a RON config file 
  - `-c` to set the config file to us
  - `-g` to generate a config file using the terminal
- Matching keybindings will be outputed to stdout.

### TODO list
- [ ] making a tutorial on how to use it
- [ ] GUI
- [ ] modify config
- [ ] more **advanced** matching pattern
- [ ] ordering keybinds 
- [ ] make a wrapper for others languages such as python for ease of use
