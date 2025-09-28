//! General Purpose Timer (GPTMR)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::GPTMR;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

// GPTMR interrupt handler binding
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        unsafe { Gptmr::<GPTMR, Async>::irq_clear() };
        T::Interrupt::disable();
        T::waker().wake();
    }
}

/// GPTMR Prescaler
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

/// General Purpose Timer (GPTMR) Driver
pub struct Gptmr<'d, T: Instance, M: WaitMode> {
    _instance: Peri<'d, T>,
    _phantom: PhantomData<M>,
}

impl<'d, T: Instance> Gptmr<'d, T, Blocking> {
    /// Returns a new blocking GPTMR with given prescaler
    pub fn new_blocking(_instance: Peri<'d, T>, psc: Prescaler) -> Self {
        Self::new_inner(_instance, psc)
    }
}

impl<'d, T: Instance> Gptmr<'d, T, Async> {
    /// Returns a new async GPTMR with given prescaler
    pub fn new_async(
        _instance: Peri<'d, T>,
        psc: Prescaler,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Self::new_inner(_instance, psc)
    }

    pub async fn wait(&mut self) {
        unsafe {
            T::Interrupt::enable();
        }

        poll_fn(|cx| {
            T::waker().register(cx.waker());

            // If interrupt is disabled, we know IRQ triggered
            // We have to ack IRQ in handler, which clears pending bit leaving us nothing to check
            // So we rely on this
            if !T::Interrupt::is_enabled() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl<'d, T: Instance, M: WaitMode> Gptmr<'d, T, M> {
    fn new_inner(_instance: Peri<'d, T>, psc: Prescaler) -> Self {
        let mut gptmr = Self {
            _instance,
            _phantom: PhantomData,
        };

        gptmr.set_prescaler(psc);
        gptmr
    }

    /// Set the GPTMR prescaler
    #[inline(always)]
    pub fn set_prescaler(&mut self, psc: Prescaler) {
        T::reg()
            .ctrl()
            .modify(|_, w| unsafe { w.gptmr_ctrl_prsc().bits(psc.into()) });
    }

    /// Returns the GPTMR prescaler
    pub fn prescaler(&self) -> Prescaler {
        let psc = T::reg().ctrl().read().gptmr_ctrl_prsc().bits();
        psc.into()
    }

    /// Enable the GPTMR
    #[inline(always)]
    pub fn enable(&mut self) {
        T::reg().ctrl().modify(|_, w| w.gptmr_ctrl_en().set_bit());
    }

    /// Disable the GPTMR
    #[inline(always)]
    pub fn disable(&mut self) {
        T::reg().ctrl().modify(|_, w| w.gptmr_ctrl_en().clear_bit());
    }

    /// Returns true if the GPTMR is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        T::reg().ctrl().read().gptmr_ctrl_en().bit_is_set()
    }

    /// Returns true if the GPTMR interrupt is pending
    #[inline(always)]
    pub fn irq_pending(&self) -> bool {
        T::reg().ctrl().read().gptmr_ctrl_irq_pnd().bit_is_set()
    }

    /// Returns the current GPTMR counter (in ticks)
    #[inline(always)]
    pub fn count(&self) -> u32 {
        T::reg().count().read().bits()
    }

    /// Returns the GPTMR threshold (in ticks)
    #[inline(always)]
    pub fn threshold(&self) -> u32 {
        T::reg().thres().read().bits()
    }

    /// Set the GPTMR threshold (in ticks) before interrupt is triggered
    ///
    /// GPTMR counter will automatically reset to zero after reaching threshold
    ///
    /// However, interrupt must be acknowleged via [Self::irq_clear]
    #[inline(always)]
    pub fn set_threshold(&mut self, threshold_ticks: u32) {
        T::reg()
            .thres()
            .write(|w| unsafe { w.bits(threshold_ticks) });
    }

    /// Wait for GPTMR counter to reach its threshold, blocking in the meantime
    pub fn blocking_wait(&mut self) {
        while !self.irq_pending() {}
        unsafe { Self::irq_clear() };
    }

    /// Clears a pending GPTMR interrupt
    ///
    /// This does not take `&self` to allow easy use from interrupt handler
    ///
    /// # Safety
    /// The caller is responsible for ensuring this does not cause unexpected behavior
    #[inline(always)]
    pub unsafe fn irq_clear() {
        // TODO: Investigate why this needs calling in a loop
        // Calling clear then waiting until irq is no longer pending just seems to hang
        while T::reg().ctrl().read().gptmr_ctrl_irq_pnd().bit_is_set() {
            T::reg()
                .ctrl()
                .modify(|_, w| w.gptmr_ctrl_irq_clr().set_bit());
        }
    }
}

trait SealedWaitMode {}

/// GPTMR Wait mode
#[allow(private_bounds)]
pub trait WaitMode: SealedWaitMode {}

/// Blocking TRNG
pub struct Blocking;
impl SealedWaitMode for Blocking {}
impl WaitMode for Blocking {}

/// Async TRNG
pub struct Async;
impl SealedWaitMode for Async {}
impl WaitMode for Async {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::gptmr::RegisterBlock;
    fn waker() -> &'static AtomicWaker;
}

/// A valid GPTMR peripheral
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    type Interrupt: Interrupt;
}
impl SealedInstance for GPTMR {
    fn reg() -> &'static crate::pac::gptmr::RegisterBlock {
        unsafe { &*crate::pac::Gptmr::ptr() }
    }

    fn waker() -> &'static AtomicWaker {
        static WAKER: AtomicWaker = AtomicWaker::new();
        &WAKER
    }
}
impl Instance for GPTMR {
    type Interrupt = crate::interrupt::typelevel::GPTMR;
}
