//! Watchdog Timer (WDT)
use crate::peripherals::WDT;
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};

/// Cause of last hardware reset
pub enum ResetCause {
    /// Reset caused by external reset signal pin
    External,
    /// Reset caused by on-chip debugger
    OnChipDebugger,
    /// Reset caused by watchdog timeout
    Timeout,
    /// Reset caused by illegal watchdog access
    IllegalAccess,
    /// Reset caused by unknown source
    Unknown,
}

impl ResetCause {
    /// Returns the WDT reset cause as a static string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::External => "External reset signal pin",
            Self::OnChipDebugger => "On-chip debugger",
            Self::Timeout => "Watchdog timeout",
            Self::IllegalAccess => "Illegal watchdog access",
            Self::Unknown => "Unknown source",
        }
    }
}

impl From<u8> for ResetCause {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Self::External,
            0b01 => Self::OnChipDebugger,
            0b10 => Self::Timeout,
            0b11 => Self::IllegalAccess,
            _ => Self::Unknown,
        }
    }
}

pub struct Wdt<'d, T: Instance, M: LockMode> {
    _instance: Peri<'d, T>,
    _phantom: PhantomData<M>,
}

impl<'d, T: Instance> Wdt<'d, T, Unlocked> {
    /// Returns a new unlocked WDT with timeout set to 24-bit max
    ///
    /// Caller should configure timeout and then enable the WDT
    pub fn new(_instance: Peri<'d, T>) -> Self {
        let wdt = Self {
            _instance,
            _phantom: PhantomData,
        };

        // Set timeout to max so WDT does not immediately reset if user calls `enable` before `set_timeout`
        wdt.set_timeout(0xffffff);
        wdt
    }

    /// Enable WDT
    #[inline(always)]
    pub fn enable(&self) {
        T::reg().ctrl().modify(|_, w| w.wdt_ctrl_en().set_bit());
    }

    /// Disable WDT
    ///
    /// Resets the internal timeout counter to 0
    #[inline(always)]
    pub fn disable(&self) {
        T::reg().ctrl().modify(|_, w| w.wdt_ctrl_en().clear_bit());
    }

    /// Returns true if WDT is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        T::reg().ctrl().read().wdt_ctrl_en().bit_is_set()
    }

    /// Sets 24-bit WDT timeout value
    ///
    /// WDT counter is clocked at 1/4096 of CPU clock frequency
    pub fn set_timeout(&self, timeout: u32) {
        T::reg()
            .ctrl()
            .modify(|_, w| unsafe { w.wdt_ctrl_timeout().bits(timeout) });
    }

    /// Sets WDT timeout value in milliseconds (ms)
    ///
    /// Millisecond precision may not be possible depending on configured main clock frequency
    pub fn set_timeout_ms(&self, timeout_ms: u32) {
        let wdt_clock_freq: u32 = crate::sysinfo::SysInfo::clock_freq() / 4096;
        let ticks_per_ms: u32 = wdt_clock_freq / 1000;
        let timeout = timeout_ms * ticks_per_ms;
        self.set_timeout(timeout);
    }

    /// Returns a locked WDT which prevents illegal access
    ///
    /// The only way to unlock the WDT is via system reset
    #[must_use]
    pub fn lock(self) -> Wdt<'d, T, Locked> {
        T::reg().ctrl().modify(|_, w| w.wdt_ctrl_lock().set_bit());
        Wdt {
            _instance: self._instance,
            _phantom: PhantomData,
        }
    }
}

impl<'d, T: Instance, M: LockMode> Wdt<'d, T, M> {
    /// Resets WDT timeout counter
    pub fn feed(&self) {
        const PASSWORD: u32 = 0x709D1AB3;
        T::reg()
            .reset()
            .write(|w| unsafe { w.wdt_reset().bits(PASSWORD) });
    }

    /// Returns the cause of the last hardware reset
    pub fn reset_cause(&self) -> ResetCause {
        let cause_raw = T::reg().ctrl().read().wdt_ctrl_rcause().bits();
        ResetCause::from(cause_raw)
    }

    /// Forces a hardware reset by feeding an incorrect password to the WDT
    pub fn force_hw_reset(&self) {
        // WDT must be enabled for illegal access resets to trigger
        // It also appears that the WDT must be locked as well for incorrect password to trigger reset
        T::reg()
            .ctrl()
            .modify(|_, w| w.wdt_ctrl_en().set_bit().wdt_ctrl_lock().set_bit());

        // Feed incorrect password to trigger reset
        T::reg()
            .reset()
            .write(|w| unsafe { w.wdt_reset().bits(0xDEADBEEF) });
    }
}

trait SealedLockMode {}

/// WDT lock mode
#[allow(private_bounds)]
pub trait LockMode: SealedLockMode {}

/// WDT is unlocked and all registers can be written to
pub struct Unlocked;
impl SealedLockMode for Unlocked {}
impl LockMode for Unlocked {}

/// WDT is locked and certain registers cannot be written to
///
/// Attempting to circumvent the HAL and writing anyway will trigger reset
pub struct Locked;
impl SealedLockMode for Locked {}
impl LockMode for Locked {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::wdt::RegisterBlock;
}

/// A valid WDT peripheral
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}
impl SealedInstance for WDT {
    fn reg() -> &'static crate::pac::wdt::RegisterBlock {
        unsafe { &*crate::pac::Wdt::ptr() }
    }
}
impl Instance for WDT {}
