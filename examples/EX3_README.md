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
  cargo symex --elf --example ex3 --function device_test --release
  ```

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x00 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x00000191 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x2003ffe7 (32-bits)
      R1: 0x00000001 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 20
  Max number of cycles: 40
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 2 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Error: panic

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x0d (8-bits)

  End state:
      SP: 0x2003ffd0 (32-bits)
      R5: 0x0000000d (32-bits)
      PC: 0x0000021a (32-bits)
      LR: 0x00000175 (32-bits)
      R6: 0x00000008 (32-bits)
      R0: 0x00000008 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x2003ffd0 (32-bits)
      R4: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 154
  Max number of cycles: 247
  Stack usage: 48 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 3 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x08 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 142
  Max number of cycles: 228
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 4 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x07 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 127
  Max number of cycles: 204
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 5 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x06 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 112
  Max number of cycles: 180
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 6 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x05 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 97
  Max number of cycles: 156
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 7 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x04 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 82
  Max number of cycles: 132
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 8 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x03 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 67
  Max number of cycles: 108
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 9 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x02 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 52
  Max number of cycles: 84
  Stack usage: 44 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 10 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x01 (8-bits)

  End state:
      SP: 0x20040000 (32-bits)
      R5: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      LR: 0x000001a1 (32-bits)
      R6: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R7: 0x00000000 (32-bits)
      R4: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
  Instructions executed: 37
  Max number of cycles: 60
  Stack usage: 44 bytes

  You should have encountered an `Error` (if not go back and check, if still no error let me know on Discord).

- Ex3 A2)

  So why did the test fail?

  Well, the test itself seems simple enough (nothing that can go wrong here really.)

  However, on closer inspection, we find an error in our model of the functionality. Implicit to the specification, the number of bytes received must be less or equal to 8 (since we have just an 8 byte buffer right).

  Let's fix this problem by the uncommenting the `assume(n <= 8)` statement.

  Now let's try again:

  ```shell
  cargo symex --elf --example ex3 --function device_test --release
  ```

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x00 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x00000001 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001a1 (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000001 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 34
  Max number of cycles: 62
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 2 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x08 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 156
  Max number of cycles: 250
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 3 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x07 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 141
  Max number of cycles: 226
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 4 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x06 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 126
  Max number of cycles: 202
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 5 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x05 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 111
  Max number of cycles: 178
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 6 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x04 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 96
  Max number of cycles: 154
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 7 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x03 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 81
  Max number of cycles: 130
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 8 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x02 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 66
  Max number of cycles: 106
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 9 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R11: 0x00000000 (32-bits)
      any5: 0x01 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      R1: 0x2003ffd8 (32-bits)
      R2: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      PC: 0xfffffffe (32-bits)
      R4: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      LR: 0x000001ad (32-bits)
      R11: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
  Instructions executed: 51
  Max number of cycles: 82
  Stack usage: 56 bytes
  

  Now the test should pass!!! (As usual, if your result differ check again, and let me know on Discord if there is a problem.)

- Ex3 A3)

  Now, let's have a look of a user program that computes the sum of the data received `device_test_sum`. In the `Cargo.toml` we have enabled overflow checking, to ensure that the summation does not overflow.

  ```toml
  [profile.release]
  overflow-checks = true
  ```

  Now, let's see what happens:

  ```shell
  cargo symex --elf --example ex3 --function device_test_sum --release
  ```

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x00 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001e7 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000001 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 35
  Max number of cycles: 63
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 2 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x08 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 197
  Max number of cycles: 291
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 3 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x07 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 177
  Max number of cycles: 262
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 4 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x06 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 157
  Max number of cycles: 233
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 5 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x05 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 137
  Max number of cycles: 204
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 6 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x04 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 117
  Max number of cycles: 175
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 7 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x03 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 97
  Max number of cycles: 146
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 8 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x02 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 77
  Max number of cycles: 117
  Stack usage: 56 bytes

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 9 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Success: returned void

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x01 (8-bits)

  End state:
      R6: 0x00000000 (32-bits)
      LR: 0x000001f3 (32-bits)
      PC: 0xfffffffe (32-bits)
      R5: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      R0: 0x00000000 (32-bits)
      R2: 0x00000000 (32-bits)
      SP: 0x20040000 (32-bits)
      R4: 0x00000000 (32-bits)
      R1: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
  Instructions executed: 57
  Max number of cycles: 88
  Stack usage: 56 bytes


  In this case you should now have got 9 paths (as usual, if not double check and report problems.)

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

  *I obtain 37 paths for this execution and most of them raised an error*

  At this point you should have obtained a significant number of paths, many of them failing with an `Error`. (If not double check and report.)

  Now re-run the test but pipe it to a file:

  ```shell
  cargo symex --elf --example ex3 --function device_test_sum --release > ex3_fail.txt
  ```

  Now, take a close look at the generated file and identify the first path that led to an error.

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 3 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Error: panic

  Symbolic:
      R4: 0x00000000 (32-bits)
      R5: 0x00000000 (32-bits)
      R6: 0x00000000 (32-bits)
      R7: 0x00000000 (32-bits)
      R8: 0x00000000 (32-bits)
      any5: 0x02 (8-bits)
      any6: 0xc0 (8-bits)
      any7: 0x7e (8-bits)

  End state:
      R0: 0x0000003e (32-bits)
      R5: 0x0000013e (32-bits)
      R7: 0x2003fff8 (32-bits)
      R4: 0x00000001 (32-bits)
      SP: 0x2003ffd8 (32-bits)
      LR: 0x0000037f (32-bits)
      R1: 0x000000c0 (32-bits)
      R8: 0x00000002 (32-bits)
      PC: 0x000003b4 (32-bits)
      R2: 0x0000007e (32-bits)
      R6: 0x2003ffd8 (32-bits)
  Instructions executed: 99
  Max number of cycles: 147
  Stack usage: 56 bytes

  Under `Symbolic:` for each test you will find the assigned registers and a set of `any` marked symbolic values. The `any` values are introduced in the `Device` model.

  As seen, the number of generated `any` values differs between reported paths.

  Explain in your own words for any given path what the first `any` represent and how the remaining number of `any` values related to the first. Also, explain why some paths succeed while other's fail. 

  Hint 1: look at the device model code. Where do we introduce symbolic (`any()`) values and in what order. You should be able to see the structure here. Also look at the previous (succeeding) paths, for further context.

  Hint 2: what could possibly go wrong in the test? The critical operation here is the `sum += device.data()`.

  *The first any is always any5 and this any increases while we have more paths but it didn't increase at every path, it's because our first any gives the number of bytes we will received for this path. Then we have the corresponding number of any values representing the value of each of the bytes. For example, if we have any5 = 0x02, we will have any6 and any7 as random value for the two bytes.*

  *Some paths succeded and other fail due to an overflow of the variable sum. We can simply solve this problem by changing the type of the variable by u16.*

- Ex4 A5)

  With this analysis at hand, we should now be able to fix the problem in the test. (The model is not wrong here, the problem was in the test.)

  Apply the patch needed, and confirm that you get 9 succeeding paths.

  Check-in both this file and the updated `ex3` file.

---

## Learning outcomes

Although the device has a trivial specification, the example show that we can both create a model of a specification and verify that an application (in this case `device_test`) is correctly using the hardware model.

Now, have a look at the errata for the [52840 v3 Errata](https://docs.nordicsemi.com/bundle/errata_nRF52840_Rev3/page/ERR/nRF52840/Rev3/latest/err_840.html) (a 40 page document), and you will find numerous examples where design errors have slipped through testing. Notice, this is the 3rd revision in production and the 7th iteration of the chip including Engineering samples. Still it takes 40 pages to list the remaining bugs and suggest mitigations.

So, why are the chips produces still broken? (Nordic knows what's wrong, right...) There are numerous reasons, one is cost (taping out a new chip is costly), but more importantly, customers (product owners) are already chipping products based on the broken chips (and learned to live with them). Juggling firmware in a C/C++ based firmware workflow is already a scary proposition - adding further complexity by taking fine grained chip versioning (stepping) into account would make the situation even worse. Thus, for a customer it is typically "safer" to live with the bugs than to maintain different firmware functionality dependent on chip version. (If it ain't totally broken - don't fix it.) This is of course far from ideal as the performance suffers (not being able to utilize the device to its full potential, moreover, the mitigations (software workarounds), may impose further overhead). 

Notice here: The verification approach we have taken here focus the functional properties modelled as state and state transitions, and as such does not cover extra functional properties, such as electrical properties and timely behavior. However, at glance, a majority of the bugs listed in the errata relates functional behavior. Have a look yourself:

```
RTC: Register values invalid
CLOCK: Some registers are not reset when expected
I2S, New and inherited anomalies
I2S: RXPTRUPD and TXPTRUPD events asserted after STOP
...
```

With that said, the hardware is the easy part. Tools for chip design already include formal verification (model-checking), physics based electrical simulation, rigorous methods to testing etc. etc.

When it comes to firmware design.... typically nothing besides happy hacking until something eventually does not crash.

Later in the course, we will dig deeper into embedded Rust and the way we can leverage on the Rust language and ecosystem to build abstractions of the underlying hardware. In this way, a Rust based workflow facilitates firmware versioning and offers new opportunities to manage various chip steppings.  

Here is where we should focus our effort.

This simple example showcase a path to the holy grail! Hardware software co-design, where we prove compliance of the firmware to the hardware specification. Ideally, both the hardware model (that we derived manually) and the implementation (that Nordic engineered) should both be automatically generated from a singe specification. That is the ultimate goal, where correctness by construction can become a reality.
