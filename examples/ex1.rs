//!
//! ```shell
//! cargo symex --example ex1 --function get_sign_test
//! ```
use symex_lib::Any;

fn get_sign(v: i32) -> i32 {
    if v > 0 {
        return 1;
    } else if v == 0 {
        return 0;
    } else {
        return -1;
    }
}

// this is a test for the get_sign
pub fn get_sign_test() -> i32 {
    let v = i32::any();
    get_sign(v)
}

// this is just here to make Rust happy :)
fn main() {}
