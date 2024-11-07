//! cargo symex --elf --example ex2 --function timed_loop --release

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal as _;
use panic_halt as _;

#[no_mangle]
#[inline(never)]
pub fn timed_loop() {
    for _ in 0..10000 {
        cortex_m::asm::nop();
    }
}

#[entry]

fn main() -> ! {
    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        core::ptr::read_volatile(&timed_loop());
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
