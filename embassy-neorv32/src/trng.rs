//! True Random-Number Generator (TRNG)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::TRNG;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

// TRNG interrupt handler binding.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        T::Interrupt::disable();
        T::waker().wake();
    }
}

/// True Random-Number Generator (TRNG) Driver.
pub struct Trng<'d, M: ReadMode> {
    reg: &'static crate::pac::trng::RegisterBlock,
    waker: &'static AtomicWaker,
    _phantom: PhantomData<&'d M>,
}

impl<'d> Trng<'d, Blocking> {
    /// Returns a new instance of a blocking TRNG and enables it.
    pub fn new_blocking<T: Instance>(_instance: Peri<'d, T>) -> Self {
        Self::new_inner(_instance)
    }
}

impl<'d> Trng<'d, Async> {
    /// Returns a new instance of an async TRNG and enables it.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Self::new_inner(_instance)
    }

    /// Reads a byte from the TRNG.
    pub async fn read_byte(&mut self) -> u8 {
        // Note: If data is not available immediately, we won't actually get woken up again
        // until the FIFO is completely full (not just when a byte is available).
        //
        // Not ideal, but that is the only interrupt trigger available.
        poll_fn(|cx| {
            self.waker.register(cx.waker());
            if self.data_available() {
                Poll::Ready(self.read_unchecked())
            } else {
                unsafe { crate::enable_periph_irq!(TRNG) }
                Poll::Pending
            }
        })
        .await
    }

    /// Reads bytes from TRNG FIFO until buffer is full.
    pub async fn read(&mut self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.read_byte().await;
        }
    }
}

impl<'d, M: ReadMode> Trng<'d, M> {
    fn new_inner<T: Instance>(_instance: Peri<'d, T>) -> Self {
        let mut trng = Self {
            reg: T::reg(),
            waker: T::waker(),
            _phantom: PhantomData,
        };

        trng.enable();
        trng
    }

    // Private convenience wrapper around raw register
    fn read_unchecked(&self) -> u8 {
        self.reg.data().read().trng_data().bits()
    }

    /// Enables the TRNG.
    #[inline(always)]
    pub fn enable(&mut self) {
        self.reg.ctrl().modify(|_, w| w.trng_ctrl_en().set_bit());
    }

    /// Disables the TRNG, clearing the FIFO.
    #[inline(always)]
    pub fn disable(&mut self) {
        self.reg.ctrl().modify(|_, w| w.trng_ctrl_en().clear_bit());
    }

    /// Flushes/clears the TRNG FIFO.
    #[inline(always)]
    pub fn flush(&mut self) {
        self.reg
            .ctrl()
            .modify(|_, w| w.trng_ctrl_fifo_clr().set_bit());
    }

    /// Returns the TRNG FIFO depth.
    #[inline(always)]
    pub fn fifo_depth(&self) -> u32 {
        // Read value is log2, so do inverse log for actual value
        1 << self.reg.ctrl().read().trng_ctrl_fifo_size().bits() as u32
    }

    /// Returns true if TRNG is running in simulation.
    ///
    /// If so, the output is pseudo-random as opposed to true random.
    #[inline(always)]
    pub fn sim_mode(&self) -> bool {
        self.reg.ctrl().read().trng_ctrl_sim_mode().bit_is_set()
    }

    /// Returns true if TRNG data is available.
    #[inline(always)]
    pub fn data_available(&self) -> bool {
        self.reg.ctrl().read().trng_ctrl_avail().bit_is_set()
    }

    /// Reads a byte from the TRNG if available, blocking if not.
    pub fn blocking_read_byte(&self) -> u8 {
        while !self.data_available() {}
        self.read_unchecked()
    }

    /// Reads bytes from TRNG FIFO until buffer is full, blocking if empty.
    pub fn blocking_read(&self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.blocking_read_byte();
        }
    }
}

trait SealedReadMode {}

/// TRNG Read mode.
#[allow(private_bounds)]
pub trait ReadMode: SealedReadMode {}

/// Blocking TRNG.
pub struct Blocking;
impl SealedReadMode for Blocking {}
impl ReadMode for Blocking {}

/// Async TRNG.
pub struct Async;
impl SealedReadMode for Async {}
impl ReadMode for Async {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::trng::RegisterBlock;
    fn waker() -> &'static AtomicWaker;
}

/// A valid TRNG peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    type Interrupt: Interrupt;
}
impl SealedInstance for TRNG {
    fn reg() -> &'static crate::pac::trng::RegisterBlock {
        unsafe { &*crate::pac::Trng::ptr() }
    }

    fn waker() -> &'static AtomicWaker {
        static WAKER: AtomicWaker = AtomicWaker::new();
        &WAKER
    }
}
impl Instance for TRNG {
    type Interrupt = crate::interrupt::typelevel::TRNG;
}
