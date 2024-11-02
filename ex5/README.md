# EX5: Comparing symex estimates to real measurements.

In this exercise you will get to see the inaccuracies in the symex tool.
To show this, we will be using the previous exercises `measure` function.

This function exists in all previous examples in some form or another
and looks something like:

```rust
#[inline(never)]
#[no_mangle]
fn measure(...) -> ... {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    // Do the thing you want to measure.
    unsafe {
        asm!("bkpt 2");
    }
    end_cyclecount();
}
```

This crate uses this signature to measure cycles, both using `symex` and `probe-rs`. For `symex` we simply evaluate the
`measure` function, but for `probe-rs` we have to be a bit more clever and use the hardware.
For this we can use the `SYSTIC` peripheral to register the cycle counts at specific points.
If we look at the `main` functions in the previous examples you can see that we start the cycle counter 

```rust
#[entry]
fn main() -> ! {
    // Start systic timer and enable the cycle counter.
    //
    // This is mandatory for us to be able to measure the cycle counts with probe-rs.
    let pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let _clocks = nrf52840_hal::clocks::Clocks::new(pac.CLOCK).enable_ext_hfosc();
    let systic_reload_time: u32 = 0x00ffffff;
    // The important bits are here.
    let mut systic = core.SYST;
    systic.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();
    ...
    ...
```

And in the [`main`](./src/main.rs) file in this crate we simply wait for the break point, and then read the count from `SYSTIC` as such

```rust
    core.wait_for_core_halted(Duration::from_millis(500))
        .unwrap();
    // See SYST_CVR in armv7em spec.
    let end = core.read_word_32(0xe000e018).unwrap() & 0x00FFFFFF;
```

This is more or less all we need to do. The final step is to compile the binaries and point the tool at them.
This is where [`run_task`](./run_task) comes in. It compiles all of the previous examples and runs the test on them
and reports the results as `exec.log`, you run it as such:

```bash
# in ex5 folder.

# Argument order <architecture number>, <debug or release>
./run_task v7em release
```
It is worth noting that <v7em> is inserted in to `thumb<inserted here>-none-eabi` to provide the target architecture for the build.
So if v7em is provided you build for thumbv7em-none-eabi. Similarly, if v6m is provided you build for thumbv6m-none-eabi.


## A)

Run the tool and report the estimated cycle counts and the measured cycle-counts here.

  [Paste your output here].

## B)

Which 








