//! Watchdog Timer (WDT)
use core::marker::PhantomData;

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

pub struct Wdt<T: Instance, M: Mode> {
    _instance: PhantomData<T>,
    _mode: PhantomData<M>,
}

impl<T: Instance> Wdt<T, Unlocked> {
    /// Returns a new unlocked WDT with timeout set to 24-bit max
    ///
    /// Caller should configure timeout and then enable the WDT
    pub fn new(_instance: T) -> Self {
        let wdt = Self {
            _instance: PhantomData,
            _mode: PhantomData,
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
        const WDT_CLOCK_FREQ: u32 = crate::CPU_CLK_FREQ / 4096;
        const TICKS_PER_MS: u32 = WDT_CLOCK_FREQ / 1000;
        let timeout = timeout_ms * TICKS_PER_MS;
        self.set_timeout(timeout);
    }

    /// Returns a locked WDT which prevents illegal access
    ///
    /// The only way to unlock the WDT is via system reset
    #[must_use]
    pub fn lock(self) -> Wdt<T, Locked> {
        T::reg().ctrl().modify(|_, w| w.wdt_ctrl_lock().set_bit());
        Wdt {
            _instance: PhantomData,
            _mode: PhantomData,
        }
    }
}

impl<T: Instance, M: Mode> Wdt<T, M> {
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

/// WDT operating mode (locked or unlocked)
#[allow(private_bounds)]
pub trait Mode: crate::Sealed {}

/// WDT is unlocked and all registers can be written to
pub struct Unlocked;
impl crate::Sealed for Unlocked {}
impl Mode for Unlocked {}

/// WDT is locked and certain registers cannot be written to
///
/// Attempting to circumvent the HAL and writing anyway will trigger reset
pub struct Locked;
impl crate::Sealed for Locked {}
impl Mode for Locked {}

/// A valid WDT peripheral
#[allow(private_bounds)]
pub trait Instance: crate::Sealed {
    fn reg() -> &'static pac::wdt::RegisterBlock;
}

impl crate::Sealed for pac::Wdt {}
impl Instance for pac::Wdt {
    fn reg() -> &'static pac::wdt::RegisterBlock {
        unsafe { &*pac::Wdt::ptr() }
    }
}
