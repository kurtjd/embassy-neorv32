//! True Random-Number Generator (TRNG)
use crate::peripherals::TRNG;
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};

/// True Random-Number Generator (TRNG) Driver
pub struct Trng<'d, M: IoMode> {
    reg: &'static pac::trng::RegisterBlock,
    _phantom: PhantomData<(&'d (), M)>,
}

impl<'d> Trng<'d, Blocking> {
    /// Returns a new instance of a blocking TRNG and enables it
    pub fn new_blocking<T: Instance>(_instance: Peri<'d, T>) -> Self {
        let trng = Self {
            reg: T::reg(),
            _phantom: PhantomData,
        };

        trng.enable();
        trng
    }
}

impl<'d> Trng<'d, Async> {
    /// Returns a new instance of an async TRNG
    pub fn new_async<T: Instance>(_instance: Peri<'d, T>) -> Self {
        todo!()
    }
}

impl<'d, M: IoMode> Trng<'d, M> {
    /// Enables the TRNG
    #[inline(always)]
    pub fn enable(&self) {
        self.reg.ctrl().modify(|_, w| w.trng_ctrl_en().set_bit());
    }

    /// Disables the TRNG, clearing the FIFO
    #[inline(always)]
    pub fn disable(&self) {
        self.reg.ctrl().modify(|_, w| w.trng_ctrl_en().clear_bit());
    }

    /// Flushes/clears the TRNG FIFO
    #[inline(always)]
    pub fn flush(&self) {
        self.reg
            .ctrl()
            .modify(|_, w| w.trng_ctrl_fifo_clr().set_bit());
    }

    /// Returns the TRNG FIFO depth
    #[inline(always)]
    pub fn fifo_depth(&self) -> u8 {
        self.reg.ctrl().read().trng_ctrl_fifo_size().bits()
    }

    /// Returns true if TRNG is running in simulation
    ///
    /// If so, the output is pseudo-random as opposed to true random
    #[inline(always)]
    pub fn sim_mode(&self) -> bool {
        self.reg.ctrl().read().trng_ctrl_sim_mode().bit_is_set()
    }

    /// Returns true if TRNG data is available
    #[inline(always)]
    pub fn data_available(&self) -> bool {
        self.reg.ctrl().read().trng_ctrl_avail().bit_is_set()
    }

    /// Reads a byte from the TRNG if available, blocking if not
    pub fn blocking_read_byte(&self) -> u8 {
        while !self.data_available() {}
        self.reg.data().read().trng_data().bits()
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
pub trait Instance: crate::Sealed + PeripheralType {
    fn reg() -> &'static pac::trng::RegisterBlock;
}

impl crate::Sealed for TRNG {}
impl Instance for TRNG {
    fn reg() -> &'static pac::trng::RegisterBlock {
        unsafe { &*pac::Trng::ptr() }
    }
}
