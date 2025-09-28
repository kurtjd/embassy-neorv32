#![no_std]
#![no_main]

use embassy_neorv32::bind_interrupts;
use embassy_neorv32::dma::{self, Dma};
use embassy_neorv32::peripherals;
use embassy_neorv32::uart::{self, Uart};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::once_lock::OnceLock;
use embassy_time::Timer;
use panic_halt as _;

bind_interrupts!(struct Irqs {
    DMA => dma::InterruptHandler<peripherals::DMA>;
});

type UartMutex = Mutex<CriticalSectionRawMutex, Uart<'static, peripherals::UART0, uart::Blocking>>;
static UART: OnceLock<UartMutex> = OnceLock::new();

#[embassy_executor::task]
async fn dma_transfer(
    mut dma: Dma<'static, peripherals::DMA, dma::Async>,
    uart: &'static UartMutex,
) {
    loop {
        let src = [42; 2048];
        let mut dst = [69; 2048];

        let res = dma.transfer(&src, &mut dst).await;
        {
            let mut uart = uart.lock().await;
            if res.is_ok() && src[0] == dst[0] {
                uart.blocking_write(b"DMA transfer complete\n");
            } else {
                uart.blocking_write(b"DMA transfer failed\n");
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART just for printing WDT state
    let uart = Uart::new_blocking(p.UART0, 50_000_000, true, false);
    let uart = UART.get_or_init(|| Mutex::new(uart));

    // Setup DMA
    let dma = Dma::new_async(p.DMA, Irqs);
    spawner.must_spawn(dma_transfer(dma, uart));

    loop {
        uart.lock().await.blocking_write(b"Doing some work...\n");
        Timer::after_micros(10).await;
    }
}
