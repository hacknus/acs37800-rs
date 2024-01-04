# ACS37800 Rust Driver

[![crates.io](https://img.shields.io/crates/v/acs37800.svg)](https://crates.io/crates/acs37800)
[![Docs](https://docs.rs/acs37800/badge.svg)](https://docs.rs/acs37800)
[![Rust](https://github.com/hacknus/acs37800-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/acs37800-rs/actions/workflows/rust.yml)

This is a platform-agnostic rust driver for the [ACS37800](https://www.allegromicro.com/en/products/sense/current-sensor-ics/zero-to-fifty-amp-integrated-conductor-sensor-ics/acs37800) current sensor.  
Fully supported in `#![no_std]` environments.

## Example
To implement this driver, consult the example:  
Put this into your `cargo.toml`:
```toml
[dependencies]
tmc5160 = { git = "https://github.com/hacknus/acs37800-rs" }
# required for the register configs to_u32_le() function
modular-bitfield-to-value = {git = "https://github.com/hacknus/modular-bitfield-to-value"}
```
Add the following imports:
```rust
use acs37800::registers::*;

// required for the to_u32_le() function.
use modular_bitfield_to_value::ToValue;
```

Configure the I2C bus in the `main()` function as follows:
```rust
let scl = gpiob.pb6;
let sda = gpiob.pb7;
let i2c1 = dp.I2C1.i2c(
    (scl, sda),
    i2cMode::Standard {
    frequency: 100.kHz(),
    },
    &clocks,
);
```
and to use the driver, implement the driver as shown below:
```rust
{

    // set up current sensor
    let mut current_sensor = Acs37800::new(i2c)
        .with_r_iso(1_000_000)
        .with_r_sense(16_900);

}
```