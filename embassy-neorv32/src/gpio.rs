//! General-Purpose Input/Output (GPIO)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::GPIO;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

const PORT_COUNT: usize = 32;
static PORT_WAKERS: [AtomicWaker; PORT_COUNT] = [const { AtomicWaker::new() }; PORT_COUNT];

// TODO: Add doc strings after API stabilized

/// GPIO interrupt handler binding.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let pending = T::reg().irq_pending().read().bits();
        let mut disabled = T::reg().irq_enable().read().bits();

        // Wake and disable every port that has IRQ pending
        for (i, waker) in PORT_WAKERS.iter().enumerate() {
            let port_bit = 1 << i;
            if (pending & port_bit) != 0 {
                waker.wake();
                disabled &= !port_bit;
            }
        }

        // TODO: Maybe not sound, what if interrupt fires at this moment and we clear it below without waking?

        // Clear pending
        T::reg().irq_pending().write(|w| unsafe { w.bits(0) });

        // Disable interrupts for ports that were just pending
        T::reg().irq_enable().write(|w| unsafe { w.bits(disabled) });
    }
}

pub enum IrqType {
    LowLevel,
    HighLevel,
    FallingEdge,
    RisingEdge,
}

pub struct Gpio<'d, T: Instance, M: InputMode> {
    _instance: Peri<'d, T>,
    _phantom: PhantomData<M>,
}

impl<'d, T: Instance> Gpio<'d, T, Blocking> {
    pub fn new_blocking(_instance: Peri<'d, T>) -> Self {
        Self {
            _instance,
            _phantom: PhantomData,
        }
    }

