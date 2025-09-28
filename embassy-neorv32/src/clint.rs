//! Core Local Interruptor (CLINT)
use crate::peripherals::CLINT;
use embassy_hal_internal::{Peri, PeripheralType};

/// CLINT Driver
pub struct Clint<'d, T: Instance> {
    _instance: Peri<'d, T>,
}

impl<'d, T: Instance> Clint<'d, T> {
    /// Create a new instance of a CLINT driver.
    ///
    /// **Note**: If `time-driver` feature is enabled,
    /// `MTIMER` is read-only.
    pub fn new(_instance: Peri<'d, T>) -> Self {
        Self { _instance }
    }

    /// Enable machine software interrupts.
    pub fn mswi_enable(&mut self) {
        unsafe { T::reg().mswi().enable() }
    }

    /// Disable machine software interrupts.
    pub fn mswi_disable(&mut self) {
        T::reg().mswi().disable();
    }

    /// Enable machine timer interrupts.
    #[cfg(not(feature = "time-driver"))]
    pub fn mtimer_enable(&mut self) {
        unsafe { T::reg().mtimer().enable() }
    }

    /// Disable machine timer interrupts.
    #[cfg(not(feature = "time-driver"))]
    pub fn mtimer_disable(&mut self) {
        T::reg().mtimer().disable();
    }

    /// Returns current value of machine timer compare register.
    pub fn mtime_cmp(&self) -> u64 {
        T::reg().mtimer().mtimecmp0().read()
    }

    /// Sets the machine timer compare register value.
    #[cfg(not(feature = "time-driver"))]
    pub fn set_mtime_cmp(&mut self, val: u64) {
        T::reg().mtimer().mtimecmp0().write(val);
    }

    /// Returns current value of machine timer register.
    pub fn mtime(&self) -> u64 {
        T::reg().mtimer().mtime().read()
    }

    /// Sets the machine timer register value.
    #[cfg(not(feature = "time-driver"))]
    pub fn set_mtime(&mut self, val: u64) {
        T::reg().mtimer().mtime().write(val);
    }
}

trait SealedInstance {
    fn reg() -> crate::pac::Clint;
}

/// A valid CLINT peripheral
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}
impl SealedInstance for CLINT {
    fn reg() -> crate::pac::Clint {
        unsafe { crate::pac::Clint::steal() }
    }
}
impl Instance for CLINT {}
