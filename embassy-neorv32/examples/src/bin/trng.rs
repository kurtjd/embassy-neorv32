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
    let mut uart = Uart::new_blocking(p.uart0, 50_000_000, true, false);

    // Setup TRNG
    let trng = Trng::new_blocking(p.trng);
    if trng.sim_mode() {
        uart.blocking_write(b"Running in simulation so PRNG is used\n");
    }

    loop {
        let rand = trng.blocking_read_byte();
        writeln!(&mut uart, "Random byte: 0x{rand:02X}").unwrap();
        Timer::after_micros(200).await;
    }
}
