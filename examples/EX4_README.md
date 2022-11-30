# Ex4

This is optional extra assignments for higher grades.

## Functional equality

Assume we have an implementation for the `sum` function:

```rust
fn sum_recursive(n: u8) -> u32 {
    match n {
        0 => 0,
        _ => n as u32 + sum_recursive(n - 1),
    }
}
```

This implementation is very close to the inductive specification of summation of natural numbers spanning from 0 to n. So we consider this to be our reference implementation.

---

## A) Problem with recursion

However, as we are concerned with the performance (not just correctness) we see potential problem(s) with this implementation.

Formulate in your own words what potential problem(s) you see with this approach.

[Your answer here]

Hint: What does a recursive call imply in the general case.

---

## B) Iterative solution

To mitigate the problem of the recursive implementation we came up with an iterative implementation.

```rust
fn sum_iterative(n: u8) -> u32 {
    let mut sum = 0;
    for v in 0..n {
        sum += v as u32
    }
    sum
}
```

In order to "replace" the trusted recursive implementation with the iterative one, we want to prove them equal. 

```rust
// test equal
pub fn equal_iter_rec() {
    let n = u8::any();
    // assume(n < 10);
    assert!(sum_iterative(n) == sum_recursive(n));
}
```

Run:
```shell
cargo symex --example ex4 --function equal_iter_rec --release
```

After you have seen a couple PATHs being produced you can now press `Ctrl-C` to quit `symex`. 

A few things here to notice.

- For this example, each `n` will render a new path. So the number of paths is linear to `n`. `n` is this case can be any number that fits into a `u8` (so 2**8 = 256). Under the hood, the SMT solver will try to satisfy constraints with increasing complexity, so the execution time is likely exponential in `n`. This is the worst case scenario for symbolic execution.

- Looking at the generated output we see that PATH 2 and higher all run into `Error: Abort -1`, so even if we quit `symex` early, we still have useful errors to work with.

Now look at the `sum_iterative` code in detail, identify the problem and fix the bug.

Hint. It passes for `n == 0` but not for `n == 1`, so there is something wrong with the range in the for loop.

After the fix, the iterative and recursive implementations should render the same result, for any `n`, but as mentioned from complexity point of view we hit the worst case scenario for symbolic execution, and it will take lots of time to let `symex` cover all possible assignments of `n`. So abort once you have seen enough PATHs passing. 

You can uncomment `// assume(n < 10)` to strengthen the initial path condition, in effect you will see ony the 10 first paths generated.

So now we have some 10 PATHs, with concrete assignments of `n` (0, 1 , ..., 9). Although, this not a complete proof of equality we see that the functions produce the same results for the covered input assignments, thus we have gained some confidence that the implementations will produce the same results for any `n`. 

