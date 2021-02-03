#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

/// An interface for managing MQTT settings.
pub struct Test { }

fn take_objects(id: &'static str) -> Result<(), ()> {
    // Do something conditional with the ID to prevent optimization.
    for c in id.chars() {
        if c != 'a' {
            return Err(());
        }
    }

    Ok(())
}

impl Test {
    /// Construct a new settings interface using the network stack.
    ///
    /// # Args
    /// * `stack` - The TCP network stack to use for communication.
    /// * `id` - The ID for uniquely identifying the device.
    /// * `broker` - The IpAddress of the MQTT broker.
    /// * `settings` - The initial settings of the interface.
    ///
    /// # Returns
    /// A new `MqttInterface` object that can be used for settings configuration and telemtry.
    pub fn new(id: &'static str) -> Result<(), ()> {
        take_objects(id)?;
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let _ = Test::new("a").unwrap();
    loop {
        asm::nop();
    }
}
