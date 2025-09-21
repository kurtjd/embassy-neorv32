//! UART
use core::marker::PhantomData;

/// UART Error
pub enum Error {
    /// TX FIFO is full
    TxFifoFull,
    /// RX FIFO is empty
    RxFifoEmpty,
}

/// UART driver capable of Rx and Tx
pub struct Uart<T: Instance, M: Mode> {
    _instance: PhantomData<T>,
    _mode: PhantomData<M>,
}

impl<T: Instance> Uart<T, Blocking> {
    /// Creates a new blocking (sync) UART driver with given baud rate
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

        // TODO: Get main clock freq either statically or runtime via SysInfo
        let mut baud_div = 100_000_000 / (2 * baud_rate);
        let mut prsc_sel = 0;

        // Calculate clock prescaler and baud rate prescaler
        // See: https://github.com/stnolting/neorv32/blob/main/sw/lib/source/neorv32_uart.c#L47
        while baud_div >= 0xfff {
            if prsc_sel == 2 || prsc_sel == 4 {
                baud_div >>= 3;
            } else {
                baud_div >>= 1;
            }
            prsc_sel += 1;
        }

        // TODO: Docs give conflicting info on baud rate prescaler size (10 vs 12 bits). Investigate.
        // Going with 12 bits for now
        T::reg().ctrl().modify(|_, w| unsafe {
            w.uart_ctrl_prsc()
                .bits(prsc_sel & 0b111)
                .uart_ctrl_baud()
                .bits((baud_div as u16 - 1) & 0xfff)
        });

        uart.enable();
        uart
    }

    /// Reads a byte from RX FIFO if not empty, otherwise error
    pub fn read_byte(&self) -> Result<u8, Error> {
        if !self.rx_fifo_empty() {
            Ok(T::reg().data().read().bits() as u8)
        } else {
            Err(Error::RxFifoEmpty)
        }
    }

    /// Reads bytes from RX FIFO if not empty until buffer is full, otherwise error
    pub fn read(&self, buf: &mut [u8]) -> Result<(), Error> {
        for byte in buf.iter_mut() {
            *byte = self.read_byte()?;
        }

        Ok(())
    }

    /// Writes a byte to TX FIFO if not full, otherwise error
    pub fn write_byte(&self, byte: u8) -> Result<(), Error> {
        if !self.tx_fifo_full() {
            T::reg().data().write(|w| unsafe { w.bits(byte as u32) });
            Ok(())
        } else {
            Err(Error::TxFifoFull)
        }
    }

    /// Writes bytes to TX FIFO if not full, otherwise error
    pub fn write(&self, bytes: &[u8]) -> Result<(), Error> {
        for byte in bytes {
            self.write_byte(*byte)?;
        }

        Ok(())
    }

    /// Writes a character to TX FIFO if not full, otherwise error
    #[inline(always)]
    pub fn putc(&self, c: char) -> Result<(), Error> {
        self.write_byte(c as u8)
    }

    /// Writes a string to TX FIFO if not full, otherwise error
    #[inline(always)]
    pub fn puts(&self, s: &str) -> Result<(), Error> {
        self.write(s.as_bytes())
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
    pub fn new(_instance: T, _baud_rate: u32, _sim: bool, _flow_control: bool) -> Self {
        todo!()
    }
}

impl<T: Instance, M: Mode> Uart<T, M> {
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
    pub fn read_byte_blocking(&self) -> u8 {
        while self.rx_fifo_empty() {}
        T::reg().data().read().bits() as u8
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty
    pub fn read_blocking(&self, buf: &mut [u8]) {
        for byte in buf.iter_mut() {
            *byte = self.read_byte_blocking();
        }
    }

    /// Writes a byte to TX FIFO, blocking if full
    pub fn write_byte_blocking(&self, byte: u8) {
        while self.tx_fifo_full() {}
        T::reg().data().write(|w| unsafe { w.bits(byte as u32) });
    }

    /// Writes bytes to TX FIFO, blocking if full
    pub fn write_blocking(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte_blocking(*byte);
        }
    }

    /// Writes a character to TX FIFO, blocking if full
    #[inline(always)]
    pub fn putc_blocking(&self, c: char) {
        self.write_byte_blocking(c as u8);
    }

    /// Writes a string to TX FIFO, blocking if full
    #[inline(always)]
    pub fn puts_blocking(&self, s: &str) {
        self.write_blocking(s.as_bytes());
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

impl<T: Instance, M: Mode> Drop for Uart<T, M> {
    fn drop(&mut self) {
        self.flush();
        self.disable();
    }
}

trait Sealed {}

#[allow(private_bounds)]
pub trait Mode: Sealed {}

pub struct Blocking;
impl Sealed for Blocking {}
impl Mode for Blocking {}

// TODO: Actually add async support
pub struct Async;
impl Sealed for Async {}
impl Mode for Async {}

#[allow(private_bounds)]
pub trait Instance: Sealed {
    fn reg() -> &'static pac::uart0::RegisterBlock;
}

macro_rules! impl_instance {
    ($periph:ident) => {
        impl Sealed for pac::$periph {}
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
