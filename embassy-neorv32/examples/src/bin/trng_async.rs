#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::trng::Trng;
use embassy_neorv32::uart::Uart;
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = Uart::new_blocking(p.UART0, 50_000_000, true, false);

    // Setup async TRNG
    let trng = Trng::new_async(p.TRNG);
    if trng.sim_mode() {
        uart.blocking_write(b"Running in simulation so PRNG is used\n");
    }

    loop {
        let mut buf = [0; 4];
        trng.read(&mut buf).await;
        let word = u32::from_be_bytes(buf); // BE vs LE arbitrary here
        writeln!(&mut uart, "Random word: 0x{word:08X}").unwrap();
        Timer::after_micros(200).await;
    }
}
