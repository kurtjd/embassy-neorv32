// TODO: Come up with strategy to bind interrupt similar to Embassy's internal cortex-m interrupt hal
// Because we still want non-async users to be able to implement their own ISRs

embassy_hal_internal::peripherals!(WDT, UART0, UART1, GPTMR, SYSINFO, TRNG, DMA);
