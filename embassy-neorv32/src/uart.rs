//! Universal Asynchronous Receiver and Transmitter (UART)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::{UART0, UART1};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

/// UART interrupt handler binding.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        // If RX FIFO is not empty, disable RX not empty IRQ and wake RX task
        if T::reg().ctrl().read().uart_ctrl_rx_nempty().bit_is_set() {
            T::reg()
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_rx_nempty().clear_bit());
            T::rx_waker().wake();
        }

        // If TX FIFO is not full, disable TX not full IRQ and wake TX task
        if T::reg().ctrl().read().uart_ctrl_tx_nfull().bit_is_set() {
            T::reg()
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_tx_nfull().clear_bit());
            T::tx_waker().wake();
        }

        // TODO: Investigate if it makes sense to handle RX FIFO full and TX FIFO empty IRQs
    }
}

/// UART driver.
pub struct Uart<'d, M: IoMode> {
    rx: UartRx<'d, M>,
    tx: UartTx<'d, M>,
}

impl<'d> Uart<'d, Blocking> {
    /// Creates a new blocking UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    ///
    /// **Note**: Must call [Self::split] to create Rx and Tx drivers.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> Self {
        Self::new_inner(_instance, baud_rate, sim, flow_control)
    }

    /// Creates a new RX-only blocking UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_blocking_rx<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> UartRx<'d, Blocking> {
        let uart = Self::new_inner(_instance, baud_rate, sim, flow_control);
        uart.rx
    }

    /// Creates a new TX-only blocking UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_blocking_tx<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> UartTx<'d, Blocking> {
        let uart = Self::new_inner(_instance, baud_rate, sim, flow_control);
        uart.tx
    }
}

impl<'d> Uart<'d, Async> {
    /// Creates a new async UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    ///
    /// **Note**: Must call [Self::split] to create Rx and Tx drivers.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        let uart = Self::new_inner(_instance, baud_rate, sim, flow_control);
        unsafe { T::Interrupt::enable() }
        uart
    }

    /// Creates a new RX-only async UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_async_rx<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> UartRx<'d, Async> {
        let uart = Self::new_inner(_instance, baud_rate, sim, flow_control);
        unsafe { T::Interrupt::enable() }
        uart.rx
    }

    /// Creates a new TX-only async UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_async_tx<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> UartTx<'d, Async> {
        let uart = Self::new_inner(_instance, baud_rate, sim, flow_control);
        unsafe { T::Interrupt::enable() }
        uart.tx
    }
}

impl<'d, M: IoMode> Uart<'d, M> {
    fn new_inner<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> Self {
        // Enable simulation mode if applicable
        if sim {
            T::reg()
                .ctrl()
                .modify(|_, w| w.uart_ctrl_sim_mode().set_bit());
        }

        // Enable flow control if applicable
        if flow_control {
            T::reg()
                .ctrl()
                .modify(|_, w| w.uart_ctrl_hwfc_en().set_bit());
        }

        let mut baud_div = crate::sysinfo::SysInfo::clock_freq() / (2 * baud_rate);
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

        // Set the clock and baudrate prescalers
        T::reg().ctrl().modify(|_, w| unsafe {
            w.uart_ctrl_prsc()
                .bits(prsc_sel & 0b111)
                .uart_ctrl_baud()
                .bits((baud_div as u16 - 1) & 0x3ff)
        });

        // Enable UART
        T::reg().ctrl().modify(|_, w| w.uart_ctrl_en().set_bit());

        // Create RX
        let tx = UartTx {
            reg: T::reg(),
            waker: T::tx_waker(),
            _phantom: PhantomData,
        };

        // Create TX
        let rx = UartRx {
            reg: T::reg(),
            waker: T::rx_waker(),
            _phantom: PhantomData,
        };

        // Finally return it
        Self { rx, tx }
    }

    /// Splits the UART driver into separate [UartRx] and [UartTx] drivers.
    ///
    /// Helpful for sharing the UART among receiver/transmitter tasks.
    pub fn split(self) -> (UartRx<'d, M>, UartTx<'d, M>) {
        (self.rx, self.tx)
    }
}

/// RX-only UART driver.
pub struct UartRx<'d, M: IoMode> {
    reg: &'static crate::pac::uart0::RegisterBlock,
    waker: &'static AtomicWaker,
    _phantom: PhantomData<&'d M>,
}

impl<'d> UartRx<'d, Blocking> {
    /// Enable or disable RX FIFO not empty interrupt.
    pub fn enable_irq_rx_nempty(&mut self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_nempty().bit(enabled));
    }

    /// Enable or disable RX FIFO full interrupt.
    pub fn enable_irq_rx_full(&mut self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_full().bit(enabled));
    }
}

impl<'d> UartRx<'d, Async> {
    /// Reads a byte from RX FIFO.
    pub async fn read_byte(&self) -> u8 {
        // If data is available, return it immediately
        if !self.fifo_empty() {
            return self.read_unchecked();
        }

        // Otherwise, enable interrupt and wait
        // Note: CS used here since interrupt modifies register
        critical_section::with(|_| {
            self.reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_rx_nempty().set_bit())
        });

