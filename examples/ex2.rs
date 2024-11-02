//! cargo symex --elf --example ex2 --function timed_loop --release

#![no_std]
#![no_main]

use core::arch::asm;
use cortex_m_rt::entry;
use nrf52840_hal::pac;
use panic_halt as _;
use symex_lib::{end_cyclecount, start_cyclecount};

#[no_mangle]
#[inline(never)]
pub fn timed_loop() {
    for _ in 0..10000 {
        cortex_m::asm::nop();
    }
}

#[inline(never)]
#[no_mangle]
/// Ex5 measure the time complexity of timed_loop.
fn measure() {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = timed_loop();
    unsafe {
        asm!("bkpt 2");
    }
    end_cyclecount();
    r
}

#[entry]
/// This is left for ex5.
fn main() -> ! {
    // Start systic timer and enable the cycle counter.
    //
    // This is mandatory for us to be able to measure the cycle counts with probe-rs.
    let pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let _clocks = nrf52840_hal::clocks::Clocks::new(pac.CLOCK).enable_ext_hfosc();
    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();

    let measure_result = measure();

    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        let _ = core::ptr::read_volatile(&measure_result);
    }
    loop {}
}
