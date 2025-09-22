#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::sysinfo::SysInfo;
use embassy_neorv32::uart::Uart;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = Uart::new_blocking(p.uart0, 50_000_000, true, false);

    // Setup SysInfo
    let sysinfo = SysInfo::new(p.sysinfo);

    // Print clock frequency
    writeln!(
        &mut uart,
        "Clock frequency: {} MHz",
        sysinfo.clock_freq() / 1_000_000
    )
    .unwrap();

    // Print memory size
    writeln!(&mut uart, "IMEM size: {} KiB", sysinfo.imem_size() / 1024).unwrap();
    writeln!(&mut uart, "DMEM size: {} KiB", sysinfo.dmem_size() / 1024).unwrap();

    // Print misc info
    writeln!(&mut uart, "Num harts: {}", sysinfo.num_harts()).unwrap();
    writeln!(&mut uart, "Boot mode: {}", sysinfo.boot_mode().as_str()).unwrap();
    writeln!(
        &mut uart,
        "Bus timeout cycles: {}",
        sysinfo.bus_timeout_cycles()
    )
    .unwrap();

    // Retrieve SoC Config
    let soc_config = sysinfo.soc_config();

    // Print processor features
    uart.puts_blocking("\nProcessor Features:\n");
    if soc_config.imem() {
        uart.puts_blocking("Internal IMEM\n");
    }
    if soc_config.dmem() {
        uart.puts_blocking("Internal DMEM\n");
    }
    if soc_config.icache() {
        uart.puts_blocking("Internal ICACHE\n");
    }
    if soc_config.dcache() {
        uart.puts_blocking("Internal DCACHE\n");
    }
    if soc_config.imem_as_rom() {
        uart.puts_blocking("Internal IMEM as pre-initialized ROM\n");
    }
    if soc_config.bootloader() {
        uart.puts_blocking("Internal bootloader\n");
    }
    if soc_config.xbus() {
        uart.puts_blocking("External bus interface (XBUS)\n");
    }
    if soc_config.ocd() {
        uart.puts_blocking("On-chip debugger\n");
    }
    if soc_config.ocd_auth() {
        uart.puts_blocking("On-chip debugger authentication\n");
    }

    // Print supported peripherals
    uart.puts_blocking("\nPeripherals Supported:\n");
    if soc_config.uart0() {
        uart.puts_blocking("UART0\n");
    }
    if soc_config.uart1() {
        uart.puts_blocking("UART1\n");
    }
    if soc_config.twi() {
        uart.puts_blocking("TWI\n");
    }
    if soc_config.twd() {
        uart.puts_blocking("TWD\n");
    }
    if soc_config.spi() {
        uart.puts_blocking("SPI\n");
    }
    if soc_config.sdi() {
        uart.puts_blocking("SDI\n");
    }
    if soc_config.gptmr() {
        uart.puts_blocking("GPTMR\n");
    }
    if soc_config.gpio() {
        uart.puts_blocking("GPIO\n");
    }
    if soc_config.pwm() {
        uart.puts_blocking("PWM\n");
    }
    if soc_config.wdt() {
        uart.puts_blocking("WDT\n");
    }
    if soc_config.dma() {
        uart.puts_blocking("DMA\n");
    }
    if soc_config.trng() {
        uart.puts_blocking("TRNG\n");
    }
    if soc_config.onewire() {
        uart.puts_blocking("ONEWIRE\n");
    }
    if soc_config.neoled() {
        uart.puts_blocking("NEOLED\n");
    }
    if soc_config.tracer() {
        uart.puts_blocking("TRACER\n");
    }
    if soc_config.slink() {
        uart.puts_blocking("SLINK\n");
    }
    if soc_config.clint() {
        uart.puts_blocking("CLINT\n");
    }
    if soc_config.cfs() {
        uart.puts_blocking("CFS\n");
    }
}