        poll_fn(|cx| {
            self.waker.register(cx.waker());
            if !self.fifo_empty() {
                Poll::Ready(self.read_unchecked())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Reads bytes from RX FIFO until buffer is full.
    pub async fn read(&self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.read_byte().await;
        }
    }
}

impl<'d, M: IoMode> UartRx<'d, M> {
    // Private convenience wrapper around reading DATA
    fn read_unchecked(&self) -> u8 {
        self.reg.data().read().bits() as u8
    }

    /// Returns true if RX FIFO is empty.
    pub fn fifo_empty(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_rx_nempty().bit_is_clear()
    }

    /// Returns true if RX FIFO is full.
    pub fn fifo_full(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_rx_full().bit_is_set()
    }

    /// Returns true if RX FIFO has overflowed.
    pub fn fifo_overflow(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_rx_over().bit_is_set()
    }

    /// Returns depth of the RX FIFO.
    pub fn fifo_depth(&self) -> u32 {
        // TODO: Patch SVD to make this a field
        // TODO: Since shares reg with DATA, this also removes byte from FIFO... not good
        // Read value is log2, so do inverse log for actual value
        let raw = (self.reg.data().read().bits() >> 8) as u8 & 0b1111;
        1 << raw as u32
    }

    /// Reads a byte from RX FIFO, blocking if empty.
    pub fn blocking_read_byte(&self) -> u8 {
        while self.fifo_empty() {}
        self.read_unchecked()
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty.
    pub fn blocking_read(&self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.blocking_read_byte();
        }
    }
}

/// TX-only UART driver.
pub struct UartTx<'d, M: IoMode> {
    reg: &'static crate::pac::uart0::RegisterBlock,
    waker: &'static AtomicWaker,
    _phantom: PhantomData<&'d M>,
}

// TODO: Revisit soundness of this
unsafe impl<'d, M: IoMode> Send for UartTx<'d, M> {}

impl<'d> UartTx<'d, Blocking> {
    /// Enable or disable TX FIFO empty interrupt.
    pub fn enable_irq_tx_empty(&mut self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_empty().bit(enabled));
    }

    /// Enable or disable TX FIFO not full interrupt.
    pub fn enable_irq_tx_nfull(&mut self, enabled: bool) {
        self.reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_nfull().bit(enabled));
    }
}

impl<'d> UartTx<'d, Async> {
    /// Writes a byte to TX FIFO.
    pub async fn write_byte(&mut self, byte: u8) {
        // If FIFO isn't full, write and return immediately
        if !self.fifo_full() {
            return self.write_unchecked(byte);
        }

        // Otherwise, enable interrupt and wait
        // Note: CS used here since interrupt modifies register
        critical_section::with(|_| {
            self.reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_tx_nfull().set_bit())
        });

        poll_fn(|cx| {
            self.waker.register(cx.waker());
            if !self.fifo_full() {
                self.write_unchecked(byte);
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Writes bytes from buffer to TX FIFO.
    pub async fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte).await;
        }
    }
}

impl<'d, M: IoMode> UartTx<'d, M> {
    // Private convenience wrapper around wrtiting a byte to DATA
    fn write_unchecked(&mut self, byte: u8) {
        self.reg.data().write(|w| unsafe { w.bits(byte as u32) });
    }

    /// Returns true if TX FIFO is empty.
    pub fn fifo_empty(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_tx_empty().bit_is_set()
    }

    /// Returns true if TX FIFO is full.
    pub fn fifo_full(&self) -> bool {
        self.reg.ctrl().read().uart_ctrl_tx_nfull().bit_is_clear()
    }

    /// Returns depth of the TX FIFO.
    pub fn fifo_depth(&self) -> u32 {
        // TODO: Patch SVD to make this a field
        // TODO: Since shares reg with DATA, this also removes byte from FIFO... not good
        // Read value is log2, so do inverse log for actual value
        let raw = (self.reg.data().read().bits() >> 12) as u8 & 0b1111;
        1 << raw as u32
    }

    /// Writes a byte to TX FIFO, blocking if full.
    pub fn blocking_write_byte(&mut self, byte: u8) {
        while self.fifo_full() {}
        self.write_unchecked(byte);
    }

    /// Writes bytes to TX FIFO, blocking if full.
    pub fn blocking_write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.blocking_write_byte(*byte);
        }
    }

    /// Blocks until all TX complete.
    pub fn flush(&mut self) {
        while self.reg.ctrl().read().uart_ctrl_tx_busy().bit_is_set() {}
    }
}

// Convenience for writing formatted strings to UART
// TODO: Other Embassy HALs don't seem to do this so look at other approaches
impl<'d, M: IoMode> core::fmt::Write for UartTx<'d, M> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.blocking_write(s.as_bytes());
        Ok(())
    }
}

trait SealedIoMode {}

/// UART IO mode.
#[allow(private_bounds)]
pub trait IoMode: SealedIoMode {}

/// Blocking UART.
pub struct Blocking;
impl SealedIoMode for Blocking {}
impl IoMode for Blocking {}

/// Async UART.
pub struct Async;
impl SealedIoMode for Async {}
impl IoMode for Async {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::uart0::RegisterBlock;
    fn rx_waker() -> &'static AtomicWaker;
    fn tx_waker() -> &'static AtomicWaker;
}

/// A valid UART peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    type Interrupt: Interrupt;
}

macro_rules! impl_instance {
    ($periph:ident, $rb:ident) => {
        impl SealedInstance for $periph {
            // Note: uart0 and uart1 can both share uart0::RegisterBlock
            // PAC is able to coerce uart1::ptr() to it with correct base address
            #[inline(always)]
            fn reg() -> &'static crate::pac::uart0::RegisterBlock {
                unsafe { &*crate::pac::$rb::ptr() }
            }

            #[inline(always)]
            fn rx_waker() -> &'static AtomicWaker {
                static WAKER: AtomicWaker = AtomicWaker::new();
                &WAKER
            }

            #[inline(always)]
            fn tx_waker() -> &'static AtomicWaker {
                static WAKER: AtomicWaker = AtomicWaker::new();
                &WAKER
            }
        }
        impl Instance for $periph {
            type Interrupt = crate::interrupt::typelevel::$periph;
        }
    };
}

impl_instance!(UART0, Uart0);
impl_instance!(UART1, Uart1);
