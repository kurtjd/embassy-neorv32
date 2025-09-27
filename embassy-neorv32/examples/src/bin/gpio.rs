#![no_std]
#![no_main]
use embassy_neorv32::gpio::Gpio;
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    let gpio = Gpio::new_blocking(p.GPIO);
    let port = gpio.new_port(p.PORT0);

    loop {
        port.toggle();
        Timer::after_micros(200).await;
    }
}
