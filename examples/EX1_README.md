# Ex1

In this exercise you will get familiar with the `symex` tool. It is still fairly primitive but shows the principles of symbolic execution, which is the point here.

The `cargo-symex` cli toll integrates as a sub-command `cargo symex`. You can invoke it with `--help` to get the set of features. For this lab you need to give the `--elf` option, since we want to analyze the code at binary level.

Run the tool:
```shell
cargo symex --elf --example ex1 --function get_sign
```

If you get an error at this point check the install instructions. If still does not work let me know on the discord and we'll figure out a solution.

- Ex1 A1)

  *Output :*

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Success: returned void

Symbolic:
    R0: 0x00000001 (32-bits)

End state:
    R1: 0x00000001 (32-bits)
    SP: 0x20040000 (32-bits)
    LR: 0xfffffffe (32-bits)
    R0: 0x00000001 (32-bits)
    PC: 0xfffffffe (32-bits)
Instructions executed: 9
Max number of cycles: 10
Stack usage: 0 bytes

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 2 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Success: returned void

Symbolic:
    R0: 0x80000001 (32-bits)

End state:
    R1: 0xffffffff (32-bits)
    SP: 0x20040000 (32-bits)
    LR: 0xfffffffe (32-bits)
    R0: 0xffffffff (32-bits)
    PC: 0xfffffffe (32-bits)
Instructions executed: 9
Max number of cycles: 9
Stack usage: 0 bytes

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 3 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Success: returned void

Symbolic:
    R0: 0x00000000 (32-bits)

End state:
    R1: 0x00000000 (32-bits)
    SP: 0x20040000 (32-bits)
    LR: 0xfffffffe (32-bits)
    R0: 0x00000000 (32-bits)
    PC: 0xfffffffe (32-bits)
Instructions executed: 9
Max number of cycles: 9
Stack usage: 0 bytes

time: 6.383128ms


  If everything worked out correctly `symex` will report three paths (PATH 1, PATH 2 and PATH 3).

  For each path it should report `Symbolic:` (reflecting the symbolic argument register(s)), and `End State:` (the final values for each register being touched). You should also see the aggregated number of assembly instruction executed along the path, and the estimated (upper bound) execution time in CPU cycles.

- Ex1 A2) 
  
  Each path returns with a value (as the function `get_sign` is returning an `i32` typed value). Arguments is stored in the `R0..R3` (additional arguments are passed on the stack). Results come in `R0, R1` (essentially following/extending on the C ABI). For mare info you can look at e.g. [Arm Blog](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/on-the-aapcs-with-an-application-to-efficient-parameter-passing), or even the ARM official spec [ARM ABI](https://github.com/ARM-software/abi-aa/releases), (but its a lot of info to dig through). 

  The register values are written in hex, use a converter e.g., [RapidTables](https://www.rapidtables.com/convert/number/binary-to-decimal.html).

  For each path now look at the `Symbolic:` and `End State` and carefully match which reported path, that corresponds to each path in the source code:
  
  Path 1
  
  *Input : 1*

  *Result : 1*

  Path 2
  
  *Input : -2147483647*

  *Result : -1*

  Path 3
  
  *Input : 0*

  *Result : 0*

- Ex1 B1)

  Now look at the function `addu`, it takes a single value, and returns a tuple (first element is the input value, second element is double its value).

  Run the example:

  ```shell
  cargo symex --elf --example ex1 --function addu
  ```
  *Output :*

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Success: returned void

Symbolic:
    R0: 0x00000000 (32-bits)

End state:
    PC: 0xfffffffe (32-bits)
    SP: 0x20040000 (32-bits)
    LR: 0xfffffffe (32-bits)
    R0: 0x00000000 (32-bits)
    R1: 0x00000000 (32-bits)
Instructions executed: 3
Max number of cycles: 5
Stack usage: 0 bytes

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 2 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Error: panic

Symbolic:
    R0: 0x80000000 (32-bits)
    R7: 0x00000000 (32-bits)

End state:
    PC: 0x000001d8 (32-bits)
    SP: 0x2003fff8 (32-bits)
    LR: 0x00000167 (32-bits)
    R0: 0x00000210 (32-bits)
    R1: 0x00000000 (32-bits)
    R7: 0x2003fff8 (32-bits)
Instructions executed: 8
Max number of cycles: 11
Stack usage: 8 bytes

time: 6.497159ms


  Rust is designed for safety, so it generates a panic (by default) if an unsigned integer addition overflows (or rather wraps around under modular arithmetics).

  [Which input value in unsigned integer in decimal triggered the panic]

  *Input that triggered the panic : R0 = 0 and R7 = 2147483647*

  You should also have a success path.

  [Which input value in unsigned integer in decimal triggered the success path]

  *Input that triggered the succes : 0*

  [Which was the corresponding result (R1) in unsigned decimal]

  *Corresponding result : 0*

  Now fix the code such that it returns the `u32::MAX` in case the value would wrap. Hint, you can use an `if then else` expression here.

  Run the example, to validate that there are no panics.

  How many paths did you have.

  *I get two paths*

  Now let us analyze the paths.

  [Which input value in unsigned integer in decimal triggered a normal add]

  *Input value that triggered a normal add : 2147483646*

  [Which was the corresponding result (R1) in unsigned decimal]

  *Corresponding result : R1 = 4294967292*

  [Which input value in unsigned integer in decimal triggered a saturating add]

  *Input value that trigger a saturating add : 4294967294*

  [Which was the corresponding result (R1) in unsigned decimal]

  *Corresponding result : R1 = 4294967295*

  Add/commit/push as a new git branch `B1`.

- Ex1 B2)

  Now look at the function `addi`, it takes a single value, and returns a tuple (first element is the input value, second element is double its value).

  Run the example:

  ```shell
  cargo symex --elf --example ex1 --function addi
  ```

  *Output :*

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Success: returned void

