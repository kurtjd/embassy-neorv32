#![no_std]
#![no_main]
use embassy_neorv32::gpio::Gpio;
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = embassy_neorv32::init();

    let gpio = Gpio::new_blocking(p.GPIO);
    let mut port = gpio.new_port(p.PORT0);

    loop {
        port.toggle();
        embassy_neorv32::delay_us(200);
    }
}
