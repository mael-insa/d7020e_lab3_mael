
# Ex2

In this exercise we will look at a release build of the `timed_loop` from previous lab (`d7020e_lab1`). 

- Ex2 A1)

  Since we are compiling for a slightly different architecture (the more primitive `armv6m` ISA), we first want to have a look at the generated assembly.

  ```shell
  cargo objdump --example ex2 --release -- --disassemble > ex2.objdump
  ```

  In the generated `ex2.objdump`, locate the `timed_loop` function. Paste the assembly (including the `.word ...` trailing the function).

  [Paste assembly here]

  Explain in your own words how the loop counter was initialized to 10000. Hint, confer to the documentation of the instruction set [Cortex-M0+](https://developer.arm.com/documentation/ddi0484/c/CHDCICDF).

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

  Do the number of cycles reported match your expectations. (Lookup the `nop`, `subs` and `b <cc>` instructions in [Cortex-M0+](https://developer.arm.com/documentation/ddi0484/c/CHDCICDF)).

  What do you find?

  [Your answer here]

  As you might have noticed there is a discrepancy. Now lookup the same instructions for the older M0 sibling [Cortex-M0](https://developer.arm.com/documentation/ddi0432/c/programmers-model/instruction-set-summary). These both implement the same `arm6m` ISA, with different to their implementation only.

  What do you find?

  [Your answer here]

  An in particular, which implementation M0/M0+, do you infer `symex` to model?

  [Your answer here]

---

Learning outcomes:

In this exercise you have seen a simple example showing how `symex` can exactly determine the number of clock cycles executed along each path.

In general, the number of paths can be large, so we need additional tooling to collect and extract desired information, here we have just covered the basics.





  

  https://developer.arm.com/documentation/ddi0484/c/CHDCICDF

As the `timed_loop` function does not take any arguments, the code is completely deterministic. Thus there will be only one path.
  
  [Paste output here]

  If everything runs as expected you should now have 3 paths. (If not go back and check again. If still you don't get three paths let me know on Discord.)

  You should now see that the number of instructions that `symex` interpreted to find these paths now has been significantly reduced. The optimization helps when dealing with larger programs (this one is rather trivial of course and not a problem in debug build.)

  How many instructions where processed:

  [Your answer here]

- Ex2 A2)

  Now correlated the concrete assignments of the symbolic variable `v` to the different arms of the match.

  You may find that the concrete values derived by `symex` (or rather the SMT solver) for each path differs from the one in Ex 1.

  What value of `v` triggered `match 0`.

  [Your answer here] 

- Ex2 A3)
  
  What value of `v` triggered `match 1`.

  [Your answer here] 

- Ex2 A4)
  
  What value of `v` triggered `match -1`.

  [Your answer here] 

