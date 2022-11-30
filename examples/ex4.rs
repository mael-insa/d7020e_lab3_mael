//! cargo symex --example ex4 --function equal
#[allow(dead_code)]
use symex_lib::{assume, Any};

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

// test equal
pub fn equal_iter_rec() {
    let n = u8::any();
    // assume(n < 10);
    assert!(sum_iterative(n) == sum_recursive(n));
}

// mathematical formula
fn sum_formula(n: u8) -> u32 {
    let n: u32 = n as u32;
    n * (n + 1) / 2
}

// test equal rec_formula
pub fn equal_rec_formula() {
    let n = u8::any();
    assume(n < 10);
    assert!(sum_recursive(n) == sum_formula(n));
}

// test complexity sum_recursive
pub fn complexity_sum_recursive() {
    let n = u8::any();
    assume(n < 10);
    let _ = sum_recursive(n);
}

// test complexity sum_iterative
pub fn complexity_sum_iterative() {
    let n = u8::any();
    assume(n < 10);
    let _ = sum_iterative(n);
}

// test complexity sum_formula
pub fn complexity_sum_formula() {
    let n = u8::any();
    assume(n < 10);
    let _ = sum_formula(n);
}






// some function
fn f(a: i32, b: i32, c: i32) -> i32 {
    2 * a - 5 / b + c
}

// test solve for result == 42
fn test_f() {
    let a = i32::any();
    let b = i32::any();
    let c = i32::any();

    // assume(a > 10);
    // assume(b < 2);

    assert!(f(a, b, c) == 42);
}

// this is just here to make Rust happy :)
fn main() {}
