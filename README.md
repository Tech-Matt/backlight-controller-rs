# backlight-controller-rs
A simple backlight controller for intel_backlight in Rust. This code makes use of DBus system bus in Linux.
Through the bus a user can make some calls like `SetBrightness` which usually require root, and thus 
modify the brightness. The code can then be used from a *HotKey Daemon* Like **sxhkd** to bind the command to keyboard shortcuts (a setup guide is provided below). 

## Command line options
The script provides 3 basic commands:
- `brightrs -i [VAL]` increases the screen brightness by 5%;
- `brightrs -d [VAL]` decreases screen brightness by 5%;
- `brightrs -s [VAL]` set a specific value in the possible range.

<!-- remember to update the instructions when you make changes** -->

## HotKey Deamon Setup