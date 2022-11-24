# Symbolic Execution

In this lab you will work with symbolic execution using the [symex](https://github.com/norlen/symex) crate. 

## Install

`symex` is in early development and has not yet been released through `crates.io`. Instead you can install the tool directly from the git repo.

```shell
cargo install --git https://github.com/norlen/symex.git
```

Alternatively, if you are interested in looking into the source code/and or play around with the examples in the repository, you may clone the repo and install the tool manually as follows:

- `cd` to some folder outside this repo. 
  
- `git clone https://github.com/norlen/symex) repository`, and 

- `cd symex`. Now you can install the `cargo-symex` cargo sub command. 

- `cargo install --path cargo-symex` 

## Rust version

Due to LLVM dependencies the current implementation is limited to Rust < 1.64. cd to the folder where you want to run cargo symex from and override the Rust version to be used as below:

## Exercises