    pub fn new_port<P: PortInstance>(&self, _instance: Peri<'d, P>) -> Port<'d, Blocking> {
        Port::new_blocking(_instance)
    }

    pub fn new_input<P: PortInstance>(&self, _instance: Peri<'d, P>) -> Input<'d, Blocking> {
        Input::new_blocking(_instance)
    }
}

impl<'d, T: Instance> Gpio<'d, T, Async> {
    pub fn new_async(
        _instance: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        // Enable GPIO interrupts
        unsafe { T::Interrupt::enable() };

        Self {
            _instance,
            _phantom: PhantomData,
        }
    }

    pub fn new_port<P: PortInstance>(&self, _instance: Peri<'d, P>) -> Port<'d, Async> {
        Port::new_async(_instance)
    }

    pub fn new_input<P: PortInstance>(&self, _instance: Peri<'d, P>) -> Input<'d, Async> {
        Input::new_async(_instance)
    }
}

impl<'d, T: Instance, M: InputMode> Gpio<'d, T, M> {
    pub fn new_output<P: PortInstance>(&self, _instance: Peri<'d, P>) -> Output<'d> {
        Output::new(_instance)
    }
}

pub struct Port<'d, M: InputMode> {
    portno: u32,
    _phantom: PhantomData<&'d M>,
}

impl<'d> Port<'d, Blocking> {
    fn new_blocking<T: PortInstance>(_instance: Peri<'d, T>) -> Self {
        Self::new_inner(_instance)
    }

    pub fn irq_enable(&mut self, irq_type: IrqType) {
        match irq_type {
            IrqType::LowLevel => {
                self.set_level_trigger();
                self.set_trigger_polarity_low();
            }
            IrqType::HighLevel => {
                self.set_level_trigger();
                self.set_trigger_polarity_high();
            }
            IrqType::FallingEdge => {
                self.set_edge_trigger();
                self.set_trigger_polarity_low();
            }
            IrqType::RisingEdge => {
                self.set_edge_trigger();
                self.set_trigger_polarity_high();
            }
        }

        GPIO::irq_enable(self.portno);
    }

    pub fn irq_disable(&mut self) {
        GPIO::irq_disable(self.portno);
    }
}

impl<'d> Port<'d, Async> {
    fn new_async<T: PortInstance>(_instance: Peri<'d, T>) -> Self {
        Self::new_inner(_instance)
    }

    async fn wait(&mut self) {
        GPIO::irq_enable(self.portno);

        poll_fn(|cx| {
            PORT_WAKERS[self.portno as usize].register(cx.waker());

            // If irq is disabled, we know interrupt actually fired
            if !self.irq_enabled() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    pub async fn wait_for_low(&mut self) {
        self.set_level_trigger();
        self.set_trigger_polarity_low();
        self.wait().await
    }

    pub async fn wait_for_high(&mut self) {
        self.set_level_trigger();
        self.set_trigger_polarity_high();
        self.wait().await
    }

    pub async fn wait_for_falling_edge(&mut self) {
        self.set_edge_trigger();
        self.set_trigger_polarity_low();
        self.wait().await
    }

    pub async fn wait_for_rising_edge(&mut self) {
        self.set_edge_trigger();
        self.set_trigger_polarity_high();
        self.wait().await
    }

    pub async fn wait_for_any_edge(&mut self) {
        if self.is_low() {
            self.wait_for_rising_edge().await
        } else {
            self.wait_for_falling_edge().await
        }
    }
}

impl<'d, M: InputMode> Port<'d, M> {
    fn new_inner<T: PortInstance>(_instance: Peri<'d, T>) -> Self {
        Self {
            portno: T::PORT,
            _phantom: PhantomData,
        }
    }

    fn set_level_trigger(&mut self) {
        GPIO::set_level_trigger(self.portno);
    }

    fn set_edge_trigger(&mut self) {
        GPIO::set_edge_trigger(self.portno);
    }

    fn set_trigger_polarity_low(&mut self) {
        GPIO::set_trigger_polarity_low(self.portno);
    }

    fn set_trigger_polarity_high(&mut self) {
        GPIO::set_trigger_polarity_high(self.portno);
    }

    pub fn split(self) -> (Input<'d, M>, Output<'d>) {
        let input = Input {
            port: Port {
                portno: self.portno,
                _phantom: PhantomData,
            },
        };

        let output = Output {
            port: Port {
                portno: self.portno,
                _phantom: PhantomData,
            },
        };

        (input, output)
    }

    pub fn is_low(&self) -> bool {
        GPIO::is_low(self.portno)
    }

    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high();
        } else {
            self.set_low();
        }
    }

    pub fn set(&mut self, val: bool) {
        if val {
            self.set_high();
        } else {
            self.set_low();
        }
    }

    pub fn set_low(&mut self) {
        GPIO::set_low(self.portno);
    }

    pub fn set_high(&mut self) {
        GPIO::set_high(self.portno);
    }

    pub fn is_set_low(&self) -> bool {
        GPIO::is_set_low(self.portno)
    }

    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    pub fn irq_enabled(&self) -> bool {
        GPIO::irq_enabled(self.portno)
    }
}

pub struct Input<'d, M: InputMode> {
    port: Port<'d, M>,
}

impl<'d> Input<'d, Blocking> {
    fn new_blocking<T: PortInstance>(_instance: Peri<'d, T>) -> Self {
        Self {
            port: Port::new_blocking(_instance),
        }
    }
}

impl<'d> Input<'d, Async> {
    fn new_async<T: PortInstance>(_instance: Peri<'d, T>) -> Self {
        Self {
            port: Port::new_async(_instance),
        }
    }

    pub async fn wait_for_low(&mut self) {
        self.port.wait_for_low().await
    }

    pub async fn wait_for_high(&mut self) {
        self.port.wait_for_high().await
    }

    pub async fn wait_for_falling_edge(&mut self) {
        self.port.wait_for_falling_edge().await
    }

    pub async fn wait_for_rising_edge(&mut self) {
        self.port.wait_for_rising_edge().await
    }

    pub async fn wait_for_any_edge(&mut self) {
        self.port.wait_for_any_edge().await
    }
}

impl<'d, M: InputMode> Input<'d, M> {
    pub fn is_low(&self) -> bool {
        self.port.is_low()
    }

    pub fn is_high(&self) -> bool {
        self.port.is_high()
    }
}

pub struct Output<'d> {
    // Async output doesn't make any sense... right?
    port: Port<'d, Blocking>,
}

impl<'d> Output<'d> {
    fn new<T: PortInstance>(_instance: Peri<'d, T>) -> Self {
        Self {
            port: Port::new_blocking(_instance),
        }
    }

    pub fn toggle(&mut self) {
        self.port.toggle();
    }

    pub fn set(&mut self, val: bool) {
        self.port.set(val);
    }

    pub fn set_low(&mut self) {
        self.port.set_low();
    }

    pub fn set_high(&mut self) {
        self.port.set_high();
    }

    pub fn is_set_low(&self) -> bool {
        self.port.is_set_low()
    }

    pub fn is_set_high(&self) -> bool {
        self.port.is_set_high()
    }
}

// TODO: Impl embedded-hal traits

trait SealedInputMode {}

/// Port Input mode
#[allow(private_bounds)]
pub trait InputMode: SealedInputMode {}

/// Blocking Input
pub struct Blocking;
impl SealedInputMode for Blocking {}
impl InputMode for Blocking {}

/// Async Input
pub struct Async;
impl SealedInputMode for Async {}
impl InputMode for Async {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::gpio::RegisterBlock;

    fn is_low(portno: u32) -> bool;
    fn _is_high(portno: u32) -> bool;
    fn is_set_low(portno: u32) -> bool;
    fn _is_set_high(portno: u32) -> bool;

    fn set_low(portno: u32);
    fn set_high(portno: u32);

    fn set_level_trigger(portno: u32);
    fn set_edge_trigger(portno: u32);
    fn set_trigger_polarity_low(portno: u32);
    fn set_trigger_polarity_high(portno: u32);

    fn irq_enable(portno: u32);
    fn irq_disable(portno: u32);
    fn irq_enabled(portno: u32) -> bool;
    fn _irq_pending(portno: u32) -> bool;
    fn _irq_ack(portno: u32);
}

/// A valid GPIO peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    type Interrupt: Interrupt;
}

// TODO: Probably more efficient to not use CS here and let Port do it to batch together
impl SealedInstance for GPIO {
    fn reg() -> &'static crate::pac::gpio::RegisterBlock {
        unsafe { &*crate::pac::Gpio::ptr() }
    }

    fn is_low(portno: u32) -> bool {
        (Self::reg().port_in().read().bits() & (1 << portno)) == 0
    }

    fn _is_high(portno: u32) -> bool {
        !Self::is_low(portno)
    }

    fn is_set_low(portno: u32) -> bool {
        (Self::reg().port_out().read().bits() & (1 << portno)) == 0
    }

    fn _is_set_high(portno: u32) -> bool {
        !Self::is_set_low(portno)
    }

    fn set_low(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .port_out()
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << portno)) })
        });
    }

    fn set_high(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .port_out()
                .modify(|r, w| unsafe { w.bits(r.bits() | (1 << portno)) })
        });
    }

    fn set_level_trigger(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .irq_type()
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << portno)) })
        });
    }

    fn set_edge_trigger(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .irq_type()
                .modify(|r, w| unsafe { w.bits(r.bits() | (1 << portno)) })
        });
    }

    fn set_trigger_polarity_low(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .irq_polarity()
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << portno)) })
        });
    }

    fn set_trigger_polarity_high(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .irq_polarity()
                .modify(|r, w| unsafe { w.bits(r.bits() | (1 << portno)) })
        });
    }

    fn irq_enable(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .irq_enable()
                .modify(|r, w| unsafe { w.bits(r.bits() | (1 << portno)) })
        });
    }

    fn irq_disable(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .irq_enable()
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << portno)) })
        });
    }

    fn irq_enabled(portno: u32) -> bool {
        (Self::reg().irq_enable().read().bits() & (1 << portno)) != 0
    }

    fn _irq_pending(portno: u32) -> bool {
        (Self::reg().irq_pending().read().bits() & (1 << portno)) != 0
    }

    fn _irq_ack(portno: u32) {
        critical_section::with(|_| {
            Self::reg()
                .irq_pending()
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << portno)) })
        });
    }
}
impl Instance for GPIO {
    type Interrupt = crate::interrupt::typelevel::GPIO;
}

trait SealedPortInstance {}

/// A valid PORT peripheral
#[allow(private_bounds)]
pub trait PortInstance: SealedPortInstance + PeripheralType {
    const PORT: u32;
}

// TODO: Macro this and impl for all 32 ports
impl SealedPortInstance for crate::peripherals::PORT0 {}
impl PortInstance for crate::peripherals::PORT0 {
    const PORT: u32 = 0;
}
