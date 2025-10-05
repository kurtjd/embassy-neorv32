#![no_std]
#![no_main]
use embassy_neorv32::bind_interrupts;
use embassy_neorv32::peripherals;
use embassy_neorv32::uart::{self, UartTx};
use panic_halt as _;

bind_interrupts!(struct Irqs {
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

// Ported to Rust from:
// https://github.com/stnolting/neorv32/blob/main/sw/lib/source/neorv32_aux.c#L605
async fn print_logo(uart: &mut UartTx<'static, uart::Async>) {
    const LOGO: [[u16; 7]; 9] = [
        [0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0300, 0xc630],
        [0x60c7, 0xfc7f, 0x87f8, 0xc0c7, 0xf87f, 0x8303, 0xfffc],
        [0xf0cc, 0x00c0, 0xcc0c, 0xc0cc, 0x0cc0, 0xc30f, 0x000f],
        [0xd8cc, 0x00c0, 0xcc0c, 0xc0c0, 0x0c01, 0x8303, 0x1f8c],
        [0xcccf, 0xf8c0, 0xcff8, 0xc0c0, 0xf806, 0x030f, 0x1f8f],
        [0xc6cc, 0x00c0, 0xcc18, 0x6180, 0x0c18, 0x0303, 0x1f8c],
        [0xc3cc, 0x00c0, 0xcc0c, 0x330c, 0x0c60, 0x030f, 0x000f],
        [0xc187, 0xfc7f, 0x8c06, 0x0c07, 0xf8ff, 0xc303, 0xfffc],
        [0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0300, 0xc630],
    ];

    for row in LOGO {
        uart.write_byte(b'\n').await;
        for val in row {
            let mut tmp = val;
            for _ in 0..16 {
                let c = if (tmp as i16) < 0 { b'#' } else { b' ' };
                uart.write_byte(c).await;
                tmp <<= 1;
            }
        }
    }
    uart.write_byte(b'\n').await;
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART in simulation mode with no HW flow control and arbitrary baud rate (since sim is immediately prints)
    // Note: In simulation async UART is noticeably slower than blocking since there is a bit of overhead,
    // but writes are instantaneous so we don't get any benefits for that overhead.
    let mut uart = UartTx::new_async(p.UART0, 19200, true, false, Irqs);
    print_logo(&mut uart).await;

    // Note: '\n' seems necessary for UART writes for sim to flush output
    uart.write(b"Hello world! :)\n").await;
}
