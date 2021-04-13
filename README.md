# Board support crate for Crazyflie
The [Crazyflie 2.1](https://www.bitcraze.io) is a capable little drone weighing
in at just 27 grams. It is fully open-source which makes it easy to play around
with and learn from.

This crate implements board support for the Crazyflie so that one can program
the main `MCU` in Rust. See the [`./examples` folder](./examples/) for
inspiration.

## Uploading to Crazyflie
First we need some prerequisites, install `dfu-utils` through your package
manager and through `Cargo` install `cargo install cargo-binutils` and add the
necessary components with `rustup component add llvm-tools-preview`.

Next we will convert the example into a `.bin` file so that we can load that
onto the Crazyflie. To do this use
[`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils) as follows

```bash
cargo objcopy --example blinky --release -- -O binary target/thumbv7em-none-eabihf/release/blinky.bin
```

To automatically set the Crazyflie to bootloader mode we can add `-w
radio://0/80/2M` to the `cfloader` command to do it for us. The command should
look like:

```bash
python3 -m cfloader -w radio://0/80/2M flash target/thumbv7em-none-eabihf/release/blinky.bin stm32-fw
```
(`cfloader` can be installed through `pip3 install --user cflib`)

This will set the Crazyflie into bootloader mode, upload the binary and restart
the `STM32F405` for us.

You should now see all lights blinking on the Crazyflie!

### Manual
Start the Crazyflie in bootloader mode by turning it off and then holding the
on/off button in for about 3 seconds. Both blue LEDs on the tail should blink in
an alternating pattern.

```bash
python3 -m cfloader flash target/thumbv7em-none-eabihf/release/blinky.bin stm32-fw
```

### Resetting back
To get back to the proper firmware, use `cfclient`, select bootloader. Then
select `Cold boot (recovery)` and restart the Crazyflie in bootloader mode
(alternating blue blinking). From there you should be able to flash the regular
firmware.
