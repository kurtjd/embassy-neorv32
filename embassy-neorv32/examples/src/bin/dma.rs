#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_neorv32::dma::Dma;
use embassy_neorv32::uart::Uart;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART just for printing WDT state
    let mut uart = Uart::new_blocking(p.UART0, 50_000_000, true, false);

    // Setup DMA
    let dma = Dma::new_blocking(p.DMA);

    let src = 0xBAADF00Du32.to_le_bytes();
    let mut dst = 0xDEADBEEFu32.to_le_bytes();

    writeln!(&mut uart, "Src: 0x{:08X}", u32::from_le_bytes(src)).unwrap();
    writeln!(&mut uart, "Dst: 0x{:08X}", u32::from_le_bytes(dst)).unwrap();

    uart.blocking_write(b"Starting transfer...\n");
    if dma.blocking_transfer(&src, &mut dst).is_ok() {
        writeln!(&mut uart, "Src: 0x{:08X}", u32::from_le_bytes(src)).unwrap();
        writeln!(&mut uart, "Dst: 0x{:08X}", u32::from_le_bytes(dst)).unwrap();
    } else {
        uart.blocking_write(b"DMA transfer failed!\n");
    }
}
