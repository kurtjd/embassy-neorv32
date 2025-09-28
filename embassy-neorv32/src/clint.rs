//! Core Local Interruptor (CLINT)
use crate::peripherals::CLINT;
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};

/// CLINT Driver
pub struct Clint<'d, T: Instance> {
    _phantom: PhantomData<&'d T>,
}

impl<'d, T: Instance> Clint<'d, T> {
    /// Create a new instance of a CLINT driver
    ///
    /// **Note**: If `time-driver` feature is enabled,
    /// `MTIMER` is read-only
    pub fn new(_instance: Peri<'d, T>) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Enable machine software interrupts
    pub fn mswi_enable(&self) {
        unsafe { T::reg().mswi().enable() }
    }

    /// Disable machine software interrupts
    pub fn mswi_disable(&self) {
        T::reg().mswi().disable();
    }

    /// Enable machine timer interrupts
    #[cfg(not(feature = "time-driver"))]
    pub fn mtimer_enable(&self) {
        unsafe { T::reg().mtimer().enable() }
    }

    /// Disable machine timer interrupts
    #[cfg(not(feature = "time-driver"))]
    pub fn mtimer_disable(&self) {
        T::reg().mtimer().disable();
    }

    /// Returns current value of machine timer compare register
    pub fn mtime_cmp(&self) -> u64 {
        T::reg().mtimer().mtimecmp0().read()
    }

    /// Sets the machine timer compare register value
    #[cfg(not(feature = "time-driver"))]
    pub fn set_mtime_cmp(&self, val: u64) {
        T::reg().mtimer().mtimecmp0().write(val);
    }

    /// Returns current value of machine timer register
    pub fn mtime(&self) -> u64 {
        T::reg().mtimer().mtime().read()
    }

    /// Sets the machine timer register value
    #[cfg(not(feature = "time-driver"))]
    pub fn set_mtime(&self, val: u64) {
        T::reg().mtimer().mtime().write(val);
    }
}

/// A valid CLINT peripheral
#[allow(private_bounds)]
pub trait Instance: crate::Sealed + PeripheralType {
    fn reg() -> crate::pac::Clint;
}

impl crate::Sealed for CLINT {}
impl Instance for CLINT {
    fn reg() -> crate::pac::Clint {
        unsafe { crate::pac::Clint::steal() }
    }
}
