//! Example 5 runner.
//!
//! This app runs a measurement on the actual system and runs the same measurement in symex.
//! The goal for this application is show that, although safe, the estimations provided by symex
//! for the `m7` architechture is quite inprecise.
//!
//! ## Binary assumptions
//!
//! This assumes that there exists a function *meaure* in the binary and that this function is, at
//! some point, called by the main function.
//!
//! ## For the interested student:
//!
//! 1. Why do you think that the cycle estimates are this inaccurate?
//! 2. Try compiling for a cortexm0 processor (ARMv6-M) and run the same test.
//!     2.1. Were the results similar?
//!     2.2  Why do you think that is?
use std::path::Path;
use std::{fs, time::Duration};

use probe_rs::{flashing, MemoryInterface, Permissions, Session};
use symex::run_elf;

use std::fs::File;
use std::io::prelude::*;

/// Runs all of the binaries in the `test_binaries` directory through both symex and the real
/// hardware.
fn main() {
    println!("Utility to measure HW cycles for the nRF52840_xxAA");

    // 1. Connect to the devkit.
    //let session = Session::auto_attach("nRF52840_xxAA", Permissions::default());
    //let mut session = match session {
    //    Ok(session) => session,
    //    Err(_e) => {
    //        eprintln!("Could not attach to nrf52840, please make sure that it is connected");
    //        return;
    //    }
    //};
    //println!("attached to nRF52840_xxAA {:?}", session.architecture());

    // 2. Collect binaires to test.
    // Get the binaries
    let list_of_files = fs::read_dir("test_binaries");
    if list_of_files.is_err() {
        eprintln!("Could not open directory test_binaries, please make sure that it exists.");
        return;
    }
    // Not actually unsafe since we just checked it.
    let mut list_of_files = unsafe { list_of_files.unwrap_unchecked() }.peekable();

    if list_of_files.peek().is_none() {
        eprintln!("test_binaries is empty");
        return;
    }

    // 3. Open log file.
    let mut f = File::options()
        .create(true)
        .append(true)
        .open("exec.log")
        .unwrap();

    // 4. Measure the cycle counts in hardware and in symex.
    for to_test in list_of_files {
        // Ignore any files that do not exist.
        let path = match to_test {
            Ok(val) => val,
            Err(_) => continue,
        }
        .path();

        let name = match path.file_stem().map(|name| name.to_str()) {
            Some(Some(name)) => name,
            _ => continue,
        };

        // 4.1 Run the actual tests.

        println!("Measuring : {name}");
        //let hw_measurement = measure_hw(&path, &mut session);
        //println!("HW : {hw_measurement}");

        let symex_measurement = measure_symex(&path);

        let _ = write!(
            &mut f,
            "Name : {name} \n\thw \t:  \n\tsymex \t: {symex_measurement}\n"
        );
        println!("{name}:\n\thw \t:  \n\tsymex \t: {symex_measurement}\n")
    }
}

/// Runs symex on the binary.
fn measure_symex<P: AsRef<Path>>(path: &P) -> u64 {
    // Run the file through symex, and test the function measure.
    let results = run_elf::run_elf(path, "measure", false).unwrap();
    // Return path that took the longest.
    results.iter().map(|r| r.max_cycles).max().unwrap() as u64
}

/// Measures the amount of clock cycles that the hardware took to run the same path.
fn measure_hw<P: AsRef<Path>>(path: &P, session: &mut Session) -> u64 {
    // 1. Load the binary on to the target and get the first, and only, core.
    flashing::download_file(session, path, flashing::Format::Elf).unwrap();
    let mut core = session.core(0).unwrap();

    // 2. Reset the device and wait for system halt on breakpoint.

    // Start program
    core.reset().unwrap();
    // Wait until first measuring point
    core.wait_for_core_halted(Duration::from_millis(5000))
        .unwrap();

    // 3. Read the number of cycles executed at this point.

    // Read the first cycle counting register.
    //
    // See C1.8.6 in the armv7em manual DWT_CYCCNT.
    let start = core.read_word_32(0xe000e018).unwrap() & 0x00FFFFFF;

    // 4. Run the program until the next breakpoint.
    // run until next measuring point
    core.run().unwrap();
    core.wait_for_core_halted(Duration::from_millis(500))
        .unwrap();

    // 5. Read the number of cycles executed at the end of the pogram and compute difference.
    let end = core.read_word_32(0xe000e018).unwrap() & 0x00FFFFFF;

    // calculate a measured time
    // compensate for bkpt discrepancy by subtracting 5 (determined by experimentation)
    let diff = start - end - 5;
    diff as u64
}
