//! cargo symex --elf --example ex1 --function get_sign [--release]
//! cargo symex --elf --example ex1 --function addu [--release]
//! cargo symex --elf --example ex1 --function addi [--release]
//!
//! functional verification

#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m_rt::entry;
use nrf52840_hal::pac;
use panic_halt as _;
use symex_lib::{end_cyclecount, start_cyclecount};

#[no_mangle]
#[inline(never)]
pub fn get_sign(v: i32) -> i32 {
    if v > 0 {
        return 1;
    } else if v == 0 {
        return 0;
    } else {
        return -1;
    }
}

#[no_mangle]
#[inline(never)]
pub fn addu(v0: u32) -> (u32, u32) {
    (v0, v0 + v0)
}

#[no_mangle]
#[inline(never)]
pub fn addi(v0: i32) -> (i32, i32) {
    (v0, v0 + v0)
}

#[inline(never)]
#[no_mangle]
/// Ex5 measure the time complexity of get_sign.
fn measure(v: i32) -> i32 {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = get_sign(v);
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

    let l = get_sign(3);
    // let (v0, v1, v2, v3) = add(1, 2, 3, 4);
    let (v0, v1) = addu(1);
    let (v2, v3) = addi(2);
    let measure_result = measure(1);

    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        let _ = core::ptr::read_volatile(&l);
        let _ = core::ptr::read_volatile(&v0);
        let _ = core::ptr::read_volatile(&v1);
        let _ = core::ptr::read_volatile(&v2);
        let _ = core::ptr::read_volatile(&v3);
        let _ = core::ptr::read_volatile(&measure_result);
    }
    loop {}
}
