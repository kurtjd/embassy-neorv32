#![no_std]
#![no_main]
use embassy_neorv32::gptmr::{Gptmr, Prescaler};
use embassy_neorv32::uart::Uart;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let uart = Uart::new_blocking(p.UART0, 50_000_000, true, false);

    // Setup GPTMR
    let gptmr = Gptmr::new_blocking(p.GPTMR, Prescaler::Psc64);
    gptmr.set_threshold(1000);
    gptmr.enable();

    loop {
        uart.blocking_write(b"Waiting...\n");
        gptmr.blocking_wait();
    }
}
