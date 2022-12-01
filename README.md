# Symbolic Execution

In this lab you will work with symbolic execution using the [symex](https://github.com/norlen/symex) crate. 

---

## Install

### LLVM

The current `symex` implementation uses LLVM-IR under the hood. For the tool to work, you need to install `llvm` using your package manager.

- arch: The latest packed version in arch linux is LLVM-14 (221125).
  ```shell
  pacman -S llvm
  ```

- ubuntu like: The latest packed version in ubuntu is LLVM-14 (221126)
  ```shell
  sudo apt install clang lldb lld
  ```

  You can verify the `clang` installation by:

  ```shell
  clang --version
  clang++ --version
  ```

### ZLIB

If you run into a compilation error when installing `symex` you may need to install `zlib`.

- ubuntu like: 

  ```shell
  apt install zlib1g-dev
  ```

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

Optional exercises

- `ex4`

  Here you will prove functional equality by means of symbolic execution. You will also investigate the correlation between instructions executed by `symex` and instructions executed on bare metal hardware.