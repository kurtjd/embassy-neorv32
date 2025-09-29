#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_neorv32::uart::Uart;
use embassy_neorv32::wdt::{ResetCause, Wdt};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let p = embassy_neorv32::init();

    // Setup UART just for printing WDT state
    let mut uart = Uart::new_blocking_tx(p.UART0, 19200, true, false);

    // Setup WDT with timeout of 1ms and enable it then lock it
    let wdt = Wdt::new(p.WDT);
    wdt.set_timeout_ms(1);
    wdt.enable();
    let wdt = wdt.lock();

    // Print the last reset cause
    let reset_cause = wdt.reset_cause();
    writeln!(
        &mut uart,
        "Last hardware reset cause: {}",
        reset_cause.as_str()
    )
    .unwrap();

    // On first reset, let's see if illegal access triggers a reset
    if matches!(reset_cause, ResetCause::External) {
        uart.blocking_write(b"Forcing HW reset...\n");
        wdt.force_hw_reset();
    }

    // On subsequent resets we feed a few times then wait for timeout reset to trigger
    for _ in 0..10 {
        uart.blocking_write(b"Feeding watchdog...\n");
        wdt.feed();
        embassy_neorv32::delay_us(200);
    }
    uart.blocking_write(b"Waiting for watchdog timeout...\n");

    loop {
        riscv::asm::wfi();
    }
}
