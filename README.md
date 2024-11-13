# Symbolic Execution

In this lab you will work with symbolic execution using the [symex](https://github.com/ivajon/symex/tree/trivial_stack_size) crate.

The repository is a fork by LTU student Ivar Jönsson of the original work [symex](https://github.com/norlen/symex) by LTU students Joacim Norlen and Erik Serrander.

## Learning objectives

- General understanding of static symbolic execution:

  `Symex` is an example of a static symbolic execution framework. Static here implies that symex either covers all feasible paths or fails with an error (no approximation is done by the tool).

- General characteristics of low-level symbolic execution:

  In the `trivial_stack_size` branch, Ivar has extended `symex` with an execution engine for the `armv7em` instruction set (ISA). This allows for detailed/low-level analysis of binary code (extracted from `elf` files).

  Benefits of binary level analysis include:

  - Detailed analysis including generated code for function calls and stack handling.

  - Possibility to analysis of code including inline assembly and/or external legacy C/C++ code.

  - No need to blindly trust compiler backend (LLVM + linker) to be correct, as the analysis is done on the generated binary.

  Drawbacks of binary level analysis include:

  - Performance (analysis at binary level is more detailed at the cost of (typically) increased number of instructions to analyze)

  - An custom execution engine for each target ISA architecture is required (in this case the `armv7em`), whereas target agnostic analysis at LLVM-IR level covers all backends supported by LLVM.

- General understanding of execution time estimation by path analysis:

  Based on the vendors specification of the `armv7em` ISA the `symex` tool estimates the CPU time fore each path found (and thus the worst case execution time can be estimated).

  Overhead due to RAM/FLASH wait states and bus arbitration is currently not modelled in `symex`, however such extension can be developed.

Write a short reflection in your own words below, showing how you gained understanding towards the learning objectives/goals.

At the end of this lab, I acquired knowledge about symbolic execution. It allows us to see the different paths taken by the code, execution times, stack usage, the number of iterations or clock times required to execute a function. If a path leads to an error: panic, we can now put safeguards in place in the code to avoid them, for example in the event of an oveerflow on a variable. Finally, we can use symbolic execution to compare the performance of two functions that give us the same results in order to determine which is more efficient, which is important for embedded use. We can also analyse disassembled code to understand in detail what the coded functions produce. 

---

## Install

For Windows WSL please confer to [WSL.md](WSL.md).

`symex` is in early development and has not yet been released through `crates.io`. Instead you can install the tool directly from the git repo.

```shell
cargo install --git https://github.com/ivajon/symex --branch trivial_stack_size cargo-symex
```

Alternatively, if you are interested in looking into the source code/and or play around with the examples in the repository, you may clone the repo and install the tool manually as follows:

- `cd` to some folder outside this repo.
- `git clone https://github.com/ivajon/symex/tree/trivial_stack_size`, and

- `cd symex`. Now you can install the `cargo-symex` cargo sub command.

- `cargo install --path cargo-symex`

If you already have a `cargo-symex` installed you can update an install by adding the `--force` flag to the install command.

### Rust version

Make sure you are on the latest Rust toolchain.

Add the `thumbv7em-none-eabi` compilation target.

```shell
rustup update
rustup target add thumbv7em-none-eabi
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

  Here you will also estimate worst case execution time (measured by clock cycles) and worst case stack usage.
