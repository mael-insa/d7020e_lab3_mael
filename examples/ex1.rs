//! cargo symex --elf --example ex1 --function get_sign [--release]
//! cargo symex --elf --example ex1 --function addu [--release]
//! cargo symex --elf --example ex1 --function addi [--release]
//!
//! functional verification

#![no_std]
#![no_main]
#![deny(dead_code)]
use cortex_m_rt::entry;
use nrf52840_hal as _;
use panic_halt as _;

#[no_mangle]
#[inline(never)]
fn get_sign(v: i32) -> i32 {
    #[allow(clippy::comparison_chain)]
    if v > 0 {
        1
    } else if v == 0 {
        0
    } else {
        -1
    }
}

#[no_mangle]
#[inline(never)]
fn addu(v0: u32) -> (u32, u32) {
    if v0 > u32::MAX / 2 {
        (v0, u32::MAX)
    } else {
        (v0, v0 + v0) 
    }
    
}

#[no_mangle]
#[inline(never)]
fn addi(v0: i32) -> (i32, i32) {
    (v0, v0 + v0)
}

#[entry]
fn main() -> ! {
    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        let _ = core::ptr::read_volatile(&get_sign(0));
        let _ = core::ptr::read_volatile(&addu(0));
        let _ = core::ptr::read_volatile(&addi(0));
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
