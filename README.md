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

- `ex1` 

  In this exercise you will get familiar with the `symex` tool. It is still fairly primitive but shows the principles of symbolic execution, which is the point here.

- `ex2`

  In this exercise we will look at a release build of the `ex1` program. 

- `ex3`
  
  In this example we will see how we can model hardware, and verify that the user program accesses the hardware correctly.