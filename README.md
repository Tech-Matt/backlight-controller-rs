# backlight-controller-rs
A simple backlight controller for Intel backlight in Rust

## (Un)Technical Overview
> I'll be extremely un-technical here, cause my knowledge on the topic is extremely poor

Apparently [DBus](https://freedesktop.org/wiki/Software/dbus/), a linux cool communication bus, both used by system processes and application processes as an IPC method, provides some cool *interfaces*, to which a user may interact with. This comes in handy, cause
changing the backlight in Linux is a **privileged** action and thus you need to be root or use *sudo* to manually change the controlling 
files in `/sys/class/backlight/`. But, if we do use **Polkit** (8), we can change the brightness using the DBus interface listed below:

```
interface org.freedesktop.login1.Session

[...]

SetBrightness(in s subsystem, in s name, in u brightness);
```

## So apparently Polkit really is awesome
Polkit provides a way for **unpriviliged** users to ask an *Authority*, if they can perform a **privileged** action through a system process.
Polkit does that by checking a set of rules. Allowed, or not allowed actions that can be performed by the user are to be found in 
`/usr/share/polkit-1/actions`.
