#![no_std]
#![no_main]
use embassy_neorv32::uart::Uart;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::once_lock::OnceLock;

// Note: In simulation a microsecond is a lot longer :P
use embassy_time::Timer;

static UART: OnceLock<Mutex<CriticalSectionRawMutex, Uart>> = OnceLock::new();

#[embassy_executor::task]
async fn hello(uart: &'static Mutex<CriticalSectionRawMutex, Uart>) {
    loop {
        uart.lock().await.write("Hello world\n".as_bytes());
        Timer::after_micros(1).await;
    }
}

#[embassy_executor::task]
async fn risky_ferris(uart: &'static Mutex<CriticalSectionRawMutex, Uart>) {
    loop {
        uart.lock().await.write("Rust on neorv32!\n".as_bytes());
        uart.lock()
            .await
            .write("RISC architecture is gonna change everything\n".as_bytes());
        Timer::after_micros(200).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Note: '\n' seems necessary for UART writes for sim to flush output
    let uart = Uart::new(p.uart0);
    let uart = UART.get_or_init(|| Mutex::new(uart));

    spawner.must_spawn(hello(uart));

    // 🦀
    spawner.must_spawn(risky_ferris(uart));
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        riscv::asm::wfi();
    }
}
