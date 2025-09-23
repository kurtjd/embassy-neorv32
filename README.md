# embassy-neorv32
An Embassy HAL for the RISCV-based [NEORV32](https://github.com/stnolting/neorv32) SoC/microcontroller

## Overview
This HAL is being designed to work in both a traditional synchronous context
(without the need for Embassy or any other async executor) as well as an asynchronous context with
Embassy support. Main focus now is on building the synchronous/blocking HAL before tackling the async HAL.

Still quite a bit to do to get all peripherals fully supported, however the `embassy-neorv32/examples`
folder can serve as a good base for running general Rust code on a simulated NEORV32.

This is currently being built to only support a single-core, rv32i, CLINT-enabled, cacheless configuration.
Planning to investigate support for a "max" configuration in the future.

## Peripherals Currently Supported
- SysInfo
- WDT
- Synchronous UART
- Synchronous TRNG
- Synchronous GPTMR

## TODO
- Iron out a few kinks in the SVD, svd2rust, and riscv-rt
- Make a few more simple sync HALs for other peripherals that can be easily tested in simulation to verify basic functionality
- Make one or two simple async HALs for some peripherals just to confirm proper interrupt response
- Fully flesh out sync HALs for peripherals that can be tested in simulation
- Fully flesh out async HALs for peripherals that can be tested in simulation
- Get real hardware to write/test peripheral HALs for I2C, SPI, etc
- Add feature gates for supported peripherals
- Allow easily configuration for clock frequency (for time-driver) and base ISA
- Investigate if changes in base ISA will impact correctness (currently targeting rv32i, no extensions)
- Investigate support for configs without CLINT
- Investigate ways to support CFS (Custom Functions Subsystem)
- Investigate dual-core support
- Investigate impact of I-CACHE/D-CACHE on correctness

## Run
- Clone [neorv32](https://github.com/stnolting/neorv32)
- Update `embassy-neorv32/examples/run-sim` to your `neorv32` path
- Install [GHDL](https://github.com/ghdl/ghdl) simulator
- Install [llvm-objcopy](https://llvm.org/docs/CommandGuide/llvm-objcopy.html)
- Run `cd embassy-neorv32/examples`
- Run `cargo run --release --bin hello-world`

## References
- [NEORV32 Datasheet](https://stnolting.github.io/neorv32/)