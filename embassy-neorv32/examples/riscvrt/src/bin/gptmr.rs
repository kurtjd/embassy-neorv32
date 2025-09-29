#![no_std]
#![no_main]
use embassy_neorv32::gptmr::{Gptmr, Prescaler};
use embassy_neorv32::uart::Uart;
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = Uart::new_blocking_tx(p.UART0, 19200, true, false);

    // Setup GPTMR
    let mut gptmr = Gptmr::new_blocking(p.GPTMR, Prescaler::Psc64);
    gptmr.set_threshold(1000);
    gptmr.enable();

    loop {
        uart.blocking_write(b"Waiting...\n");
        gptmr.blocking_wait();
    }
}
