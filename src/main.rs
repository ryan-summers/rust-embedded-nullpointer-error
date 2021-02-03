#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

/// A zero-sized test structure.
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
    /// Test new function
    ///
    /// # Args
    /// * `id` - The ID for uniquely identifying the device.
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
