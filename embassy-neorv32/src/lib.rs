#![no_std]
mod time_driver;
pub mod uart;
use neorv32_pac::Peripherals;

pub fn init() -> Peripherals {
    let p = Peripherals::take().expect("Perhipherals must not already be taken");

    // SAFETY: This can only be called once and before any CS
    unsafe {
        riscv::interrupt::enable();
    }

    p
}
