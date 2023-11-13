#![no_std]
#![no_main]
//! cargo symex --example ex4 --function equal
//!
//! functional equivalence

use panic_halt as _;
use rp2040_hal::entry;

#[allow(dead_code)]
use symex_lib::assume;

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
    for v in 0..=n {
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

// main here to prevent LLVM to optimize out our code
#[entry]
fn main() -> ! {
    equal_iter_rec(1);
    equal_rec_formula(1);
    complexity_sum_recursive(1);
    let si = complexity_sum_iterative(1);
    let sf = complexity_sum_formula(1);
    unsafe {
        let _ = core::ptr::read_volatile(&si);
        let _ = core::ptr::read_volatile(&sf);
    }
    loop {}
}
