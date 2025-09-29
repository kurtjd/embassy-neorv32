#![no_std]
#![no_main]
use embassy_neorv32::bind_interrupts;
use embassy_neorv32::gpio::{self, Gpio};
use embassy_neorv32::peripherals;
use embassy_time::Timer;
use panic_halt as _;

bind_interrupts!(struct Irqs {
    GPIO => gpio::InterruptHandler<peripherals::GPIO>;
});

#[embassy_executor::task]
async fn input_task(mut input_pin: gpio::Input<'static, gpio::Async>) {
    loop {
        input_pin.wait_for_falling_edge().await;
        // Do something
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    let gpio = Gpio::new_async(p.GPIO, Irqs);
    let port = gpio.new_port(p.PORT0);
    let (input_pin, mut output_pin) = port.split();

    spawner.must_spawn(input_task(input_pin));
    loop {
        output_pin.toggle();
        Timer::after_micros(200).await;
    }
}
