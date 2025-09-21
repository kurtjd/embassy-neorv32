#![no_std]
mod time_driver;
pub mod uart;
pub mod wdt;

pub use pac;

// TODO: Get main clock freq either statically via config or runtime via SysInfo
const CPU_CLK_FREQ: u32 = 100_000_000;

pub fn init() -> pac::Peripherals {
    let p = pac::Peripherals::take().expect("Perhipherals must not already be taken");

    // SAFETY: This can only be called once and before any CS
    unsafe {
        riscv::interrupt::enable();
    }

    p
}

// Pattern to prevent traits that need to be publicly callable but not implementable
// These traits should depend on `crate::Sealed`
trait Sealed {}
