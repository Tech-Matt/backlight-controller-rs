# backlight-controller-rs
A simple backlight controller for Intel backlight in Rust

## (Un)Technical Overview
> I'll be extremely un-technical here, cause my knowledge on the topic is extremely poor

Apparently [DBus](https://freedesktop.org/wiki/Software/dbus/), a linux cool communication bus, both used by system processes and application processes as an IPC method, provides some cool *interfaces*, to which a user may interact with. This comes in handy, cause
changing the backlight in Linux is a **privileged** action and thus you need to be root or use *sudo* to manually change the controlling 
files in `/sys/class/backlight/`. But, if we do use **Polkit** (8), we can change the brightness using the DBus interface listed in ```man org.freedesktop.login1```.

```
interface org.freedesktop.login1.Session

[...]

SetBrightness(in s subsystem, in s name, in u brightness);
```

## So apparently Polkit really is awesome
Polkit provides a way for **unpriviliged** users to ask an *Authority*, if they can perform a **privileged** action through a system process.
Polkit does that by checking a set of rules. Allowed, or not allowed actions that can be performed by the user are to be found in 
`/usr/share/polkit-1/actions`.

> Apparently there is a ```/usr/share/polkit-1/actions/org.freedesktop.login1.policy``` but I have
> not found anything about brightness in there. Does it mean it is an **unpriviliged** action?


## Backlights Settings
In my case these are the properties of my backlight (found in `/sys/class/backlight/`)

| Property | Value |
| -------- | ----- |
| Vendor | intel |
| Max Brightness | 1500 |

## Inspecting the DBus

A very useful cli app called `busctl(1)` can be used to inspect the DBus. Some cool commands are:

- ```busctl list```: this list all the peers on the bus
- ```busctl monitor``` or ```busctl capture```: with these ones it is possible to see the entire traffic on the bus, and messages sent, pretty cool
- ```busctl introspect```: used to see interfaces and objects methods of specific services
- ```busctl tree```: Show a tree of all peers on the bus
- ```busctl call```: invoke a method and see the response
- and many more

In particular the `introspect SERVICE OBJECT [INTERFACE]` command is quite useful. We can use it to see the possible backlight controls.
The commands requires then the service and object fields. We can retrieve the service name from:
`busctl list`.
This gives:
```
org.freedesktop.login1
```
And then with `busctl tree`:
```
Service org.freedesktop.login1:
└─ /org
  └─ /org/freedesktop
    ├─ /org/freedesktop/LogControl1
    └─ /org/freedesktop/login1
      ├─ /org/freedesktop/login1/seat
      │ ├─ /org/freedesktop/login1/seat/auto
      │ ├─ /org/freedesktop/login1/seat/seat0
      │ └─ /org/freedesktop/login1/seat/self
      ├─ /org/freedesktop/login1/session
      │ ├─ /org/freedesktop/login1/session/_31
      │ ├─ /org/freedesktop/login1/session/_32
      │ ├─ /org/freedesktop/login1/session/auto
      │ └─ /org/freedesktop/login1/session/self
      └─ /org/freedesktop/login1/user
        ├─ /org/freedesktop/login1/user/_1000
        └─ /org/freedesktop/login1/user/self
```

From here we can see that the object name is `/org/freedesktop/login1`.
We can then try to issue `busctl introspect org.freedesktop.login1 /org/freedesktop/login1` and see what happens:

I can see that there is an interface called ```org.freedesktop.login1.Manager``` which I can try to
introspect.

```
busctl introspect org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager | grep SetBrightness
```

> Unfortunately this gave no match. The fact that there is also no polkit rule for setting brightness
> makes me think there is no method whatsoever implemented, but this is kinda strange since it was
> mentioned in the man page

AH AH! Found. Watching again in `man org.freedesktop.login1` I could see that the
`SetBrightness` method was actually defined in the object `/org/freedesktop/login1/session/1`
which indeed list the method `SetBrightness` if `busctl introspect` is run against it.

Cool! that means if I can use DBus from Rust and call that method, I can change the
brightness.

## Next Steps

- Test `busctl call` to call the `SetBrightness` method.
- Test also wrong inputs to see what happens
- Go watch the Rust Dbus library and see how to implement it
- Check some resources on how to properly handle and write a console app in Rust
