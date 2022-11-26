# Ex1

In this exercise you will get familiar with the `symex` tool. It is still fairly primitive but shows the principles of symbolic execution, which is the point here.

Run the tool:
```shell
cargo symex --example ex1 --function get_sign_test
```

If you get an error at this point check the install instructions. If still does not work let me know on the discord and we'll figure out a solution.

- Ex1 A1)

  [Paste your output here]

  If everything worked out correctly `symex` will report three paths (PATH 1, PATH 2 and PATH 3).

  For each path it should report `Success`, i.e., that the path was executed till end without any error (no assertion violations encountered).

- Ex1 A2) 
  
  Each path returns with a value (as the function `get_sign_test` is returning an `i32` typed value).

  The value is reported simply as a bit vector, so have a suitable converter at hand, e.g., [RapidTables](https://www.rapidtables.com/convert/number/binary-to-decimal.html).

  For PATH 1, the output should be trivial (0...01).
  Look at the source code to figure out what the path condition is for `get_sign` to return 1.

  [Your answer here]

- Ex1 A3)
  
  Under PATH 1 you also will find:

  ```shell
  Symbolic:
    a-2155673927: 0x40000000 (32-bits) // <- concrete assignment
  ```

  This amounts to a concrete assignment (value) of `v` triggering PATH 1. Now translate this value to an unsigned integer (the type of `v` is `i32`).

  [Your answer here]

- Ex1 A4)

  Does this value (`v`) meet the condition Ex1 A2)?

  [Yor answer here]

- Ex1 B1) 

  Now, let's look at the second path (PATH 2).

  Translate the `Success: returned` bit-vector value to an `i32`.

  [Your answer here]

- Ex1 B2) 

  Look at the source code to figure out what the path condition is for `get_sign` to hit this path.

  [Your answer here]

- Ex1 B3)
  
  Now translate the concrete value of `v` triggering this path to an unsigned integer (the type of `v` is `i32`).

  [Your answer here]

- Ex1 B4)

  Does this value meet the condition Ex1 B2)?
  
  [Yor answer here]

- Ex1 C1)

  Finally we repeat this for PATH 3.

  Translate the return value to an `i32`.

  [Your answer here]

- Ex1 C2) 

  Look at the source code to figure out what the path condition is for `get_sign` to hit this path.

  [Your answer here]

- Ex1 C3)

  Now translate the concrete value of `v` triggering this path to an unsigned integer (the type of `v` is `i32`).

  [Your answer here]

- Ex1 C4)

  Does this value meet the condition Ex1 C2)?
  
  [Yor answer here]

Remember to check in your changes to this file.
