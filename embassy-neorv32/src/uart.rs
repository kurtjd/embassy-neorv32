//! UART
use core::marker::PhantomData;

/// UART driver capable of Rx and Tx
pub struct Uart<T: Instance, M: IoMode> {
    _instance: PhantomData<T>,
    _mode: PhantomData<M>,
}

impl<T: Instance> Uart<T, Blocking> {
    /// Creates a new blocking UART driver with given baud rate
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true
    pub fn new_blocking(_instance: T, baud_rate: u32, sim: bool, flow_control: bool) -> Self {
        let uart = Self {
            _instance: PhantomData,
            _mode: PhantomData,
        };

        uart.reset();

        if sim {
            T::reg()
                .ctrl()
                .modify(|_, w| w.uart_ctrl_sim_mode().set_bit());
        }

        if flow_control {
            T::reg()
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

        T::reg().ctrl().modify(|_, w| unsafe {
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
        T::reg()
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_nempty().bit(enabled));
    }

    /// Enable or disable RX FIFO full interrupt
    #[inline(always)]
    pub fn en_irq_rx_full(&self, enabled: bool) {
        T::reg()
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_full().bit(enabled));
    }

    /// Enable or disable TX FIFO empty interrupt
    #[inline(always)]
    pub fn en_irq_tx_empty(&self, enabled: bool) {
        T::reg()
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_empty().bit(enabled));
    }

    /// Enable or disable TX FIFO not full interrupt
    #[inline(always)]
    pub fn en_irq_tx_nfull(&self, enabled: bool) {
        T::reg()
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_nfull().bit(enabled));
    }
}

impl<T: Instance> Uart<T, Async> {
    /// Creates a new async UART driver with given baud rate
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true
    pub fn new_async(_instance: T, _baud_rate: u32, _sim: bool, _flow_control: bool) -> Self {
        todo!()
    }
}

impl<T: Instance, M: IoMode> Uart<T, M> {
    #[inline(always)]
    fn enable(&self) {
        T::reg().ctrl().modify(|_, w| w.uart_ctrl_en().set_bit());
    }

    #[inline(always)]
    fn disable(&self) {
        T::reg().ctrl().modify(|_, w| w.uart_ctrl_en().clear_bit());
    }

    #[inline(always)]
    fn reset(&self) {
        T::reg().ctrl().reset();
    }

    /// Reads a byte from RX FIFO, blocking if empty
    pub fn blocking_read_byte(&self) -> u8 {
        while self.rx_fifo_empty() {}
        T::reg().data().read().bits() as u8
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
        T::reg().data().write(|w| unsafe { w.bits(byte as u32) });
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
        T::reg().ctrl().read().uart_ctrl_rx_nempty().bit_is_clear()
    }

    /// Returns true if RX FIFO is full
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> bool {
        T::reg().ctrl().read().uart_ctrl_rx_full().bit_is_set()
    }

    /// Returns true if RX FIFO has overflowed
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> bool {
        T::reg().ctrl().read().uart_ctrl_rx_over().bit_is_set()
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
        T::reg().ctrl().read().uart_ctrl_tx_empty().bit_is_set()
    }

    /// Returns true if TX FIFO is full
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> bool {
        T::reg().ctrl().read().uart_ctrl_tx_nfull().bit_is_clear()
    }

    /// Returns depth of the RX FIFO
    #[inline(always)]
    pub fn rx_fifo_depth(&self) -> u8 {
        // TODO: Patch SVD to make this a field
        (T::reg().data().read().bits() >> 8) as u8 & 0b1111
    }

    /// Returns depth of the TX FIFO
    #[inline(always)]
    pub fn tx_fifo_depth(&self) -> u8 {
        // TODO: Patch SVD to make this a field
        (T::reg().data().read().bits() >> 12) as u8 & 0b1111
    }

    /// Blocks until all TX complete
    #[inline(always)]
    pub fn flush(&self) {
        while T::reg().ctrl().read().uart_ctrl_tx_busy().bit_is_set() {}
    }
}

impl<T: Instance, M: IoMode> Drop for Uart<T, M> {
    fn drop(&mut self) {
        self.flush();
        self.disable();
    }
}

// Convenience for writing formatted strings to UART
// TODO: Other Embassy HALs don't seem to do this so look at other approaches
// Don't like how it requires an &mut
impl<T: Instance, M: IoMode> core::fmt::Write for Uart<T, M> {
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
pub trait Instance: crate::Sealed {
    fn reg() -> &'static pac::uart0::RegisterBlock;
}

macro_rules! impl_instance {
    ($periph:ident) => {
        impl crate::Sealed for pac::$periph {}
        impl Instance for pac::$periph {
            #[inline(always)]
            fn reg() -> &'static pac::uart0::RegisterBlock {
                unsafe { &*pac::$periph::ptr() }
            }
        }
    };
}

impl_instance!(Uart0);
impl_instance!(Uart1);
