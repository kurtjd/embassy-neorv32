#![no_std]
pub mod clint;
pub mod dma;
pub mod gpio;
pub mod gptmr;
mod interrupts;
pub mod sysinfo;
#[cfg(feature = "time-driver")]
mod time_driver;
pub mod trng;
pub mod uart;
pub mod wdt;

// Peripherals and interrupts supported by the NEORV32 chip
mod chip {
    pub use neorv32_pac as pac;
    // TODO: List all 32 ports
    embassy_hal_internal::peripherals!(CLINT, WDT, UART0, UART1, GPTMR, TRNG, DMA, GPIO, PORT0);
    pub mod interrupts {
        crate::interrupt_mod!(TRNG, DMA, GPTMR, GPIO);
    }
}

pub use chip::pac;
pub use chip::{Peripherals, interrupts::*, peripherals};

// TODO: Get main clock freq either statically via config or runtime via SysInfo
// This will likely need to only be runtime determined since SysInfo allows dynamic changing of freq
// That could make time-driver problematic which needs static freq... need to look into
const CPU_CLK_FREQ: u32 = 100_000_000;

pub fn init() -> Peripherals {
    // Attempt to take first so we panic before doing anything else
    let p = Peripherals::take();

    // TODO: Want to ensure this is called before using asyncs, but not force it if not
    // SAFETY: This can only be called once and before any CS
    unsafe { riscv::interrupt::enable() };

    p
}
