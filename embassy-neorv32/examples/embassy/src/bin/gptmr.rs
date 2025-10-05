#![no_std]
#![no_main]
use embassy_neorv32::bind_interrupts;
use embassy_neorv32::gptmr::{self, Gptmr, Prescaler};
use embassy_neorv32::peripherals;
use embassy_neorv32::uart::{self, UartTx};
use panic_halt as _;

bind_interrupts!(struct Irqs {
    GPTMR => gptmr::InterruptHandler<peripherals::GPTMR>;
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_async(p.UART0, 19200, true, false, Irqs);

    // Setup GPTMR
    let mut gptmr = Gptmr::new_async(p.GPTMR, Prescaler::Psc64, Irqs);
    gptmr.set_threshold(1000);
    gptmr.enable();

    loop {
        uart.write(b"Waiting for timer to expire...\n").await;
        gptmr.wait().await;
    }
}
