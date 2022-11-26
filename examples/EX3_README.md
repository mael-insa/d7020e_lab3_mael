# Ex3

In this example we will see how we can model hardware, and verify that the user program accesses the hardware correctly.

Let us assume that we want to model a very simple device, with the following specification:

- State

  - The device holds an internal buffer of 8 bytes (`buffer`):

  - The device holds an internal byte sized register (`read_pos`) pointing to the next byte to read from the internal buffer.
  
- Functionality

  - The device has a byte sized register `received` which contains the number of bytes received. Reading the `received` register resets the `read_pos` register. 

- The device has a byte sized register `data` which returns the value from the internal buffer at the `read_pos` index. Reading the `data` register increments the `read_pos` register.

We model the internal state of the device by:

```rust
struct Device {
    buffer: [u8; 8],
    read_pos: u8,
}
```

And we make a first attempt to model the functionality by:

```rust
// Model of the Device functionality
impl Device {
    fn received(&mut self) -> u8 {
        self.read_pos = 0; // we reset the read_pos

        // we return an unknown number of bytes received
        u8::any()
    }

    fn data(&mut self) -> u8 {
        let r = self.buffer[self.read_pos as usize];
        self.read_pos += 1;
        r
    }
}
```

Looking at the model we see:

- `received` resets the `read_pos` and returns an unknown value (modeling that the number of bytes to be read is unknown).

- `data` returns the buffered value and increments the `read_pos`.


Let's see how this pans out, in `device_test`.

```rust
pub fn device_test() {
    let mut device = Device {
        buffer: [0; 8],
        read_pos: 0,
    };

    let received = device.received();
    for i in 0..received {
        let _data = device.data();
    }
}
```

First we construct our fake device and read the `received` register into `n`.

Then read the `data` register `n` times (but we don't do anything with the read data).

- Ex3 A1)
  
  Run the test.

  ```shell
  cargo symex --example ex3 --function device_test --release
  ```

  [Paste your output here]

  You should have encountered an `Error` (if not go back and check, if still no error let me know on Discord).

- Ex3 A2)

  So why did the test fail? 
  
  Well, the test itself seems simple enough (nothing that can go wrong here really.) 
  
  However, on closer inspection, we find an error in our model of the functionality. Implicit to the specification, the number of bytes received must be less than 8 (since we have just an 8 byte buffer right).

  Let's fix this problem by the uncommenting the `assume(n <= 8)` statement.

  Now let's try again:

  ```shell
  cargo symex --example ex3 --function device_test --release
  ```

  [Paste your output here]

  Now the test should pass!!! (As usual, if your result differ check again, and let me know on Discord if there is a problem.)

- Ex3 A3)

  Now, let's have a look of a user program that computes the sum of the data received `device_test_sum`. In the `Cargo.toml` we have enabled overflow checking, to ensure that the summation does not overflow.

  ```toml
  [profile.release]
  overflow-checks = true
  ```

  Now, let's see what happens:

  ```shell
  cargo symex --example ex3 --function device_test --release
  ```

  [Your output here]

  In this case you should now have got 9 paths (as usual, if not double check and report problems.)

  So why did we get 9 different paths, well there is now a potential error overflow error turning up that the Rust compiler cannot statically prove false (never to happen).

  The paths obtained relate to the number for bytes received, 0 bytes, 1 byte, 2 bytes, up to 8. (9 in total).

- Ex3 A4)

  Now let's return to our model of the device. On `reset` the data buffer is initialized to 0, and it stays 0 unless we tell it otherwise. Implicit in the specification the values held in the buffer will of course change (else there is not much use of this device, right). So let us model that!

  Uncomment the code:

  ```Rust
  assume(n <= 8);
  for v in self.buffer[0..n as usize].iter_mut() {
    *v = u8::any()
  }
  ```

  We have received an unknown number of bytes `n` under the assumption that `n <= 8`.

  We now iterate over the first `n` elements of the buffer and assign each element an unknown value.

  Let's see how this goes down.

  ```
  cargo symex --example ex3 --function device_test_sum --release
  ```

  How many paths did you obtain?

  [Your answer here]

  At this point you should have obtained a significant number of paths, many of them failing with an `Error`. (If not double check and report.)

  Now re-run the test but pipe it to a file:

  ```shell
  cargo symex --example ex3 --function device_test_sum --release > ex3_a4.txt
  ```

  Now, take a close look at the generated file and identify the first path that led to an error.

  [Past your first failing path here]

  Your result should have concrete assignments to 3 Symbolic values:
  (As usual, double check/report.) 

  Explain in your own words what these 3 Symbolic values represent.

  [Your answer here]
  
  Hint 1: look at the device model code. Where do we introduce symbolic (`any()`) values and in what order. You should be able to see the structure here. Also look at the previous (succeeding) paths, for further context.

  Hint 2: what could possibly go wrong in the test? The critical operation here is the `sum += device.data()`.

- Ex4 A5)

  With this analysis at hand. We should now be able to fix the problem in the test. (The model is not wrong here, right.)

  Apply the patch needed, and confirm that you get 9 succeeding paths.

  Check in both this file and the updated `ex4` file.

---

## Learning outcomes

Although the device has a trivial specification, the example show that we can both create a model of a specification and verify that an application (in this case `device_test`) is correctly using the hardware model.

Now, have a look at the errata for the 52840 v3 (a 40 page document), and you will find numerous examples where design errors have slipped through testing. Notice, this is the 3rd revision in production and the 7th iteration of the chip including Engineering samples. Still it takes 40 pages to list the remaining bugs and suggest mitigations. 

The approach we have taken here focus the functional properties modelled as state and state transitions, and as such does not cover extra functional properties, such as electrical properties and timely behavior. However, at glance, a majority of the bugs listed seems to relate functional behavior. Have a look yourself:

```
RTC: Register values invalid
CLOCK: Some registers are not reset when expected
I2S, New and inherited anomalies
I2S: RXPTRUPD and TXPTRUPD events asserted after STOP
...
```

With that said, the hardware is the easy part. Tools for chip design already include formal verification (model-checking), physics based electrical simulation, rigorous methods to testing etc. etc.

When it comes to firmware design.... typically nothing besides happy hacking until something eventually does not crash.

Here is where we should focus our effort.

This simple example showcase a path to the holy grail! Hardware software co-design, where we prove compliance of the firmware to the hardware specification. Ideally, both the hardware model (that we derived manually) and the implementation (that Nordic engineered) should both be automatically generated from a singe specification. That is the ultimate goal, where correctness by construction can become a reality.
