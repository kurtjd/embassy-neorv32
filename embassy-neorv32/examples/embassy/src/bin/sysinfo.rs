#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::sysinfo::SysInfo;
use embassy_neorv32::uart::UartTx;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_blocking(p.UART0, 19200, false, false);

    // Print clock frequency
    writeln!(
        &mut uart,
        "Clock frequency: {} MHz",
        SysInfo::clock_freq() / 1_000_000
    )
    .unwrap();

    // Print memory size
    writeln!(&mut uart, "IMEM size: {} KiB", SysInfo::imem_size() / 1024).unwrap();
    writeln!(&mut uart, "DMEM size: {} KiB", SysInfo::dmem_size() / 1024).unwrap();

    // Print misc info
    writeln!(&mut uart, "Num harts: {}", SysInfo::num_harts()).unwrap();
    writeln!(&mut uart, "Boot mode: {}", SysInfo::boot_mode().as_str()).unwrap();
    writeln!(
        &mut uart,
        "Bus timeout cycles: {}",
        SysInfo::bus_timeout_cycles()
    )
    .unwrap();

    // Retrieve SoC Config
    let soc_config = SysInfo::soc_config();

    // Print processor features
    uart.blocking_write(b"\nProcessor Features:\n");
    if soc_config.imem() {
        uart.blocking_write(b"Internal IMEM\n");
    }
    if soc_config.dmem() {
        uart.blocking_write(b"Internal DMEM\n");
    }
    if soc_config.icache() {
        uart.blocking_write(b"Internal ICACHE\n");
    }
    if soc_config.dcache() {
        uart.blocking_write(b"Internal DCACHE\n");
    }
    if soc_config.imem_as_rom() {
        uart.blocking_write(b"Internal IMEM as pre-initialized ROM\n");
    }
    if soc_config.bootloader() {
        uart.blocking_write(b"Internal bootloader\n");
    }
    if soc_config.xbus() {
        uart.blocking_write(b"External bus interface (XBUS)\n");
    }
    if soc_config.ocd() {
        uart.blocking_write(b"On-chip debugger\n");
    }
    if soc_config.ocd_auth() {
        uart.blocking_write(b"On-chip debugger authentication\n");
    }

    // Print supported peripherals
    uart.blocking_write(b"\nPeripherals Supported:\n");
    if soc_config.uart0() {
        uart.blocking_write(b"UART0\n");
    }
    if soc_config.uart1() {
        uart.blocking_write(b"UART1\n");
    }
    if soc_config.twi() {
        uart.blocking_write(b"TWI\n");
    }
    if soc_config.twd() {
        uart.blocking_write(b"TWD\n");
    }
    if soc_config.spi() {
        uart.blocking_write(b"SPI\n");
    }
    if soc_config.sdi() {
        uart.blocking_write(b"SDI\n");
    }
    if soc_config.gptmr() {
        uart.blocking_write(b"GPTMR\n");
    }
    if soc_config.gpio() {
        uart.blocking_write(b"GPIO\n");
    }
    if soc_config.pwm() {
        uart.blocking_write(b"PWM\n");
    }
    if soc_config.wdt() {
        uart.blocking_write(b"WDT\n");
    }
    if soc_config.dma() {
        uart.blocking_write(b"DMA\n");
    }
    if soc_config.trng() {
        uart.blocking_write(b"TRNG\n");
    }
    if soc_config.onewire() {
        uart.blocking_write(b"ONEWIRE\n");
    }
    if soc_config.neoled() {
        uart.blocking_write(b"NEOLED\n");
    }
    if soc_config.tracer() {
        uart.blocking_write(b"TRACER\n");
    }
    if soc_config.slink() {
        uart.blocking_write(b"SLINK\n");
    }
    if soc_config.clint() {
        uart.blocking_write(b"CLINT\n");
    }
    if soc_config.cfs() {
        uart.blocking_write(b"CFS\n");
    }
}
