#![no_std]
mod time_driver;
pub mod uart;
use neorv32_pac::{Peripherals, interrupt::CoreInterrupt};

pub fn init() -> Peripherals {
    let p = Peripherals::take().expect("Perhipherals must not already be taken");

    // SAFETY: This can only be called once and before any CS
    unsafe {
        riscv::interrupt::enable();
    }

    p
}

// TODO: riscv-rt with `no-interrupts` is needed for custom core interrupts,
// but that seems to remove the provided aliasing to `DefaultHandler`.
//
// These are here now just to get the thing to build until they are used elsewhere in the HAL
#[riscv_rt::core_interrupt(CoreInterrupt::MachineSoft)]
fn machine_soft_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::MachineExternal)]
fn machine_external_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::TWD)]
fn twd_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::CFS)]
fn cfs_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::UART0)]
fn uart0_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::UART1)]
fn uart1_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::TRACER)]
fn tracer_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::SPI)]
fn spi_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::TWI)]
fn twi_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::GPIO)]
fn gpio_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::NEOLED)]
fn neoled_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::DMA)]
fn dma_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::SDI)]
fn sdi_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::GPTMR)]
fn gptmr_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::ONEWIRE)]
fn onewire_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::SLINK)]
fn slink_handler() -> ! {
    default_handler()
}

#[riscv_rt::core_interrupt(CoreInterrupt::TRNG)]
fn trng_handler() -> ! {
    default_handler()
}

fn default_handler() -> ! {
    loop {
        riscv::asm::wfi();
    }
}
