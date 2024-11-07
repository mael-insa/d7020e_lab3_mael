# Ex2

In this exercise we will look at a release build of the `timed_loop` from previous lab (`d7020e_lab1`).

- Ex2 A1)

  Since we are compiling for a slightly different architecture (the more primitive `armv7m` ISA), we first want to have a look at the generated assembly.

  ```shell
  cargo objdump --example ex2 --release -- --disassemble > ex2.objdump
  ```

  In the generated `ex2.objdump`, locate the `timed_loop` function. Paste the assembly (including the `.word ...` trailing the function).

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

  Compare to the result you obtained for `timed_loop` in `d7020e_lab1`.

  Does the estimate fall within the range you observed for release mode previously?

  [Your answer here]

  Do the number of assembly instructions match your expectations (relate to the generated assembly code).

  [Your answer here]

  Do the number of cycles reported match your expectations. (Lookup the `nop`, `subs` and `b <cc>` instructions in [Cortex-M4](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en)).

  What do you find?

  [Your answer here]

---

Learning outcomes:

In this exercise you have seen a simple example showing how `symex` can safely estimate the number of clock cycles executed along each path.

In general, the number of paths can be large, so we need additional tooling to collect and extract desired information, here we have just covered the basics.
