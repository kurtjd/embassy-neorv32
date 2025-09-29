#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::bind_interrupts;
use embassy_neorv32::peripherals;
use embassy_neorv32::trng::{self, Trng};
use embassy_neorv32::uart::{self, Uart};
use embassy_time::Timer;
use panic_halt as _;

bind_interrupts!(struct Irqs {
    TRNG => trng::InterruptHandler<peripherals::TRNG>;
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = Uart::new_async_tx(p.UART0, 19200, true, false, Irqs);

    // Setup async TRNG
    let mut trng = Trng::new_async(p.TRNG, Irqs);
    if trng.sim_mode() {
        uart.write(b"Running in simulation so PRNG is used\n").await;
    }
    let fifo_depth = trng.fifo_depth();
    writeln!(&mut uart, "TRNG FIFO depth: {fifo_depth}").unwrap();

    loop {
        let mut buf = [0; 8];
        trng.read(&mut buf).await;

        // BE vs LE arbitrary here
        let word1 = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        let word2 = u32::from_be_bytes(buf[4..8].try_into().unwrap());

        writeln!(&mut uart, "Random word1: 0x{word1:08X}").unwrap();
        writeln!(&mut uart, "Random word2: 0x{word2:08X}").unwrap();
        Timer::after_micros(200).await;
    }
}
