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
        crate::interrupt_mod!(UART0, UART1, TRNG, DMA, GPTMR, GPIO);
    }
}

pub use chip::pac;
pub use chip::{Peripherals, interrupts::*, peripherals};

/// Initialize the HAL and enable interrupts globally.
pub fn init() -> Peripherals {
    // Attempt to take first so we panic before doing anything else
    let p = Peripherals::take();

    // TODO: Want to ensure this is called before using asyncs, but not force it if not
    // SAFETY: This can only be called once and before any CS
    unsafe { riscv::interrupt::enable() };

    p
}

/// Simple busy-loop delay.
///
/// Mainly just here as a placeholder currently for delaying in riscvrt examples.
pub fn delay_us(us: u64) {
    let start = riscv::register::mcycle::read64();
    let fclk = sysinfo::SysInfo::clock_freq() as u64;
    let cycles = us.saturating_mul(fclk) / 1_000_000;
    let end = start + cycles;
    while riscv::register::mcycle::read64() < end {}
}
