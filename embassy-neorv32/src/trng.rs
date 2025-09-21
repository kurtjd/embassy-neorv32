//! True Random-Number Generator (TRNG)
use core::marker::PhantomData;

/// TRNG Error
pub enum Error {
    /// Data not available
    NotAvailable,
}

/// True Random-Number Generator (TRNG) Driver
pub struct Trng<T: Instance, M: Mode> {
    _instance: PhantomData<T>,
    _mode: PhantomData<M>,
}

impl<T: Instance> Trng<T, Blocking> {
    /// Returns a new instance of a blocking (sync) TRNG and enables it
    pub fn new_blocking(_instance: T) -> Self {
        let trng = Self {
            _instance: PhantomData,
            _mode: PhantomData,
        };

        trng.enable();
        trng
    }

    /// Reads a byte from the TRNG if available, returns error otherwise
    pub fn read(&self) -> Result<u8, Error> {
        if self.data_available() {
            Ok(T::reg().data().read().trng_data().bits())
        } else {
            Err(Error::NotAvailable)
        }
    }
}

impl<T: Instance> Trng<T, Async> {
    /// Returns a new instance of an async TRNG
    pub fn new(_instance: T) -> Self {
        todo!()
    }
}

impl<T: Instance, M: Mode> Trng<T, M> {
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
    pub fn blocking_read(&self) -> u8 {
        while !self.data_available() {}
        T::reg().data().read().trng_data().bits()
    }
}

/// TRNG operating mode
#[allow(private_bounds)]
pub trait Mode: crate::Sealed {}

/// Blocking (sync) TRNG
pub struct Blocking;
impl crate::Sealed for Blocking {}
impl Mode for Blocking {}

// TODO: Actually add async support
/// Async TRNG
pub struct Async;
impl crate::Sealed for Async {}
impl Mode for Async {}

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
