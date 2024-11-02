#![no_std]
#![no_main]
//! cargo symex --example ex3 --function device_test
//! cargo symex --example ex3 --function device_test_sum --release

use core::arch::asm;

use cortex_m_rt::entry;
use nrf52840_hal::pac;
use panic_halt as _;

use symex_lib::{end_cyclecount, start_cyclecount, Any};

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
        let n = u8::any();
        // assume(n <= 8);
        // for v in self.buffer[0..n as usize].iter_mut() {
        //     *v = u8::any()
        // }
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

    let mut sum = 0;
    for _ in 0..n {
        sum += device.data();
    }
    sum
}

#[inline(never)]
#[no_mangle]
/// Ex5 measure the time complexity of device_test.
fn measure() {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = device_test();
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
    let sum = device_test_sum();

    // force the result to be read, thus prevent LLVM to optimize out the `get_sign` function.
    unsafe {
        let _ = core::ptr::read_volatile(&measure_result);
        let _ = core::ptr::read_volatile(&sum);
    }
    loop {}
}
