use dbus::blocking::Connection;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a blocking connection on the DBus system bus
    let conn = Connection::new_system()?;
    /* Remember that the question mark operator is a way in Rust to implicitly 
       return the error type variant of the function it is called onto, if the value 
       returned is not an Ok variant. This of course means that the Error variant must
       be compatible by the Result error type of the main() function, but this is
       verified since we are looking for Errors which implement the Error trait. 
    */

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("org.freedesktop.login1", 
                                                        "/org/freedesktop/login1/session/self", 
                                                        Duration::from_millis(5000));
    
    // Method call
    let _:() = proxy.method_call("org.freedesktop.login1.Session", "SetBrightness", 
                                ("backlight", "intel_backlight", 800u32,))?;

    Ok(())
}
