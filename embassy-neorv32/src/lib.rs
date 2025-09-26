#![no_std]
mod chip;
pub mod dma;
pub mod gptmr;
pub mod sysinfo;
mod time_driver;
pub mod trng;
pub mod uart;
pub mod wdt;

pub use chip::{Peripherals, peripherals};

// TODO: Get main clock freq either statically via config or runtime via SysInfo
// This will likely need to only be runtime determined since SysInfo allows dynamic changing of freq
// That could make time-driver problematic which needs static freq... need to look into
const CPU_CLK_FREQ: u32 = 100_000_000;

pub fn init() -> Peripherals {
    let p = Peripherals::take();

    // SAFETY: This can only be called once and before any CS
    unsafe {
        riscv::interrupt::enable();
    }

    p
}

// Pattern to prevent traits that need to be publicly callable but not implementable
// These traits should depend on `crate::Sealed`
trait Sealed {}
