# Ex2

In this exercise we will look at a release build of the `timed_loop` also found in (`d7020e_lab1`).

- Ex2 A1)

  Let us first have a look at the generated assembly.

  ```shell
  cargo objdump --example ex2 --release -- --disassemble > ex2.objdump
  ```

  In the generated `ex2.objdump`, locate the `timed_loop` function.

  [Paste assembly here]

  Explain in your own words how the loop counter was initialized to 10000. Hint, confer to the documentation of the instruction set [Cortex-M4](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en).

  [Your answer here]

- Ex2 A2)

  Now its time (pun intended) to look at the timing estimation.

  ```shell
  cargo symex --elf --example ex2 --function timed_loop --release
  ```

  [Paste your result here]

  Explain in your own words, why only one path was reported.

  [Your answer here]

  Do the number of assembly instructions match your expectations (relate to the generated assembly code).

  [Your answer here]

  Do the number of cycles reported match your expectations. (Lookup the `nop`, `subs` and `b<cc>` instructions in [Cortex-M4](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en)).

  What do you find? (Hint, what is the assumption on `p` made by Symex.)

  [Your answer here]

---

Learning outcomes:

In this exercise you have seen a simple example showing how `symex` can safely estimate the number of clock cycles executed along each path.

In general, the number of paths can be large, so we need additional tooling to collect and extract desired information, here we have just covered the basics.

This example highlights the problem with (commercial) closed source products. Here we cannot from the ARM documentation alone deduce a cycle accurate prediction of the pipeline filling and branch prediction. 

One could potentially reverse engineer the pipeline design and predictor using cleverly designed tests/cycle accurate measurements. 

However, a better solution is to leave closed source solutions in favour of open source implementations (e.g., RISC-V based). The downside is that initial time/cost will be higher (chip design is time consuming and costly). FPGAs (programmable logic) can often be used for prototyping and low volume production, but are always inferior when it comes to performance (clocks speed/power and per unit cost) compared to custom chips (ASICs).

The open source RISC-V based [Hippomenes](https://github.com/perlindgren/hippomenes) processor (developed at LTU) provides an executable (synthesizable) specification with fully predictable behavior, thus allows for exact cycle time analysis.