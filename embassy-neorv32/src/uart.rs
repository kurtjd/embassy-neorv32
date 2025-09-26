//! UART
use crate::peripherals::{UART0, UART1};
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};

/// UART driver capable of Rx and Tx
pub struct Uart<'d, M: IoMode> {
    reg: &'static pac::uart0::RegisterBlock,
    _phantom: PhantomData<(&'d (), M)>,
}

// TODO: Consider soundness of this
unsafe impl<'d, M: IoMode> Send for Uart<'d, M> {}

impl<'d> Uart<'d, Blocking> {
    /// Creates a new blocking UART driver with given baud rate
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> Self {
        let uart = Self {
            reg: T::reg(),
            _phantom: PhantomData,
        };

        uart.reset();

        if sim {
            uart.reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_sim_mode().set_bit());
        }

        if flow_control {
            uart.reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_hwfc_en().set_bit());
        }

        let mut baud_div = crate::CPU_CLK_FREQ / (2 * baud_rate);
        let mut prsc_sel = 0;

        // Calculate clock prescaler and baud rate prescaler
        // See: https://github.com/stnolting/neorv32/blob/main/sw/lib/source/neorv32_uart.c#L47
        while baud_div >= 0x3ff {
            if prsc_sel == 2 || prsc_sel == 4 {
                baud_div >>= 3;
            } else {
                baud_div >>= 1;
            }
            prsc_sel += 1;
        }

        uart.reg.ctrl().modify(|_, w| unsafe {
            w.uart_ctrl_prsc()
                .bits(prsc_sel & 0b111)
                .uart_ctrl_baud()
                .bits((baud_div as u16 - 1) & 0x3ff)
        });

        uart.enable();
        uart
    }

    /// Enable or disable RX FIFO not empty interrupt
    #[inline(always)]
    pub fn en_irq_rx_nempty(&self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_nempty().bit(enabled));
    }

    /// Enable or disable RX FIFO full interrupt
    #[inline(always)]
    pub fn en_irq_rx_full(&self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_full().bit(enabled));
    }

    /// Enable or disable TX FIFO empty interrupt
    #[inline(always)]
    pub fn en_irq_tx_empty(&self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_empty().bit(enabled));
    }

    /// Enable or disable TX FIFO not full interrupt
    #[inline(always)]
    pub fn en_irq_tx_nfull(&self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_nfull().bit(enabled));
    }
}

impl<'d> Uart<'d, Async> {
    /// Creates a new async UART driver with given baud rate
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true
    pub fn new_async<T: Instance>(
        _instance: T,
        _baud_rate: u32,
        _sim: bool,
        _flow_control: bool,
    ) -> Self {
        todo!()
    }
}

impl<'d, M: IoMode> Uart<'d, M> {
    // TODO: Temporary just for easy debugging
    pub fn debug_print(str: &'static str) {
        let uart = unsafe { crate::Peripherals::steal() }.UART0;
        let uart = Uart::new_blocking(uart, 50_000_000, true, false);
        uart.blocking_write(str.as_bytes());
    }

    #[inline(always)]
    fn enable(&self) {
        self.reg.ctrl().modify(|_, w| w.uart_ctrl_en().set_bit());
    }

    #[inline(always)]
    fn disable(&self) {
        self.reg.ctrl().modify(|_, w| w.uart_ctrl_en().clear_bit());
    }

    #[inline(always)]
    fn reset(&self) {
        self.reg.ctrl().reset();
    }

    /// Reads a byte from RX FIFO, blocking if empty
    pub fn blocking_read_byte(&self) -> u8 {
        while self.rx_fifo_empty() {}
        self.reg.data().read().bits() as u8
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty
    pub fn blocking_read(&self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.blocking_read_byte();
        }
    }

    /// Writes a byte to TX FIFO, blocking if full
    pub fn blocking_write_byte(&self, byte: u8) {
        while self.tx_fifo_full() {}
        self.reg.data().write(|w| unsafe { w.bits(byte as u32) });
    }

    /// Writes bytes to TX FIFO, blocking if full
    pub fn blocking_write(&self, bytes: &[u8]) {
        for byte in bytes {
            self.blocking_write_byte(*byte);
        }
    }

    /// Returns true if RX FIFO is empty
    #[inline(always)]
    pub fn rx_fifo_empty(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_rx_nempty().bit_is_clear()
    }

    /// Returns true if RX FIFO is full
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_rx_full().bit_is_set()
    }

    /// Returns true if RX FIFO has overflowed
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_rx_over().bit_is_set()
    }

    /// Clears RX FIFI overflow by disabling and re-enabling module
    pub fn rx_fifo_clear_overflow(&self) {
        // Overflow is cleared by disabling the module
        self.disable();
        self.enable();
    }

    /// Returns true if TX FIFO is empty
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_tx_empty().bit_is_set()
    }

    /// Returns true if TX FIFO is full
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_tx_nfull().bit_is_clear()
    }

    /// Returns depth of the RX FIFO
    #[inline(always)]
    pub fn rx_fifo_depth(&self) -> u32 {
        // TODO: Patch SVD to make this a field
        // Read value is log2, so do inverse log for actual value
        let raw = (self.reg.data().read().bits() >> 8) as u8 & 0b1111;
        1 << raw as u32
    }

    /// Returns depth of the TX FIFO
    #[inline(always)]
    pub fn tx_fifo_depth(&self) -> u32 {
        // TODO: Patch SVD to make this a field
        // Read value is log2, so do inverse log for actual value
        let raw = (self.reg.data().read().bits() >> 12) as u8 & 0b1111;
        1 << raw as u32
    }

    /// Blocks until all TX complete
    #[inline(always)]
    pub fn flush(&self) {
        while self.reg.ctrl().read().uart_ctrl_tx_busy().bit_is_set() {}
    }
}

// Convenience for writing formatted strings to UART
// TODO: Other Embassy HALs don't seem to do this so look at other approaches
// Don't like how it requires an &mut
impl<'d, M: IoMode> core::fmt::Write for Uart<'d, M> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.blocking_write(s.as_bytes());
        Ok(())
    }
}

/// UART IO mode
#[allow(private_bounds)]
pub trait IoMode: crate::Sealed {}

/// Blocking UART
pub struct Blocking;
impl crate::Sealed for Blocking {}
impl IoMode for Blocking {}

// TODO: Actually add async support
/// Async UART
pub struct Async;
impl crate::Sealed for Async {}
impl IoMode for Async {}

/// A valid UART peripheral
#[allow(private_bounds)]
pub trait Instance: crate::Sealed + PeripheralType {
    fn reg() -> &'static pac::uart0::RegisterBlock;
}

macro_rules! impl_instance {
    ($periph:ident, $rb:ident) => {
        impl crate::Sealed for $periph {}
        impl Instance for $periph {
            // Note: uart0 and uart1 can both share uart0::RegisterBlock
            // PAC is able to coerce uart1::ptr() to it with correct base address
            #[inline(always)]
            fn reg() -> &'static pac::uart0::RegisterBlock {
                unsafe { &*pac::$rb::ptr() }
            }
        }
    };
}

impl_instance!(UART0, Uart0);
impl_instance!(UART1, Uart1);
