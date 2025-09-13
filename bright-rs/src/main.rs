use dbus::blocking::Connection;
use std::time::Duration;
use std::env;

mod errors;
use crate::errors::Error;


fn main() -> Result<(), Error> {
    // Open a blocking connection on the DBus system bus
    let conn = Connection::new_system()?;
 
    /* Expected Cli Args are:
     * script_name (@index 0) [DEFAULT IN LINUX]
     * brightness: u32 (@index 1)
     */
    let brightness_str = env::args().nth(1).ok_or(Error::MissingArg)?;
    let brightness: u32 = brightness_str.parse()?;

    // Create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("org.freedesktop.login1", 
                                "/org/freedesktop/login1/session/self", 
                                Duration::from_millis(5000));
    
    // Method call
    let _:() = proxy.method_call(
        "org.freedesktop.login1.Session", 
        "SetBrightness", 
        ("backlight", "intel_backlight", brightness,)
    )?; 

    Ok(())
}
