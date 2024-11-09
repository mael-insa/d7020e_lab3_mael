# Ex1

In this exercise you will get familiar with the `symex` tool. It is still fairly primitive but shows the principles of symbolic execution, which is the point here.

The `cargo-symex` cli toll integrates as a sub-command `cargo symex`. You can invoke it with `--help` to get the set of features. For this lab you need to give the `--elf` option, since we want to analyze the code at binary level.

Run the tool:
```shell
cargo symex --elf --example ex1 --function get_sign
```

If you get an error at this point check the install instructions. If still does not work let me know on the discord and we'll figure out a solution.

- Ex1 A1)

  [Paste your output here]

  If everything worked out correctly `symex` will report three paths (PATH 1, PATH 2 and PATH 3).

  For each path it should report `Symbolic:` (reflecting the symbolic argument register(s)), and `End State:` (the final values for each register being touched). You should also see the aggregated number of assembly instruction executed along the path, and the estimated (upper bound) execution time in CPU cycles.

- Ex1 A2) 
  
  Each path returns with a value (as the function `get_sign` is returning an `i32` typed value). Arguments is stored in the `R0..R3` (additional arguments are passed on the stack). Results come in `R0, R1` (essentially following/extending on the C ABI). For mare info you can look at e.g. [Arm Blog](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/on-the-aapcs-with-an-application-to-efficient-parameter-passing), or even the ARM official spec [ARM ABI](https://github.com/ARM-software/abi-aa/releases), (but its a lot of info to dig through). 

  The register values are written in hex, use a converter e.g., [RapidTables](https://www.rapidtables.com/convert/number/binary-to-decimal.html).

  For each path now look at the `Symbolic:` and `End State` and carefully match which reported path, that corresponds to each path in the source code:
  
  Path 1
  
  [Which input value v as a signed integer in decimal]

  [Which result value as a signed integer in decimal]

  Path 2
  
  [Which input value v as a signed integer in decimal]

  [Which result value as a signed integer in decimal]

  Path 3
  
  [Which input value v as a signed integer in decimal]

  [Which result value as a signed integer in decimal]

- Ex1 B1)

  Now look at the function `addu`, it takes a single value, and returns a tuple (first element is the input value, second element is double its value).

  Run the example:

  ```shell
  cargo symex --elf --example ex1 --function addu
  ```

  [Paste output here]

  Rust is designed for safety, so it generates a panic (by default) if an unsigned integer addition overflows (or rather wraps around under modular arithmetics).

  [Which input value in unsigned integer in decimal triggered the panic]

  You should also have a success path.

  [Which input value in unsigned integer in decimal triggered the success path]

  [Which was the corresponding result (R1) in unsigned decimal]

  Now fix the code such that it returns the `u32::MAX` in case the value would wrap. Hint, you can use an `if then else` expression here.

  Run the example, to validate that there are no panics.

  How many paths did you have.

  [Your answer here]

  Now let us analyze the paths.

  [Which input value in unsigned integer in decimal triggered a normal add]

  [Which was the corresponding result (R1) in unsigned decimal]

  [Which input value in unsigned integer in decimal triggered a saturating add]

  [Which was the corresponding result (R1) in unsigned decimal]

  Add/commit/push as a new git branch `B1`.

- Ex1 B2)

  Now look at the function `addi`, it takes a single value, and returns a tuple (first element is the input value, second element is double its value).

  Run the example:

  ```shell
  cargo symex --elf --example ex1 --function addi
  ```

  [Paste output here]

  Rust is designed for safety, so it generates a panic (by default) if a signed integer addition overflows.

  [Which input value in signed integer in decimal triggered the panic]

  You should also have a success path.

  [Which input value in signed integer in decimal triggered the success path]

  [Which was the corresponding result (R1) in signed decimal]

  Now fix the code such that it returns the `i32::MAX` in case the value would overflow on the positive side and `i32::MIN` if it would overflow on the negative side. Hint, you can use a nested `if then else` expression here.

  Run the example, to validate that there are no panics.

  How many paths did you have.

  [Your answer here]

  Now let us analyze the paths.

  [Which input value in signed integer in decimal triggered a normal add]

  [Which was the corresponding result (R1) in signed decimal]

  [Which input value in unsigned integer in decimal triggered a positive saturating add]

  [Which was the corresponding result (R1) in unsigned decimal]

  [Which input value in unsigned integer in decimal triggered a negative saturating add]

  [Which was the corresponding result (R1) in unsigned decimal]

  Add/commit/push as a new git branch `B2`.

---

Learning outcomes:

The idea here is to exercise analysis, panic detection and avoidance (not necessarily coming up with the most effective solution possible). 

The examples shown are trivial, you could probably have come up with panic:ing solutions by hand. However, for more complex examples `symex` holds your hand. Our tool will automatically find ALL paths that lead up to a panic, not relying on You to figure them out manually (by e.g., extensive unit testing).

Already at this point you have learned to reap the benefits of symbolic execution though the `symex` tool is in early stages.

As an aside: Regarding saturating arithmetics, if extreme/optimal performance is required, the Rust compiler allows to exploit hardware supported saturating arithmetics by means of compiler intrinsics, in this case the [saturating_add](https://doc.rust-lang.org/std/intrinsics/fn.saturating_add.html) intrinsic. However, (this) intrinsic is not yet stabilized thus requires the nightly compiler. 






  