# Ex4

When re-factoring or optimizing code we want to assure that two implementations are functionally equivalent. Whereas the problem is fundamentally hard (and impossible to prove in an automated fashion), we can make use of symbolic execution to gain confidence regarding functional equality.

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
// test sum_iterative == sum_recursive
pub fn equal_iter_rec(n: u8) {
    assume(n < 10);
    assert!(sum_iterative(n) == sum_recursive(n));
}
```

Run:

```shell
cargo symex --elf --example ex4 --function equal_iter_rec --release
```

After you have seen a couple PATHs being produced you can now press `Ctrl-C` to quit `symex`.

A few things here to notice.

- For this example, each `n` will render a new path. So the number of paths is linear to `n`. `n` is this case can be any number that fits into a `u8` (so 2\*\*8 = 256). Under the hood, the SMT solver will try to satisfy constraints with increasing complexity, so the execution time is likely exponential in `n`. This is the worst case scenario for symbolic execution.

- Looking at the generated output we see that PATH 2 and higher all run into `Error: Abort -1`, so even if we quit `symex` early, we still have useful errors to work with.

Now look at the `sum_iterative` code in detail, identify the problem and fix the bug.

Hint. It passes for `n == 0` but not for `n == 1`, so there is something wrong with the range in the for loop.

After the fix, the iterative and recursive implementations should render the same result, for any `n`, but as mentioned from complexity point of view we hit the worst case scenario for symbolic execution, and it will take lots of time to let `symex` cover all possible assignments of `n`. So abort once you have seen enough PATHs passing.

The assumption `assume(n < 10)` strengthens the condition on `n` such to limit the problem.

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
cargo symex --elf --example ex4 --function equal_rec_formula --release
```

Did the test pass?

[Your answer here]

Hint: The answer should not surprise you.

---

## D) Complexity

Under the hood, symbolic execution operates on a representation of the program at hand.

`symex` is designed to be modular, separating the execution engine from the constrain solver.

`symex` provides an execution engines for `LLVM-IR` and `armv6m`. This allows `symex` `LLVM-IR` analysis to be used with any language that uses `LLVM` for code generation (which includes, Rust, C, C++, etc.). However, we need access to the `LLVM-IR`, so we cannot analyze inline assembly and code linked to our application (unless it is linked in as `LLVM-IR`, so libraries compiled with `gcc` is a no go).

More importantly, analysis at `LLVM-IR` level does not reflect the exact behavior of the target binary, as applications are further optimized in the backend (on assembly level) and at link time.

The backend transformations may alter the control flow (introducing and/or reducing the number of paths). As an effect, there is not a 1-1 relation guaranteed between the PATHs reported at `LLVM-IR` level and the actual execution paths (when run by hardware). However, all transformations are required to preserve the observable behavior of the program, i.e., any path that leads to a panic! (e.g. due to an assertion violation) is required to lead to a panic! also in the resulting binary.

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
cargo symex --elf --example ex4 --function complexity_sum_recursive --release

cargo symex --elf --example ex4 --function complexity_sum_iterative --release

