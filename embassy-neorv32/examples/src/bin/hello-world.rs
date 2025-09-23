#![no_std]
#![no_main]
use embassy_neorv32::uart::Uart;
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART in simulation mode with no HW flow control and a very high baud rate (since sim is slow)
    let uart = Uart::new_blocking(p.uart0, 50_000_000, true, false);

    loop {
        // Note: '\n' seems necessary for UART writes for sim to flush output
        uart.blocking_write(b"Hello world\n");
        Timer::after_millis(1).await;
    }
}
