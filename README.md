# teensy-rs

A minimal Rust project for the [PJRC Teensy 4.1](https://www.pjrc.com/store/teensy41.html) microcontroller. Demonstrates a simple LED blink using the [`teensy4-bsp`](https://crates.io/crates/teensy4-bsp) board support package.

## Prerequisites

1. Install the ARM Cortex-M7 target:

   ```sh
   rustup target add thumbv7em-none-eabihf
   ```

2. Install [`tycmd`](https://github.com/Koromix/tytools) and ensure it's on your `PATH`.

## Usage

**Build:**

```sh
cargo build --release
```

**Flash:**

```sh
cargo run --release
```

## Troubleshooting

If flashing stalls or the board isn't enumerating USB, press the Teensy's program button to enter bootloader mode. The upload command waits for the bootloader, so you may need to press the button each time you flash.

## Resources

- [teensy4-rs](https://github.com/mciantyre/teensy4-rs) - Rust support crates for Teensy 4
- [Teensy 4.1 Documentation](https://www.pjrc.com/store/teensy41.html)
