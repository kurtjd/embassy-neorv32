//! True Random-Number Generator (TRNG)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::TRNG;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

static TRNG_WAKER: AtomicWaker = AtomicWaker::new();

// TRNG interrupt handler binding.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        T::Interrupt::disable();
        TRNG_WAKER.wake();
    }
}

/// True Random-Number Generator (TRNG) Driver.
pub struct Trng<'d, T: Instance, M: ReadMode> {
    _instance: Peri<'d, T>,
    _phantom: PhantomData<M>,
}

impl<'d, T: Instance> Trng<'d, T, Blocking> {
    /// Returns a new instance of a blocking TRNG and enables it.
    pub fn new_blocking(_instance: Peri<'d, T>) -> Self {
        Self::new_inner(_instance)
    }
}

impl<'d, T: Instance> Trng<'d, T, Async> {
    /// Returns a new instance of an async TRNG and enables it.
    pub fn new_async(
        _instance: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Self::new_inner(_instance)
    }

    /// Reads a byte from the TRNG.
    pub async fn read_byte(&mut self) -> u8 {
        poll_fn(|cx| {
            TRNG_WAKER.register(cx.waker());
            if self.data_available() {
                Poll::Ready(self.blocking_read_byte())
            } else {
                unsafe {
                    T::Interrupt::enable();
                }
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

impl<'d, T: Instance, M: ReadMode> Trng<'d, T, M> {
    fn new_inner(_instance: Peri<'d, T>) -> Self {
        let mut trng = Self {
            _instance,
            _phantom: PhantomData,
        };

        trng.enable();
        trng
    }

    /// Enables the TRNG.
    #[inline(always)]
    pub fn enable(&mut self) {
        T::reg().ctrl().modify(|_, w| w.trng_ctrl_en().set_bit());
    }

    /// Disables the TRNG, clearing the FIFO.
    #[inline(always)]
    pub fn disable(&mut self) {
        T::reg().ctrl().modify(|_, w| w.trng_ctrl_en().clear_bit());
    }

    /// Flushes/clears the TRNG FIFO.
    #[inline(always)]
    pub fn flush(&mut self) {
        T::reg()
            .ctrl()
            .modify(|_, w| w.trng_ctrl_fifo_clr().set_bit());
    }

    /// Returns the TRNG FIFO depth.
    #[inline(always)]
    pub fn fifo_depth(&self) -> u32 {
        // Read value is log2, so do inverse log for actual value
        1 << T::reg().ctrl().read().trng_ctrl_fifo_size().bits() as u32
    }

    /// Returns true if TRNG is running in simulation.
    ///
    /// If so, the output is pseudo-random as opposed to true random.
    #[inline(always)]
    pub fn sim_mode(&self) -> bool {
        T::reg().ctrl().read().trng_ctrl_sim_mode().bit_is_set()
    }

    /// Returns true if TRNG data is available.
    #[inline(always)]
    pub fn data_available(&self) -> bool {
        T::reg().ctrl().read().trng_ctrl_avail().bit_is_set()
    }

    /// Reads a byte from the TRNG if available, blocking if not.
    pub fn blocking_read_byte(&self) -> u8 {
        while !self.data_available() {}
        T::reg().data().read().trng_data().bits()
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
}
impl Instance for TRNG {
    type Interrupt = crate::interrupt::typelevel::TRNG;
}
