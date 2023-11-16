# Instructions to install

These instructions assume the `ubuntu` distribution. Similar setup should apply for other distributions.

## Install WSL2

WSL2 is preferred if possible. Remember to turn on virtualization in your BIOS, else it will default to WSL1.

Follow the instructions for installing WSL2 `ubuntu` [here](https://learn.microsoft.com/en-us/windows/wsl/install).

## Installing Rust and friends

The following has been tested using `Windows Terminal` with `Power Shell`.

Exit any running WSL2 instances and the terminal/prompt you used to install WLS `ubuntu`. 

Start a new `Windows Terminal` in user mode (not `Administrator`), and WSL2 `ubuntu` by:

```shell
wsl
cd
```

`wsl` starts the WSL2 `ubuntu`, and by default you will be in the default folder for your `Windows Terminal`. For working in the linux world you want to do `cd`, putting you in your linux home folder. It is adviced for performance and user experience to use the linux file system.

### Installing Rust

Run in terminal:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose the default install and run:

```shell
source "$HOME/.cargo/env"
```

This gives you a local install of Rust in the linux world. This install can be run side by side to Rust installed natively in Windows (and YES, you might want that as well but not needed here.)

You can now try that it works:

```shell
cargo
```

If this fails, you have to go back and try to figure out the problem.

### Cloning lab3

Now we want to clone the lab3:

```shell
git clone https://vesuvio-git.neteq.ltu.se/pln/d7020e_lab3.git --branch master
```

Notice, the `--branch master`, its needed to override the politically correct `--branch main` default.

Now you should be able to `cd` to the cloned repo and run:

```shell
code .
``` 

 This will use the Windows `vscode` installation so you don't need a local linux install, however you will notice that the plugins you have in windows, if any, needs local install for you WSL domain. So you can now install `rust-analyser`, and other friends of `code`.

### Installing cargo-symex

Before we can install `cargo-symex`, we first need to fix some dependencies:

```shell
sudo apt-get update
sudo apt install build-essential
sudo apt-get install cmake
```

Given this worked out, you should be good to go:

```shell
cargo install --git https://github.com/s7rul/symex --branch armv6-m cargo-symex
```

You can test that you got it right by:

```shell
cargo symex --help
```

From this and on you should be able to [continue](README.md#rust-version) happily with the lab, WSL is almost linux :)





