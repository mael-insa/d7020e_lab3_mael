#![no_std]
#![no_main]
//! cargo symex --elf --example ex4 --function <function to test>
//!
//! functional equivalence

use core::arch::asm;

use cortex_m_rt::entry;
use nrf52840_hal::pac;
use panic_halt as _;

#[allow(dead_code)]
use symex_lib::assume;
use symex_lib::{end_cyclecount, start_cyclecount};

// recursive
fn sum_recursive(n: u8) -> u32 {
    match n {
        0 => 0,
        _ => n as u32 + sum_recursive(n - 1),
    }
}

// iterative
fn sum_iterative(n: u8) -> u32 {
    let mut sum = 0;
    for v in 0..n {
        sum += v as u32
    }
    sum
}

#[no_mangle]
#[inline(never)]
// test sum_iterative == sum_recursive
pub fn equal_iter_rec(n: u8) {
    assume(n < 10);
    assert!(sum_iterative(n) == sum_recursive(n));
}

// mathematical formula
fn sum_formula(n: u8) -> u32 {
    let n: u32 = n as u32;
    n * (n + 1) / 2
}

#[no_mangle]
#[inline(never)]
// test equal rec_formula
pub fn equal_rec_formula(n: u8) {
    assume(n < 10);
    assert!(sum_recursive(n) == sum_formula(n));
}

// test complexity sum_recursive
#[no_mangle]
#[inline(never)]
pub fn complexity_sum_recursive(n: u8) {
    assume(n < 10);
    let _ = sum_recursive(n);
}

// test complexity sum_iterative
#[no_mangle]
#[inline(never)]
pub fn complexity_sum_iterative(n: u8) {
    assume(n < 10);
    let _ = sum_iterative(n);
}

// test complexity sum_formula
#[no_mangle]
#[inline(never)]
pub fn complexity_sum_formula(n: u8) {
    assume(n < 10);
    let _ = sum_formula(n);
}

#[inline(never)]
#[no_mangle]
/// Ex5 measure the time complexity of the sum functions.
fn measure(n: u8) -> u32 {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    assume(n < 10);
    let r = sum_formula(n);
    // Try to change the function call to either of these.
    //
    // Is there a significant difference in the accuracy of the symex estimates?
    //let r = sum_iterative(n);
    //let r = sum_recursive(n);
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
    let n = measure(9);
    equal_iter_rec(1);
    equal_rec_formula(1);
    complexity_sum_recursive(1);
    let si = complexity_sum_iterative(1);
    let sf = complexity_sum_formula(1);
    unsafe {
        let _ = core::ptr::read_volatile(&si);
        let _ = core::ptr::read_volatile(&sf);
        let _ = core::ptr::read_volatile(&n);
    }
    loop {}
}
