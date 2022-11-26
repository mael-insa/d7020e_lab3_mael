
# Ex2

In this exercise we will look at a release build of the same program. Run:

```shell
cargo symex --example ex2 --function get_sign_test --release
```

Surprisingly enough you will get just one path. This is since in release build Rust will make aggressive optimizations, and `symex` will find that all paths end up at successfully returning the value of `get_sign(v)`.

One way to ensure that `symex` finds each path is to match the result of `get_sign(v)` against the expected values, `0, 1, -1`, and for each arm force an error by `panic!()`, as done in the `get_sign_test_release` function. 

- Ex2 A1)
  
  Now run:

  ```shell
  cargo symex --example ex2 --function get_sign_test_release --release
  ```
  
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