There exists other (formal) methods to prove functional equality, e.g., deductive program verification using induction [Mathematical induction](https://en.wikipedia.org/wiki/Mathematical_induction).

Such formal approaches are very powerful but hard to master, and has thus not gained uptake outside of very niche areas (e.g., avionics) as they are considered to hard and costly.

---

## C) Formula

You might have observed that there is an alternative mathematical formula `sum = n * (n + 1) / 2` that computes the summation of the first `n` natural numbers.

```rust
// mathematical formula
fn sum_formula(n: u8) -> u32 {
    let n: u32 = n as u32;
    n * (n + 1) / 2
}
```

Now let's verify for `n < 10` that `sum_recursive` and `sum_formula` produces the same results.

```rust
// test equal rec_formula
pub fn equal_rec_formula() {
    let n = u8::any();
    assume(n < 10);
    assert!(sum_recursive(n) == sum_formula(n));
}
```

Run the test:

```shell
cargo symex --example ex4 --function equal_rec_formula --release
```

Did the test pass?

[Your answer here]

Hint: The answer should not surprise you.

---

## D) Complexity

Under the hood, symbolic execution operates on a representation of the program at hand. 

`symex` is designed to be modular, separating the execution engine from the constrain solver. 

As of today `symex` provides an execution engine for `LLVM-IR`. This allows `symex` to be used with any language that uses `LLVM` for code generation (which includes, Rust, C, C++, etc.).

Application compiled to executables, are further optimized in the backend (on assembly level) and at link time.

The backend transformations may alter the control flow (introducing and/or reducing the number of paths). As an effect, there is not a 1-1 relation guaranteed between the PATHs reported and the actual execution paths (when run by hardware). However, all transformations are required to preserve the observable behavior of the program, i.e., any path that leads to a panic! (e.g. due to an assertion violation) is required to lead to a panic! also in the resulting binary.

Thus from a correctness point of view we can analyze the program at any level (e.g., at `MIR` level using [seer](https://crates.io/crates/seer), `LLVM-IR` using [KLEE](http://klee.github.io/), or at assembly level for x86 and ARM [SAGE](https://www.microsoft.com/en-us/research/publication/automated-whitebox-fuzz-testing/). (SAGE uses symbolic execution under the hood in fuzz-testing framework).

The paths observed at `LLVM-IR` express high (albeit not 100%) correlation to the paths observed in the generated binaries. Moreover, the instructions on `LLVM-IR` level are highly correlated (albeit not 100% also in this case) to the generated assembly instructions. In effect, the symbolic execution is highly correlated to the concrete execution of a produced binary.

For more information see, the recent Master's projects at LTU:

- [Calculation of WCET with symbolic execution](https://ltu.diva-portal.org/smash/record.jsf?aq2=%5B%5B%5D%5D&c=37&af=%5B%5D&searchType=LIST_LATEST&sortOrder2=title_sort_asc&query=&language=sv&pid=diva2%3A1689116&aq=%5B%5B%5D%5D&sf=all&aqe=%5B%5D&sortOrder=author_sort_asc&onlyFullText=false&noOfRows=50&dswid=-2424)

- [Control Flow Based Static Execution Time Analysis Using Symbolic Execution](http://www.diva-portal.org/smash/record.jsf?pid=diva2%3A1689154&dswid=-2622)

- [RAUK: Automatic Schedulability Analysis of RTIC Applications Using Symbolic Execution](https://ltu.diva-portal.org/smash/record.jsf?pid=diva2%3A1652205&dswid=-6163)

And prior research:

- [No Panic! Verification of Rust Programs by Symbolic Execution](http://ltu.diva-portal.org/smash/record.jsf?faces-redirect=true&aq2=%5B%5B%5D%5D&af=%5B%5D&searchType=SIMPLE&sortOrder2=title_sort_asc&query=&language=sv&pid=diva2%3A1256728&aq=%5B%5B%5D%5D&sf=all&aqe=%5B%5D&sortOrder=author_sort_asc&onlyFullText=false&noOfRows=50&dswid=1069)

- [Verification of Safety Functions Implemented in Rust: a Symbolic Execution based approach](http://ltu.diva-portal.org/smash/record.jsf?pid=diva2%3A1426544&dswid=-2583)


Now let us inspect the our three implementations.

```shell
cargo symex --example ex4 --function complexity_sum_recursive --release

cargo symex --example ex4 --function complexity_sum_iterative --release

cargo symex --example ex4 --function complexity_sum_formula --release
```

For each test, report the `Instructions processed`:

[Instructions processed for: complexity_sum_recursive]

[Instructions processed for: complexity_sum_iterative]

[Instructions processed for: complexity_sum_formula]

In your own words, try to explain the results obtained. Why did the complexity_sum_formula just produce one path?

[Your answer here]

---

## E) Real hardware

In lab1/2 we were analyzing different properties of RTIC applications.

In this part copy the lab1 `rtt_timing.rs` to a new file `rtt_sum.rs`, and copy the three sum implementations there. (You should later run these on hardware, to characterize their timing properties.) Give the `#[inline(never)]` attribute to each sum implementation so that you can find them in generated binary.

Change the `timed_loop`, so that instead of the running a simple `nop`, it should call one of the summation functions.

```rust
#[inline(never)] // Forbid inlining.
// #[link_section = ".data.timed_loop"] // for placing the function in RAM
fn timed_loop() -> (u32, u32) {
    let start = DWT::cycle_count();
    for n in 0..10 {
        // let _ = sum_recursive(n as u8); // uncomment one at a time
        // let _ = sum_iterative(n as u8);
        // let _ = sum_formula(n as u8);
    }
    let end = DWT::cycle_count();
    (start, end)
}
```

As we want to run with overflow checks, you need to change the `Cargo.toml`.

```toml
[profile.release]
...
overflow-checks = true
```

Now, enable (uncomment) the first sum implementation (`sum_recursive`) and run on hardware in `--release` mode.

- A) `sum_recursive`
  
[Your sum_recursive, cycle-count here]

Now, comment out `sum_recursive` and uncomment `sum_iterative`, and run on hardware in `--release` mode.

- B) `sum_iterative`

[Your sum_iterative, cycle-count here]

Finally repeat for `sum_formula`.

- C) `sum_formula`
  
[Your sum_formula, cycle-count here]

If you run with/without cache enabled or even from RAM does not really matter, we are just interested in the relative execution time (so keep it the same for all test-runs). However, use `--release` builds for all cases (in debug/dev mode Rust generates lots of extra overhead that makes comparisons hard to assess).

Discuss in your own words the results in terms of cycle-counts between the sum implementations.

- D) Discussion regarding cycle-counts

[Your discussion regarding cycle-counts]

Discuss in your own words the results compared to the `Instructions processed` for the sum implementations. Do you see a correlation between the `symex` results and the actual execution. 

- E) Discussion cycle-counts vs Instructions processed

[Your discussion regarding cycle-counts vs Instructions processed]

Now with all three sum implementations enabled make an objdump of the application and paste the generated assembly code for each function below:

- F) Assembly for `sum_recursive`

[Your assembly for `sum_recursive`]

- G) Assembly for `sum_iterative`

[Your assembly for `sum_iterative`]

- H) Assembly for `sum_formula`

[Your assembly for `sum_formula`]

Now, for each assembly implementation F) G) H) add comments to the assembly showing what source code statements correspond to what assembly level instructions, what registers are used for what etc.

With this at hand, discuss again the question D, results in terms of cycle-counts, and try to explain the observed results for `sum_recursive`, `sum_iterative`, and `sum_formula` related to the assembly.

- I)

[Your discussion of cycle-counts vs assembly]

Now, we can return to E), discuss in your own words how the mapping from LLVM-IR to assembly did affect the results (e.g., did you get some unexpected results, and if so could they be explained)


















 



