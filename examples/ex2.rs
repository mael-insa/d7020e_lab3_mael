//! cargo symex --elf --example ex2 --function timed_loop [--release]

#![no_std]
#![no_main]

use cortex_m::asm;
use panic_halt as _;
use rp2040_hal::entry;

#[no_mangle]
#[inline(never)]
pub fn timed_loop() {
    for _ in 0..10000 {
        asm::nop();
    }
}

#[entry]
fn main() -> ! {
    timed_loop();
    loop {}
}
