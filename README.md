# embassy-neorv32
An Embassy HAL for the RISCV-based [NEORV32](https://github.com/stnolting/neorv32) SoC/microcontroller

## Overview
Just a very basic UART and time-driver for now as a PoC

**TODO**: Everything

## Run
- Clone [neorv32](https://github.com/stnolting/neorv32)
- Update `embassy-neorv32/examples/run-sim` to your `neorv32` path
- Install [GHDL](https://github.com/ghdl/ghdl) simulator
- Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils/)
- Run `cargo run --release --bin hello-world`