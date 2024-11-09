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

This implementation is very close to the inductive specification of summation of natural numbers spanning from `0` to `n`. So we consider this to be our reference implementation.

The function `sum` computes the arithmetic progression of the series `0` to `n`. [Arithmetic Progression](https://en.wikipedia.org/wiki/Arithmetic_progression).

We want to prove that the recursive implementation correctly computes the corresponding arithmetic progression, using symbolic execution.

```rust
pub fn equal_formula_rec(n: u8) {
    assume(n < 10);
    let r = sum_recursive(n);
    let m = n as u32 * (n as u32 + 1) / 2;
    assert_eq!(m, r);
}
```

Run:

```shell
cargo symex --elf --example ex4 --function equal_formula_rec --release
```

However, inherent to symbolic execution, we might run into path explosion. For the `sum_recursive` function, each `n` will render a new path. So the number of paths is linear to `n`. `n` is this case can be any number that fits into a `u8` (so 2\*\*8 = 256). Under the hood, the SMT solver will try to satisfy constraints with increasing complexity, so the execution time is likely exponential in `n`.

This is the worst case scenario for symbolic execution. To mitigate the path explosion problem we have made an assumption to `n < 10`. It is worth to notice that we now only validate correctness for values of `n < 10`. However, we still have reason to believe that if the first 10 comparisons succeed then it is likely that also the 11th would (which generalize to all number of `n` in this particular case). Looking at the code, the only obvious thing that might go wrong is that the result would overflow. The arithmetic sum can be formulated as `n(n+1)/2`, which for the maximum `n` (255) evaluates to 32640, which in turn can be represented by a `u32` (thus the result cannot overflow).

Symbolic execution can be seen as a lightweight approach to program verification. Fully fledged proofs typically require more advanced formal methods, e.g., deductive program verification using induction [Mathematical induction](https://en.wikipedia.org/wiki/Mathematical_induction).

Such formal approaches are very powerful but hard to master, and has thus not gained uptake outside of very niche areas (e.g., avionics) as they are considered to hard and costly.

For a broader overview see e.g. [Formal Verification](https://en.wikipedia.org/wiki/Formal_verification).

---

## A) Problem with recursion

However, as we are concerned with the performance (not just correctness) we see potential problem(s) with this recursive implementation.

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

Now look at the `sum_iterative` code in detail, identify the problem and fix the bug.

Hint. It passes for `n == 0` but not for `n == 1`, so there is something wrong with the range in the for loop.

After the fix, the iterative and recursive implementations should render the same result, for any `n`, but as mentioned from complexity point of view we hit the worst case scenario for symbolic execution, and it will take lots of time to let `symex` cover all possible assignments of `n`.

So now we have some 10 PATHs, with concrete assignments of `n` (0, 1 , ..., 9). Although, this not a complete proof of equality we see that the functions produce the same results for the covered input assignments, thus we have gained some confidence that the implementations will produce the same results for any `n`.

---

## C) The Symex symbolic execution engine

Under the hood, symbolic execution operates on a representation of the program at hand.

`symex` is designed to be modular, separating the execution engine from the constrain solver.

For this lab `symex` performs symbolic execution on the `elf` binary (so it is simulating the machine code instructions according to the ARM documentation). In this way, paths found by `symex` equates 1-1 to paths taken by the actual processor. In this way, we can estimate both execution time and stack memory depth.

`symex` provides an execution engines for `LLVM-IR` and `armv6m`. `armv7em`. This allows `symex` `LLVM-IR` analysis to be used with any language that uses `LLVM` for code generation (which includes, Rust, C, C++, etc.). However, we need access to the `LLVM-IR`, so we cannot analyze inline assembly and code linked to our application (unless it is linked in as `LLVM-IR`, so libraries compiled with `gcc` is a no go).

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

- S: stack usage for the final path

Your answer for complexity_sum_recursive:

[P, I, C, S]

Your answer for complexity_sum_iterative:

[P, I, C, S]

Your answer for complexity_sum_formula:

[P, I, C, S]

In your own words, compare the P, I, C, S results obtained, and why did the complexity_sum_formula just produce one path?

[Your answer here]

Which one would you consider the best regarding performance?

[Your answer here]

---

# Learning objectives:

You should now have gained basic insights into the use of symbolic execution to:

- Test for functional equivalence. Can we replace one implementation by another?
- Symbolic execution to assess performance differences. Can we distinguish good implementations from bad ones regarding performance?
- Model based (safe) worst case cycle estimations.
- Worst case stack usage (also for recursive code).

In prior research at LTU, we developed an automated framework that generated concrete input assignments triggering each found path, which was fed to an automated test-bed that replayed each path on the actual target and measured the corresponding execution time. However, this tool was based on `cargo klee`, which is adopting _dynamic_ symbolic execution and thus may be inexact. Moreover, `klee` operates on LLVM-IR, thus even without approximation, the actual paths running on the target might differ (due to backend code generation and optimizations). Adopting `symex --elf` addresses these shortcomings, and You can take part in developing a further improved worst case execution time (WCET) framework, challenging the best WCET tools!