Symbolic:
    R0: 0xffffffff (32-bits)

End state:
    LR: 0xfffffffe (32-bits)
    R0: 0xffffffff (32-bits)
    PC: 0xfffffffe (32-bits)
    R1: 0xfffffffe (32-bits)
    SP: 0x20040000 (32-bits)
Instructions executed: 3
Max number of cycles: 5
Stack usage: 0 bytes

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 2 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Error: panic

Symbolic:
    R0: 0x7fffffff (32-bits)
    R7: 0x00000000 (32-bits)

End state:
    LR: 0x00000175 (32-bits)
    R0: 0x00000208 (32-bits)
    PC: 0x000001d0 (32-bits)
    R1: 0xfffffffe (32-bits)
    R7: 0x2003fff8 (32-bits)
    SP: 0x2003fff8 (32-bits)
Instructions executed: 8
Max number of cycles: 11
Stack usage: 8 bytes


  Rust is designed for safety, so it generates a panic (by default) if a signed integer addition overflows.

  [Which input value in signed integer in decimal triggered the panic]

  *Input that triggered the panic : R0 = 2147483647 and R7 = 0*

  You should also have a success path.

  [Which input value in signed integer in decimal triggered the success path]

  *Input that triggered the succes : 4294967295*

  [Which was the corresponding result (R1) in signed decimal]

  *Corresponding result : R1 = 4294967294*

  Now fix the code such that it returns the `i32::MAX` in case the value would overflow on the positive side and `i32::MIN` if it would overflow on the negative side. Hint, you can use a nested `if then else` expression here.

  Run the example, to validate that there are no panics.

  How many paths did you have.

  *I get three paths*

  Now let us analyze the paths.

  [Which input value in signed integer in decimal triggered a normal add]

  *Input value that triggered a normal add : 0*

  [Which was the corresponding result (R1) in signed decimal]

  *Corresponding result : R1 = 0*

  [Which input value in unsigned integer in decimal triggered a positive saturating add]

  *Input value that triggered a positive saturating add : 2147483647*

  [Which was the corresponding result (R1) in unsigned decimal]

  *Corresponding result : R1 = 2147483647*

  [Which input value in unsigned integer in decimal triggered a negative saturating add]

  *Input value that triggered a negative saturating add : -2147483648*

  [Which was the corresponding result (R1) in unsigned decimal]

  *Corresponding result : R1 = -2147483648*

  Add/commit/push as a new git branch `B2`.

---

Learning outcomes:

The idea here is to exercise analysis, panic detection and avoidance (not necessarily coming up with the most effective solution possible). 

The examples shown are trivial, you could probably have come up with panic:ing solutions by hand. However, for more complex examples `symex` holds your hand. Our tool will automatically find ALL paths that lead up to a panic, not relying on You to figure them out manually (by e.g., extensive unit testing).

Already at this point you have learned to reap the benefits of symbolic execution though the `symex` tool is in early stages.

As an aside: Regarding saturating arithmetics, if extreme/optimal performance is required, the Rust compiler allows to exploit hardware supported saturating arithmetics by means of compiler intrinsics, in this case the [saturating_add](https://doc.rust-lang.org/std/intrinsics/fn.saturating_add.html) intrinsic. However, (this) intrinsic is not yet stabilized thus requires the nightly compiler. 






  