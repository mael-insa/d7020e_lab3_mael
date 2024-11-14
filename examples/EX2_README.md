# Ex2

In this exercise we will look at a release build of the `timed_loop` also found in (`d7020e_lab1`).

- Ex2 A1)

  Let us first have a look at the generated assembly.

  ```shell
  cargo objdump --example ex2 --release -- --disassemble > ex2.objdump
  ```

  In the generated `ex2.objdump`, locate the `timed_loop` function.

  0000013c <timed_loop>:
      13c: f242 7010    	movw	r0, #0x2710
      140: 3801         	subs	r0, #0x1
      142: bf00         	nop
      144: d1fc         	bne	0x140 <timed_loop+0x4>  @ imm = #-0x8
      146: 4770         	bx	lr


  Explain in your own words how the loop counter was initialized to 10000. Hint, confer to the documentation of the instruction set [Cortex-M4](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en).

 The line "13c: f242 7010    	movw	r0, #0x2710" is used to set the counter to 10000. The movw instruction is used to set the counter to a precise value, in this case 0x2710 in hexedecimal or 10000 in decimal.

- Ex2 A2)

  Now its time (pun intended) to look at the timing estimation.

  ```shell
  cargo symex --elf --example ex2 --function timed_loop --release
  ```

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  End state:
      SP: 0x20040000 (32-bits)
      LR: 0xfffffffe (32-bits)
      PC: 0xfffffffe (32-bits)
      R0: 0x00000000 (32-bits)
  Instructions executed: 30002
  Max number of cycles: 60002
  Stack usage: 0 bytes

  Explain in your own words, why only one path was reported.

  *Only one path was reported because there isn't any if in the function timed_loop but lonly a for so it means that there is only path, the one throught the whole function and the for loop*

  Do the number of assembly instructions match your expectations (relate to the generated assembly code).

  *My expectations were more likely 50002 rather than 30002 because we execute 10000 times the function timed_loop which is composed of 5 instructions which means that we will have 50000 instructions for the loop and then we have 2 instructions for the main leading us to 50002 instructions maybe due to some optimization process* 

  Do the number of cycles reported match your expectations. (Lookup the `nop`, `subs` and `b<cc>` instructions in [Cortex-M4](https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions?lang=en)).

  What do you find? (Hint, what is the assumption on `p` made by Symex.)

  For each instruction of the function timed_loop, we find this number of cycle :

  movw = 1 cycle
  subs = 1 cycle 
  nop = 1 cycle 
  bx = 1 + P cycle
  bne = 1 cycle (comparaison)

  We can easly determine that P = 1 and this gives us a number of cycle = 60002, which is the same as the one found in the path 1.


---

Learning outcomes:

In this exercise you have seen a simple example showing how `symex` can safely estimate the number of clock cycles executed along each path.

In general, the number of paths can be large, so we need additional tooling to collect and extract desired information, here we have just covered the basics.

This example highlights the problem with (commercial) closed source products. Here we cannot from the ARM documentation alone deduce a cycle accurate prediction of the pipeline filling and branch prediction. 

One could potentially reverse engineer the pipeline design and predictor using cleverly designed tests/cycle accurate measurements. 

However, a better solution is to leave closed source solutions in favour of open source implementations (e.g., RISC-V based). The downside is that initial time/cost will be higher (chip design is time consuming and costly). FPGAs (programmable logic) can often be used for prototyping and low volume production, but are always inferior when it comes to performance (clocks speed/power and per unit cost) compared to custom chips (ASICs).

The open source RISC-V based [Hippomenes](https://github.com/perlindgren/hippomenes) processor (developed at LTU) provides an executable (synthesizable) specification with fully predictable behavior, thus allows for exact cycle time analysis.