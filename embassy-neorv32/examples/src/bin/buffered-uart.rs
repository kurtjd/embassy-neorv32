#![no_std]
#![no_main]

#[cfg(feature = "sim")]
compile_error!("Buffered UART example not available in simulation.");

use core::fmt::Write;
use embassy_neorv32::uart::{self, buffered::BufferedUart};
use embassy_neorv32::{bind_interrupts, peripherals};
use embassy_neorv32_examples::*;

bind_interrupts!(struct Irqs {
    UART0 => uart::buffered::BufferedInterruptHandler<peripherals::UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup buffered UART
    let mut buffer = [0; 128];
    let mut uart = BufferedUart::new(p.UART0, &mut buffer, UART_BAUD, false, Irqs)
        .expect("UART must be supported");

    uart.write(b"Type 16 characters...\n").await.unwrap();
    let mut chars = [0; 16];
    uart.read(&mut chars).await;

    writeln!(&mut uart, "You typed: {}", str::from_utf8(&chars).unwrap()).unwrap();
}
