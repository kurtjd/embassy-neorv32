//! Universal Asynchronous Receiver and Transmitter (UART)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::{UART0, UART1};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};
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
        if T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_rx_nempty()
            .bit_is_set()
        {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_rx_nempty().clear_bit());
            T::info().rx_waker.wake();
        }

        // If TX FIFO is not full, disable TX not full IRQ and wake TX task
        if T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_tx_nfull()
            .bit_is_set()
        {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_tx_nfull().clear_bit());
            T::info().tx_waker.wake();
        }
    }
}

/// UART driver.
pub struct Uart<'d, M: IoMode> {
    rx: UartRx<'d, M>,
    tx: UartTx<'d, M>,
}

impl<'d, M: IoMode> Uart<'d, M> {
    fn init<T: Instance>(_instance: Peri<'d, T>, baud_rate: u32, sim: bool, flow_control: bool) {
        // Enable simulation mode if applicable
        if sim {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_sim_mode().set_bit());
        }

        // Enable flow control if applicable
        if flow_control {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_hwfc_en().set_bit());
        }

        let cpu_freq = crate::sysinfo::SysInfo::clock_freq();
        let mut baud_div = cpu_freq / (2 * baud_rate);
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
        T::info().reg.ctrl().modify(|_, w| unsafe {
            w.uart_ctrl_prsc()
                .bits(prsc_sel & 0b111)
                .uart_ctrl_baud()
                .bits((baud_div as u16 - 1) & 0x3ff)
        });

        // Enable UART
        T::info()
            .reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_en().set_bit());
    }

    fn new_inner<T: Instance>() -> Self {
        let rx = UartRx::new_inner::<T>();
        let tx = UartTx::new_inner::<T>();
        Self { rx, tx }
    }

    /// Reads a byte from RX FIFO, blocking if empty.
    pub fn blocking_read_byte(&self) -> u8 {
        self.rx.blocking_read_byte()
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty.
    pub fn blocking_read(&self, buf: &mut [u8]) {
        self.rx.blocking_read(buf);
    }

    /// Writes a byte to TX FIFO, blocking if full.
    pub fn blocking_write_byte(&mut self, byte: u8) {
        self.tx.blocking_write_byte(byte);
    }

    /// Writes bytes to TX FIFO, blocking if full.
    pub fn blocking_write(&mut self, bytes: &[u8]) {
        self.tx.blocking_write(bytes);
    }

    /// Blocks until all TX complete.
    pub fn tx_flush(&mut self) {
        self.tx.flush();
    }

    /// Splits the UART driver into separate [UartRx] and [UartTx] drivers.
    ///
    /// Helpful for sharing the UART among receiver/transmitter tasks.
    pub fn split(self) -> (UartRx<'d, M>, UartTx<'d, M>) {
        (self.rx, self.tx)
    }

    /// Splits the UART driver into separate [UartRx] and [UartTx] drivers by mutable reference.
    ///
    /// Helpful for sharing the UART among receiver/transmitter tasks without destroying the original [Uart] instance.
    pub fn split_ref(&mut self) -> (&mut UartRx<'d, M>, &mut UartTx<'d, M>) {
        (&mut self.rx, &mut self.tx)
    }
}

impl<'d> Uart<'d, Blocking> {
    /// Creates a new blocking UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> Self {
        Self::init(_instance, baud_rate, sim, flow_control);
        Self::new_inner::<T>()
    }
}

impl<'d> Uart<'d, Async> {
    /// Creates a new async UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Self::init(_instance, baud_rate, sim, flow_control);
        // SAFETY: It is valid to enable UART interrupt here
        unsafe { T::Interrupt::enable() }
        Self::new_inner::<T>()
    }

    /// Reads a byte from RX FIFO.
    pub fn read_byte(&mut self) -> impl Future<Output = u8> {
        self.rx.read_byte()
    }

    /// Reads bytes from RX FIFO until buffer is full.
    pub fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = ()> {
        self.rx.read(buf)
    }

    /// Writes a byte to TX FIFO.
    pub fn write_byte(&mut self, byte: u8) -> impl Future<Output = ()> {
        self.tx.write_byte(byte)
    }

    /// Writes bytes from buffer to TX FIFO.
    pub fn write(&mut self, bytes: &[u8]) -> impl Future<Output = ()> {
        self.tx.write(bytes)
    }
}

/// RX-only UART driver.
pub struct UartRx<'d, M: IoMode> {
    info: Info,
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: IoMode> UartRx<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        // Mark RX as active
        T::info().active.rx.store(true, Ordering::SeqCst);

        Self {
            info: T::info(),
            _phantom: PhantomData,
        }
    }

    fn read_unchecked(&self) -> u8 {
        self.info.reg.data().read().bits() as u8
    }

    fn enable_irq_rx_nempty(&mut self) {
        self.info
            .reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_nempty().set_bit());
    }

    fn fifo_empty(&self) -> bool {
        self.info
            .reg
            .ctrl()
            .read()
            .uart_ctrl_rx_nempty()
            .bit_is_clear()
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

impl<'d> UartRx<'d, Blocking> {
    /// Creates a new RX-only blocking UART driver with given baud rate.
    ///
    /// Enables hardware flow control if `flow_control` is true.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        flow_control: bool,
    ) -> Self {
        Uart::<Blocking>::init(_instance, baud_rate, false, flow_control);
        Self::new_inner::<T>()
    }
}

