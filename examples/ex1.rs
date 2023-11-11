//! cargo symex --elf --example ex1 --function get_sign_test [--release]
//! cargo symex --elf --example ex1 --function addu [--release]
//! cargo symex --elf --example ex1 --function addi [--release]
//!
//! functional verification

#![no_std]
#![no_main]

use panic_halt as _;
use rp2040_hal::entry;

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

// this is just here to make Rust happy :)
#[entry]
fn main() -> ! {
    let l = get_sign(3);
    // let (v0, v1, v2, v3) = add(1, 2, 3, 4);
    let (v0, v1) = addu(1);
    let (v2, v3) = addi(2);

    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        let _ = core::ptr::read_volatile(&l);
        let _ = core::ptr::read_volatile(&v0);
        let _ = core::ptr::read_volatile(&v1);
        let _ = core::ptr::read_volatile(&v2);
        let _ = core::ptr::read_volatile(&v3);
    }
    loop {}
}
