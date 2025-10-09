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
    // TODO: List all 32 gpio ports
    embassy_hal_internal::peripherals!(
        CLINT, WDT, UART0, UART1, GPTMR, TRNG, DMA, GPIO, PORT0, PORT1, PORT2, PORT3, PORT4, PORT5,
        PORT6, PORT7, PORT8, PORT9, PORT10, PORT11, PORT12, PORT13, PORT14, PORT15, PORT16, PORT17,
        PORT18, PORT19, PORT20, PORT21, PORT22, PORT23, PORT24, PORT25, PORT26, PORT27, PORT28,
        PORT29, PORT30, PORT31,
    );
    pub mod interrupts {
        crate::interrupt_mod!(UART0, UART1, TRNG, DMA, GPIO);
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

/// The motivation for this macro is that due to neorv32 constraints, several peripherals need
/// to disable the peripheral interrupt in their IRQ handler for proper async behavior.
///
/// The driver then needs to re-enable the interrupt on wake. The HALs are designed to not require
/// an Instance generic as part of the struct for ergonomic purposes, so we need to explicitly list
/// the peripheral name.
///
/// Currently this is fine since the neorv32 only supports single instances of
/// the peripherals where this is used, but may need revisiting if that ever changes in the future.
macro_rules! enable_periph_irq {
    ($periph:ident) => {{ <$crate::peripherals::$periph as Instance>::Interrupt::enable() }};
}
pub(crate) use enable_periph_irq;