impl<'d> UartRx<'d, Async> {
    /// Creates a new RX-only async UART driver with given baud rate.
    ///
    /// Enables hardware flow control if `flow_control` is true.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Uart::<Async>::init(_instance, baud_rate, false, flow_control);
        // SAFETY: It is valid to enable UART interrupt here
        unsafe { T::Interrupt::enable() }
        Self::new_inner::<T>()
    }

    /// Reads a byte from RX FIFO.
    pub async fn read_byte(&mut self) -> u8 {
        poll_fn(|cx| {
            self.info.rx_waker.register(cx.waker());
            if !self.fifo_empty() {
                Poll::Ready(self.read_unchecked())
            } else {
                // CS used here since interrupt modifies register
                critical_section::with(|_| self.enable_irq_rx_nempty());
                Poll::Pending
            }
        })
        .await
    }

    /// Reads bytes from RX FIFO until buffer is full.
    pub async fn read(&mut self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.read_byte().await;
        }
    }
}

impl<'d, M: IoMode> Drop for UartRx<'d, M> {
    fn drop(&mut self) {
        self.info.active.rx.store(false, Ordering::SeqCst);
        drop_rx_tx(&self.info);
    }
}

/// TX-only UART driver.
pub struct UartTx<'d, M: IoMode> {
    info: Info,
    _phantom: PhantomData<&'d M>,
}

// TODO: Revisit soundness of this
unsafe impl<'d, M: IoMode> Send for UartTx<'d, M> {}

impl<'d, M: IoMode> UartTx<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        // Mark TX as active
        T::info().active.rx.store(true, Ordering::SeqCst);

        Self {
            info: T::info(),
            _phantom: PhantomData,
        }
    }

    fn write_unchecked(&mut self, byte: u8) {
        // SAFETY: We are just writing a byte, the MSB bits are read-only
        self.info
            .reg
            .data()
            .write(|w| unsafe { w.bits(byte as u32) });
    }

    fn enable_irq_tx_nfull(&mut self) {
        self.info
            .reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_nfull().set_bit());
    }

    fn fifo_full(&self) -> bool {
        self.info
            .reg
            .ctrl()
            .read()
            .uart_ctrl_tx_nfull()
            .bit_is_clear()
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
        while self.info.reg.ctrl().read().uart_ctrl_tx_busy().bit_is_set() {}
    }
}

impl<'d> UartTx<'d, Blocking> {
    /// Creates a new TX-only blocking UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> Self {
        Uart::<Blocking>::init(_instance, baud_rate, sim, flow_control);
        Self::new_inner::<T>()
    }
}

impl<'d> UartTx<'d, Async> {
    /// Creates a new TX-only async UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Uart::<Async>::init(_instance, baud_rate, sim, flow_control);
        // SAFETY: It is valid to enable UART interrupt here
        unsafe { T::Interrupt::enable() }
        Self::new_inner::<T>()
    }

    /// Writes a byte to TX FIFO.
    pub async fn write_byte(&mut self, byte: u8) {
        poll_fn(|cx| {
            self.info.tx_waker.register(cx.waker());
            if !self.fifo_full() {
                self.write_unchecked(byte);
                Poll::Ready(())
            } else {
                // CS used here since interrupt modifies register
                critical_section::with(|_| self.enable_irq_tx_nfull());
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

impl<'d, M: IoMode> Drop for UartTx<'d, M> {
    fn drop(&mut self) {
        self.info.active.tx.store(false, Ordering::SeqCst);
        drop_rx_tx(&self.info);
    }
}

fn drop_rx_tx(info: &Info) {
    // Only disable UART if both Rx and Tx have been dropped
    if !info.active.rx.load(Ordering::SeqCst) && !info.active.tx.load(Ordering::SeqCst) {
        info.reg.ctrl().modify(|_, w| w.uart_ctrl_en().clear_bit());
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

// Serves as a "reference-counter" so we know when Uart is completely dropped
// Use two AtomicBools instead of AtomicU8 since fetch_add/fetch_sub are not available without A extension
struct Active {
    rx: AtomicBool,
    tx: AtomicBool,
}

impl Active {
    const fn new() -> Self {
        Self {
            rx: AtomicBool::new(false),
            tx: AtomicBool::new(false),
        }
    }
}

struct Info {
    reg: &'static crate::pac::uart0::RegisterBlock,
    active: &'static Active,
    rx_waker: &'static AtomicWaker,
    tx_waker: &'static AtomicWaker,
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
    fn info() -> Info;
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
            fn info() -> Info {
                static RX_WAKER: AtomicWaker = AtomicWaker::new();
                static TX_WAKER: AtomicWaker = AtomicWaker::new();
                static ACTIVE: Active = Active::new();

                Info {
                    reg: unsafe { &*crate::pac::$rb::ptr() },
                    active: &ACTIVE,
                    rx_waker: &RX_WAKER,
                    tx_waker: &TX_WAKER,
                }
            }
        }
        impl Instance for $periph {
            type Interrupt = crate::interrupt::typelevel::$periph;
        }
    };
}

impl_instance!(UART0, Uart0);
impl_instance!(UART1, Uart1);