cargo symex --elf --example ex4 --function complexity_sum_formula --release
```

For each test, report three values

- P: number of paths

- I: number of instructions for final path

- C: number of clock cycles for final path

Your answer for complexity_sum_recursive:

[P, I, C]

Your answer for complexity_sum_iterative:

[P, I, C]

Your answer for complexity_sum_formula:

[P, I, C]

In your own words, compare the P, I, C results obtained, and why did the complexity_sum_formula just produce one path?

[Your answer here]

Which one would you regarding performance?

[Your answer here]

---

## E) Real hardware

In `d7020e_lab1` we were analyzing timing properties of `timed_loop`. Copy the `rtt_timing.rs` into a new file `rtt_sum.rs` (next to the `rtt_timing.rs` file in the lab1).

Change the `timed_loop`, so that instead of the running a simple `nop`, it should call one of the summation functions.

Make sure you frequently commit and push changes to the lab1 repository, so that you can go back if you broke something.

```rust
fn timed_loop() -> (u32, u32) {
    let start = DWT::cycle_count();

    // under the assumption argument n < 10, 9 gave the worst case in `symex`
    // let _ = sum_recursive(9); // uncomment one at a time
    // let _ = sum_iterative(9);
    // let _ = sum_formula(9);

    let end = DWT::cycle_count();
    (start, end)
}
```

As we want a fair comparison run with overflow checks, you need to change the `Cargo.toml`.

```toml
[profile.release]
...
overflow-checks = true
```

Copy paste the `sum_recursive`, `sum_iterative` and `sum_formula` from lab3 into the `rtt_sum.rs` file.

- E1) `sum_recursive`

Now, enable (uncomment) the first sum implementation (`sum_recursive`) and run on hardware in `--release` mode.

[Your sum_recursive, cycle-count here]

- E2) `sum_iterative`

Now, comment out `sum_recursive` and uncomment `sum_iterative`, and run on hardware in `--release` mode.

[Your sum_iterative, cycle-count here]

- E3) `sum_formula`

Finally repeat for `sum_formula`.

[Your sum_formula, cycle-count here]

If you run with/without cache enabled or even from RAM does not really matter, we are just interested in the relative execution time (so keep it the same for all test-runs). However, use `--release` builds for all cases (in debug/dev mode Rust generates lots of extra overhead that makes comparisons hard to assess).

- E4) Discussion regarding cycle-counts

Discuss in your own words the results in terms of cycle-counts between the sum implementations.

[Your discussion regarding cycle-counts]

- E5) Discussion cycle-counts vs `symex` max number cycles

Discuss in your own words the results compared to the `Max number cycles` for the sum implementations. Do you see a correlation between the `symex` results and the actual execution.

Hints, you run under two slightly different ISAs, moreover the underlying processor implementation also differs, thus some deviance is expected. Moreover, the `nrf52840` unless perfectly aligned for the instruction cache has additional OH for instruction fetches and pipeline filling. Based on these insights discuss your obtained results.

[Your discussion regarding cycle-counts vs `symex` max number cycles]

Now with all three sum implementations enabled make an objdump of the application and paste the generated assembly code for each function below:

- E6) Assembly for `sum_recursive`

[Your assembly for `sum_recursive` armv7m (lab1)]

[Your assembly for `sum_recursive` armv6m (lab3)]

- E7) Assembly for `sum_iterative`

[Your assembly for `sum_iterative` armv7m (lab1)]

[Your assembly for `sum_iterative` armv6m (lab3)]

- E8) Assembly for `sum_formula`

[Your assembly for `sum_formula` armv7m (lab1)]

[Your assembly for `sum_formula` armv6m (lab3)]

Now, for each assembly implementation E6) E7) E8) comment on the difference between the generated code for cortex-m4 (lab1) and cortex-m0 (lab3).

- E9) Cycle-counts vs assembly

With this at hand, discuss again the question E4), results in terms of cycle-counts, and try to explain the observed results for `sum_recursive`, `sum_iterative`, and `sum_formula` related to the assembly. Can the deviations between the `symex` estimates and the measured cycle counts be explained.

[Your discussion of cycle-counts vs assembly]

- F) Optional. The Cortex-M4 (`armv7m`) is backwards compatible to the cortex-m0 (`armv6m`).

Look into the `.cargo/config.toml` and figure out changes needed to make it run with the `armv6m` ISA.

Once you got it working, make an objdump to confirm that the generated code is the same as in lab3. If not there is still something wrong, go back and fix.

Now repeat E1-3.

- F1) `sum_recursive`

[Your sum_recursive, cycle-count here]

Now, comment out `sum_recursive` and uncomment `sum_iterative`, and run on hardware in `--release` mode.

- F2) `sum_iterative`

[Your sum_iterative, cycle-count here]

Finally repeat for `sum_formula`.

- F3) `sum_formula`

Discuss in your own words the results obtained (did you get fewer/more cycles etc., when changed ISA).

[Discussion here]

Now compare to the `symex` results for each test F1, F2, F3.

Discuss in your own words, did you get closer to the `symex` estimates?

Hints. You have now eliminated differences induced by the ISA. However, there are still differences regarding the ISA implementation, memory accesses (caches/wait states), bus arbitration and pipeline filling. Based on these insights discuss your obtained results.

[Discussion here]

---

Learning objectives:

You should now have gained basic insights into the use of symbolic execution to:

- Test for functional equivalence. Can we replace one implementation by another?
- Symbolic execution to assess performance differences. Can we distinguish good implementations from bad ones regarding performance?
- Model base cycle estimation vs. measurements. How, close can estimates match real-life measurements?

In prior research at LTU, we developed an automated framework that generated concrete input assignments triggering each found path, which was fed to an automated test-bed that replayed each path on the actual target and measured the corresponding execution time. However, this tool was based on `cargo klee`, which is adopting _dynamic_ symbolic execution and thus may be inexact. Moreover, `klee` operates on the LLVM-IR, thus even without approximation, the actual paths running on the target might differ (due to backend code generation and optimizations). Adopting `symex --elf` addresses these shortcomings, and You can take part in developing a further improved worst case execution time (WCET) framework, challenging the best WCET tools!
