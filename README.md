# Symbolic Execution

In this lab you will work with symbolic execution using the [symex](https://github.com/norlen/symex) crate. 

---

## Install

### SymEx
`symex` is in early development and has not yet been released through `crates.io`. Instead you can install the tool directly from the git repo.

```shell
cargo install --git https://github.com/norlen/symex.git
```

Alternatively, if you are interested in looking into the source code/and or play around with the examples in the repository, you may clone the repo and install the tool manually as follows:

- `cd` to some folder outside this repo. 
  
- `git clone https://github.com/norlen/symex) repository`, and 

- `cd symex`. Now you can install the `cargo-symex` cargo sub command. 

- `cargo install --path cargo-symex` 

### LLVM

The current `symex` implementation uses LLVM-IR under the hood. For this to work, you need to install `llvm` using your package manager. The latest packed version in arch linux is LLVM-14 (221125).

```shell
pacman -S llvm
```

### Rust version

The Rust toolchain comes with a pre-packed LLVM backend which needs to match the system LLVM (14 in our case) for the `symex` tool to work. The latest Rust compiler is packing LLVM-15 which causes a mismatch, so for running the labs you need to use a slightly older toolchain version (1.64). Rust allows you to set the toolchain per folder. `cd` to the folder where you want to run `cargo symex` from and override the Rust toolchain version to be used as below:

```shell
cd <folder>
rustup override set 1.64
```
---

## Exercises

### Ex1

In this exercise you will get familiar with the `symex` tool. It is still fairly primitive but shows the principles of symbolic execution, which is the point here.

Run the tool:
```shell
cargo symex --example ex1 --function get_sign_test
```

If you get an error at this point check the install instructions. If still does not work let me know on the discord and we'll figure out a solution.

- Ex1 A1)

[Paste your output here]

If everything worked out correctly `symex` will report three paths (PATH 1, PATH 2 and PATH 3).

For each path it should report `Success`, i.e., that the path was executed till end without any error (no assertion violations encountered).

Each path returns with a value (as the function `get_sign_test` is returning an `i32` typed value).

The value is reported simply as a bit vector, so have a suitable converter at hand, e.g., [RapidTables](https://www.rapidtables.com/convert/number/binary-to-decimal.html).

For PATH 1, the output should be trivial (0...01).
Look at the source code to figure out what the path condition is for `get_sign` to return 1.

- Ex1 A2) 

[Your answer here]

Under PATH 1 you also will find:

```shell
Symbolic:
    a-2155673927: 0x40000000 (32-bits)
```
This amounts to a concrete assignment of `v` triggering PATH 1. Now translate this value to an unsigned integer (the type of `v` is `i32`).

- Ex1 A3)

[Your answer here]

Does this value meet the condition Ex1 A2)?

- Ex1 A4)
[Yor answer here]

