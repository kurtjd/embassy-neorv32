#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::trng::Trng;
use embassy_neorv32::uart::UartTx;
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_blocking(p.UART0, 19200, true, false);

    // Setup TRNG
    let trng = Trng::new_blocking(p.TRNG);
    if trng.sim_mode() {
        uart.blocking_write(b"Running in simulation so PRNG is used\n");
    }

    loop {
        let rand = trng.blocking_read_byte();
        writeln!(&mut uart, "Random byte: 0x{rand:02X}").unwrap();
        embassy_neorv32::delay_us(200);
    }
}
