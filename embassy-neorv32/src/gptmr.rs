//! General Purpose Timer (GPTMR)
use crate::peripherals::GPTMR;
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};

/// GPTMR Prescaler.
pub enum Prescaler {
    /// Prescaler of 2
    Psc2,
    /// Prescaler of 4
    Psc4,
    /// Prescaler of 8
    Psc8,
    /// Prescaler of 64
    Psc64,
    /// Prescaler of 128
    Psc128,
    /// Prescaler of 1024
    Psc1024,
    /// Prescaler of 2048
    Psc2048,
    /// Prescaler of 4096
    Psc4096,
}

impl From<Prescaler> for u8 {
    fn from(psc: Prescaler) -> Self {
        match psc {
            Prescaler::Psc2 => 0b000,
            Prescaler::Psc4 => 0b001,
            Prescaler::Psc8 => 0b010,
            Prescaler::Psc64 => 0b011,
            Prescaler::Psc128 => 0b100,
            Prescaler::Psc1024 => 0b101,
            Prescaler::Psc2048 => 0b110,
            Prescaler::Psc4096 => 0b111,
        }
    }
}

impl From<u8> for Prescaler {
    fn from(value: u8) -> Self {
        match value {
            0b000 => Self::Psc2,
            0b001 => Self::Psc4,
            0b010 => Self::Psc8,
            0b011 => Self::Psc64,
            0b100 => Self::Psc128,
            0b101 => Self::Psc1024,
            0b110 => Self::Psc2048,
            0b111 => Self::Psc4096,

            // We use `unreachable!` instead of returning an `Unknown` variant
            // because `Prescaler` is also an input and `Unknown` is not a valid input
            //
            // Don't like introducing a panic path though
            _ => unreachable!(),
        }
    }
}

/// General Purpose Timer (GPTMR) Driver.
pub struct Gptmr<'d> {
    reg: &'static crate::pac::gptmr::RegisterBlock,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Gptmr<'d> {
    /// Create a new instance of a GPTMR driver in a disabled state with the given prescaler.
    ///
    /// Once enabled, GPTMR will count up at a rate of `(CPU_CLK / prescaler)` until it hits
    /// the threshold programmed via [Self::set_threshold], at which point it will reset to 0
    /// and trigger an interrupt. This irq must then be manually cleared via [Self::irq_clear].
    ///
    /// **Note**: Async users should prefer `embassy_time::Timer` (via `time-driver` feature).
    ///
    /// This mainly serves as a thin wrapper HAL around the GPTMR peripheral for those who can't
    /// or won't use the `time-driver` feature, or otherwise need this for some other specific use-case.
    pub fn new<T: Instance>(_instance: Peri<'d, T>, psc: Prescaler) -> Self {
        let mut gptmr = Self {
            reg: T::reg(),
            _phantom: PhantomData,
        };

        gptmr.set_prescaler(psc);
        gptmr
    }

    /// Enable the GPTMR.
    pub fn enable(&mut self) {
        self.reg.ctrl().modify(|_, w| w.gptmr_ctrl_en().set_bit());
    }

    /// Disable the GPTMR.
    pub fn disable(&mut self) {
        self.reg.ctrl().modify(|_, w| w.gptmr_ctrl_en().clear_bit());
    }

    /// Returns true if the GPTMR is enabled.
    pub fn enabled(&self) -> bool {
        self.reg.ctrl().read().gptmr_ctrl_en().bit_is_set()
    }

    /// Set the GPTMR prescaler.
    pub fn set_prescaler(&mut self, psc: Prescaler) {
        // SAFETY: The prescaler is valid
        self.reg
            .ctrl()
            .modify(|_, w| unsafe { w.gptmr_ctrl_prsc().bits(psc.into()) });
    }

    /// Returns the GPTMR prescaler.
    pub fn prescaler(&self) -> Prescaler {
        self.reg.ctrl().read().gptmr_ctrl_prsc().bits().into()
    }

    /// Returns true if the GPTMR interrupt is pending.
    pub fn irq_pending(&self) -> bool {
        self.reg.ctrl().read().gptmr_ctrl_irq_pnd().bit_is_set()
    }

    /// Set the GPTMR threshold (in ticks) before interrupt is triggered.
    ///
    /// GPTMR counter will automatically reset to zero after reaching threshold.
    ///
    /// However, interrupt must be acknowleged via [Self::irq_clear].
    pub fn set_threshold(&mut self, ticks: u32) {
        // SAFETY: Any threshold value is valid
        self.reg.thres().write(|w| unsafe { w.bits(ticks) });
    }

    /// Returns the GPTMR threshold (in ticks).
    pub fn threshold(&self) -> u32 {
        self.reg.thres().read().bits()
    }

    /// Returns the current GPTMR counter (in ticks).
    pub fn count(&self) -> u32 {
        self.reg.count().read().bits()
    }

    /// Clears a pending GPTMR interrupt.
    ///
    /// This does not take `&self` to allow easy use from interrupt handler.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring this does not cause unexpected behavior.
    pub unsafe fn irq_clear() {
        GPTMR::reg()
            .ctrl()
            .modify(|_, w| w.gptmr_ctrl_irq_clr().set_bit());
    }
}

impl<'d> Drop for Gptmr<'d> {
    fn drop(&mut self) {
        self.disable();
    }
}

trait SealedInstance {
    fn reg() -> &'static crate::pac::gptmr::RegisterBlock;
}

/// A valid GPTMR peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}
impl SealedInstance for GPTMR {
    fn reg() -> &'static crate::pac::gptmr::RegisterBlock {
        unsafe { &*crate::pac::Gptmr::ptr() }
    }
}
impl Instance for GPTMR {}
