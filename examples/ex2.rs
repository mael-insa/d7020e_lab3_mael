//! cargo symex --elf --example ex2 --function timed_loop --release

#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use nrf52840_hal::pac::Peripherals;
use panic_halt as _;

#[no_mangle]
#[inline(never)]
pub fn timed_loop() {
    for _ in 0..10000 {
        asm::nop();
    }
}

#[entry]
fn main() -> ! {
    // Just here to ensure that we get a vector table.
    let _ = unsafe { Peripherals::steal() };
    timed_loop();
    loop {}
}
