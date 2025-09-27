#![no_std]
#![no_main]
use embassy_neorv32::bind_interrupts;
use embassy_neorv32::gptmr::{self, Gptmr, Prescaler};
use embassy_neorv32::peripherals;
use embassy_neorv32::uart::Uart;
use panic_halt as _;

bind_interrupts!(struct Irqs {
    GPTMR => gptmr::InterruptHandler<peripherals::GPTMR>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let uart = Uart::new_blocking(p.UART0, 50_000_000, true, false);

    // Setup GPTMR
    let gptmr = Gptmr::new_async(p.GPTMR, Prescaler::Psc64, Irqs);
    gptmr.set_threshold(1000);
    gptmr.enable();

    loop {
        uart.blocking_write(b"Waiting for timer to expire...\n");
        gptmr.wait().await;
    }
}
