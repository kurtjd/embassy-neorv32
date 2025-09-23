//! True Random-Number Generator (TRNG)
use core::marker::PhantomData;

/// True Random-Number Generator (TRNG) Driver
pub struct Trng<T: Instance, M: IoMode> {
    _instance: PhantomData<T>,
    _mode: PhantomData<M>,
}

impl<T: Instance> Trng<T, Blocking> {
    /// Returns a new instance of a blocking TRNG and enables it
    pub fn new_blocking(_instance: T) -> Self {
        let trng = Self {
            _instance: PhantomData,
            _mode: PhantomData,
        };

        trng.enable();
        trng
    }
}

impl<T: Instance> Trng<T, Async> {
    /// Returns a new instance of an async TRNG
    pub fn new_async(_instance: T) -> Self {
        todo!()
    }
}

impl<T: Instance, M: IoMode> Trng<T, M> {
    /// Enables the TRNG
    #[inline(always)]
    pub fn enable(&self) {
        T::reg().ctrl().modify(|_, w| w.trng_ctrl_en().set_bit());
    }

    /// Disables the TRNG, clearing the FIFO
    #[inline(always)]
    pub fn disable(&self) {
        T::reg().ctrl().modify(|_, w| w.trng_ctrl_en().clear_bit());
    }

    /// Flushes/clears the TRNG FIFO
    #[inline(always)]
    pub fn flush(&self) {
        T::reg()
            .ctrl()
            .modify(|_, w| w.trng_ctrl_fifo_clr().set_bit());
    }

    /// Returns the TRNG FIFO depth
    #[inline(always)]
    pub fn fifo_depth(&self) -> u8 {
        T::reg().ctrl().read().trng_ctrl_fifo_size().bits()
    }

    /// Returns true if TRNG is running in simulation
    ///
    /// If so, the output is pseudo-random as opposed to true random
    #[inline(always)]
    pub fn sim_mode(&self) -> bool {
        T::reg().ctrl().read().trng_ctrl_sim_mode().bit_is_set()
    }

    /// Returns true if TRNG data is available
    #[inline(always)]
    pub fn data_available(&self) -> bool {
        T::reg().ctrl().read().trng_ctrl_avail().bit_is_set()
    }

    /// Reads a byte from the TRNG if available, blocking if not
    pub fn blocking_read_byte(&self) -> u8 {
        while !self.data_available() {}
        T::reg().data().read().trng_data().bits()
    }

    /// Reads bytes from TRNG FIFO until buffer is full, blocking if empty
    pub fn blocking_read(&self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.blocking_read_byte();
        }
    }
}

/// TRNG IO mode
#[allow(private_bounds)]
pub trait IoMode: crate::Sealed {}

/// Blocking TRNG
pub struct Blocking;
impl crate::Sealed for Blocking {}
impl IoMode for Blocking {}

// TODO: Actually add async support
/// Async TRNG
pub struct Async;
impl crate::Sealed for Async {}
impl IoMode for Async {}

/// A valid TRNG peripheral
#[allow(private_bounds)]
pub trait Instance: crate::Sealed {
    fn reg() -> &'static pac::trng::RegisterBlock;
}

impl crate::Sealed for pac::Trng {}
impl Instance for pac::Trng {
    fn reg() -> &'static pac::trng::RegisterBlock {
        unsafe { &*pac::Trng::ptr() }
    }
}
