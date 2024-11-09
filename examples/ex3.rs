//! cargo symex --example ex3 --function device_test --release
//! cargo symex --example ex3 --function device_test_sum --release

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal as _;
use panic_halt as _;

#[allow(unused_imports)]
use symex_lib::{assume, Any};

// Model of the Device state
struct Device {
    buffer: [u8; 8],
    read_pos: u8,
}

impl Device {
    fn reset() -> Self {
        Device {
            buffer: [0; 8],
            read_pos: 0,
        }
    }
}

// Model of the Device functionality
impl Device {
    fn received(&mut self) -> u8 {
        self.read_pos = 0; // we reset the read_pos

        // we return an unknown number of bytes received
        #[allow(clippy::let_and_return)]
        let n = u8::any();
        //assume(n <= 8);
        //for v in self.buffer[0..n as usize].iter_mut() {
        //    *v = u8::any()
        //}
        n
    }

    #[inline(never)]
    fn data(&mut self) -> u8 {
        let r = self.buffer[self.read_pos as usize];
        self.read_pos += 1;
        r
    }
}

#[no_mangle]
#[inline(never)]
pub fn device_test() {
    let mut device = Device::reset();

    let n = device.received();
    for _ in 0..n {
        let _data = device.data();
    }
}

#[no_mangle]
#[inline(never)]
pub fn device_test_sum() -> u8 {
    let mut device = Device::reset();

    let n = device.received();

    let mut sum: u8 = 0;
    for _ in 0..n {
        sum += device.data() as u8;
    }
    sum
}

#[entry]

fn main() -> ! {
    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        core::ptr::read_volatile(&device_test());
        core::ptr::read_volatile(&device_test_sum());
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
