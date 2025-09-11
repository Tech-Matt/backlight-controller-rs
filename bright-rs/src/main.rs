use dbus::blocking::Connection;
use std::time::Duration;
use std::env;

// Implement new type alias for missing console argument

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a blocking connection on the DBus system bus
    let conn = Connection::new_system()?;
 
    // Get CLI args
    let args: Vec<String> = env::args().collect();

    /* Expected Cli Args are:
     * script_name (@index 0) [DEFAULT IN LINUX]
     * brightness: u32 (@index 1)
     */
    let brightness_str = match args.get(1) {
        Some(brightness) => brightness,
        None => todo!(),
    };

    let brightness: u32 = brightness_str.parse().expect("Parsing Failed.");

    // Create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("org.freedesktop.login1", 
                                                        "/org/freedesktop/login1/session/self", 
                                                        Duration::from_millis(5000));
    
    // Method call
    let _:() = proxy.method_call("org.freedesktop.login1.Session", "SetBrightness", 
                                ("backlight", "intel_backlight", brightness,))?; //! This also needs proper error handling

    Ok(())
}
