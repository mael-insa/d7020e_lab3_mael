# Symbolic Execution

In this lab you will work with symbolic execution using the [symex](https://github.com/s7rul/symex/tree/armv6-m) crate.

The repository is a fork by LTU student Erik Serrander of the original work [symex](https://github.com/norlen/symex) by LTU student Joacim Norlen.

## Learning objectives

- General understanding of static symbolic execution:

  `Symex` is an example of a static symbolic execution framework. Static here implies that symex either covers all feasible paths or fails with an error (no approximation is done by the tool).

- General characteristics of low-level symbolic execution:

  In the `armv6` branch, Erik Serrander has extended `symex` with an execution engine for the `armv6m` instruction set (ISA). This allows for detailed/low-level analysis of binary code (extracted from `elf` files).

  Benefits of binary level analysis include:

  - Detailed analysis including generated code for function calls and stack handling.

  - Possibility to analysis of code including inline assembly and/or external legacy C/C++ code.

  - No need to blindly trust compiler backend (LLVM + linker) to be correct, as the analysis is done on the generated binary.

  Drawbacks of binary level analysis include:

  - Performance (analysis at binary level is more detailed at the cost of (typically) increased number of instructions to analyze)

  - An custom execution engine for each target ISA architecture is required (in this case the `armv6m`), whereas target agnostic analysis at LLVM-IR level covers all backends supported by LLVM.

- General understanding of execution time estimation by path analysis:

  Based on the vendors specification of the `armv6m` ISA the `symex` tool estimates the CPU time fore each path found (and thus the worst case execution time can be estimated).

  Overhead due to RAM/FLASH wait states and bus arbitration is currently not modelled in `symex`, however such extension can be developed.

Write a short reflection in your own words below, showing how you gained understanding towards the learning objectives/goals.

[Your reflection here]

---

## Install

`symex` is in early development and has not yet been released through `crates.io`. Instead you can install the tool directly from the git repo.

```shell
cargo install --git https://github.com/s7rul/symex --branch armv6-m cargo-symex
```

Alternatively, if you are interested in looking into the source code/and or play around with the examples in the repository, you may clone the repo and install the tool manually as follows:

- `cd` to some folder outside this repo.
- `git https://github.com/s7rul/symex/tree/armv6-m`, and

- `cd symex`. Now you can install the `cargo-symex` cargo sub command.

- `cargo install --path cargo-symex`

If you already have a `cargo-symex` installed you can update an install by adding the `--force` flag to the install command.

### Rust version

Make sure you are on the latest Rust toolchain.

Add the `thumbv6m-none-eabi` compilation target.

```shell
rustup update
rustup target add thumbv6m-none-eabi
```

---

## Exercises

- `ex1`

  In this exercise you will get familiar with the `symex` tool. It is still fairly primitive but shows the principles of symbolic execution, which is the point here.

- `ex2`

  In this exercise we will look at a release build of the `ex1` program.

- `ex3`

  In this example we will see how we can model hardware, and verify that the user program accesses the hardware correctly.

- `ex4`

  Here you will prove functional equality by means of symbolic execution. You will also investigate the correlation between instructions executed by `symex` and instructions executed on bare metal hardware.
