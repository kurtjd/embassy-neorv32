// TODO: Really rough draft, definitely needs revisiting (and doc strings)
//! Direct Memory Access (DMA)
use crate::peripherals::DMA;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::Ordering;
use core::sync::atomic::fence;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use pac::interrupt::CoreInterrupt;

static DMA_WAKER: AtomicWaker = AtomicWaker::new();

#[riscv_rt::core_interrupt(CoreInterrupt::DMA)]
fn dma_handler() {
    // If we ack IRQ here, that clears DONE bit and no way to check in waker
    // So instead disable to prevent storm and ACK and re-enable in waker
    // Could use atomic flag? Not sure what the best approach is...
    riscv::interrupt::disable_interrupt(CoreInterrupt::DMA);
    DMA_WAKER.wake();
}

pub enum Error {
    BusError,
}

pub enum DataConfig {
    ConstantByte,
    ConstantWord,
    IncrementingByte,
    IncrementingWord,
}

impl From<DataConfig> for u32 {
    fn from(config: DataConfig) -> Self {
        match config {
            DataConfig::ConstantByte => 0b00,
            DataConfig::ConstantWord => 0b01,
            DataConfig::IncrementingByte => 0b10,
            DataConfig::IncrementingWord => 0b11,
        }
    }
}

pub struct TransferConfig {
    num_elems: u32,
    swap_byte_order: bool,
    src_config: DataConfig,
    dst_config: DataConfig,
}

impl TransferConfig {
    pub fn new(
        num_elems: u32,
        swap_byte_order: bool,
        src_config: DataConfig,
        dst_config: DataConfig,
    ) -> Self {
        assert!(num_elems > 0 && num_elems < 0xff_ffff);
        Self {
            num_elems,
            swap_byte_order,
            src_config,
            dst_config,
        }
    }
}

impl From<TransferConfig> for u32 {
    fn from(config: TransferConfig) -> Self {
        (u32::from(config.dst_config) << 30)
            | (u32::from(config.src_config) << 28)
            | ((config.swap_byte_order as u32) << 27)
            | (config.num_elems & 0xff_ffff)
    }
}

pub enum Descriptor {
    BaseAddress(u32),
    Config(TransferConfig),
}

/// Direct Memory Access (DMA) Driver
pub struct Dma<'d, M: IoMode> {
    reg: &'static pac::dma::RegisterBlock,
    _phantom: PhantomData<(&'d (), M)>,
}

impl<'d> Dma<'d, Blocking> {
    /// Returns a new instance of a blocking DMA and enables it
    pub fn new_blocking<T: Instance>(_instance: Peri<'d, T>) -> Self {
        Self::new_inner(_instance)
    }
}

impl<'d> Dma<'d, Async> {
    /// Returns a new instance of an async DMA and enables it
    pub fn new_async<T: Instance>(_instance: Peri<'d, T>) -> Self {
        Self::new_inner(_instance)
    }

    pub async fn transfer(&self, src: &[u8], dst: &mut [u8]) -> Result<(), Error> {
        assert!(src.len() == dst.len());

        // These fences are needed if dCache is enabled to force CPU to flush cache and see DMA writes
        fence(Ordering::Release);
        self.start_transfer(src.as_ptr(), dst.as_mut_ptr(), src.len() as u32);
        poll_fn(|cx| {
            DMA_WAKER.register(cx.waker());
            let p = if self.transfer_complete() {
                Poll::Ready(Ok(()))
            } else if self.bus_error() {
                Poll::Ready(Err(Error::BusError))
            } else {
                unsafe {
                    riscv::interrupt::enable_interrupt(CoreInterrupt::DMA);
                }
                Poll::Pending
            };

            if p.is_ready() {
                fence(Ordering::Acquire);
                unsafe {
                    Self::irq_ack();
                }
            }

            p
        })
        .await
    }
}

impl<'d, M: IoMode> Dma<'d, M> {
    fn new_inner<T: Instance>(_instance: Peri<'d, T>) -> Self {
        let dma = Self {
            reg: T::reg(),
            _phantom: PhantomData,
        };

        dma.enable();
        dma
    }

    pub fn enable(&self) {
        self.reg.ctrl().modify(|_, w| w.dma_ctrl_en().set_bit());
    }

    pub fn disable(&self) {
        self.reg.ctrl().modify(|_, w| w.dma_ctrl_en().clear_bit());
    }

    pub fn enabled(&self) -> bool {
        self.reg.ctrl().read().dma_ctrl_en().bit_is_set()
    }

    pub fn start(&self) {
        self.reg.ctrl().modify(|_, w| w.dma_ctrl_start().set_bit());
    }

    pub fn fifo_empty(&self) -> bool {
        self.reg.ctrl().read().dma_ctrl_dempty().bit_is_set()
    }

    pub fn fifo_full(&self) -> bool {
        self.reg.ctrl().read().dma_ctrl_dfull().bit_is_set()
    }

    pub fn bus_error(&self) -> bool {
        self.reg.ctrl().read().dma_ctrl_error().bit_is_set()
    }

    pub fn transfer_complete(&self) -> bool {
        self.reg.ctrl().read().dma_ctrl_done().bit_is_set()
    }

    pub fn busy(&self) -> bool {
        self.reg.ctrl().read().dma_ctrl_busy().bit_is_set()
    }

    pub fn write_descriptor(&self, desc: Descriptor) {
        let raw = match desc {
            Descriptor::BaseAddress(addr) => addr,
            Descriptor::Config(config) => config.into(),
        };
        self.reg.desc().write(|w| unsafe { w.bits(raw) });
    }

    pub fn start_transfer_raw(&self, src_addr: u32, dst_addr: u32, config: TransferConfig) {
        self.write_descriptor(Descriptor::BaseAddress(src_addr));
        self.write_descriptor(Descriptor::BaseAddress(dst_addr));
        self.write_descriptor(Descriptor::Config(config));

        self.reg.ctrl().modify(|_, w| w.dma_ctrl_start().set_bit());
    }

    pub fn start_transfer(&self, src_addr: *const u8, dst_addr: *mut u8, len: u32) {
        let config = TransferConfig::new(
            len,
            false,
            DataConfig::IncrementingByte,
            DataConfig::IncrementingByte,
        );

        self.start_transfer_raw(src_addr as u32, dst_addr as u32, config);
    }

    /// Blocking transfer not really useful in practice, just for testing at the moment
    pub fn blocking_transfer(&self, src: &[u8], dst: &mut [u8]) -> Result<(), Error> {
        assert!(src.len() == dst.len());

        fence(Ordering::Release);
        self.start_transfer(src.as_ptr(), dst.as_mut_ptr(), src.len() as u32);
        while !self.transfer_complete() && !self.bus_error() {}
        fence(Ordering::Acquire);

        let res = if self.bus_error() {
            Err(Error::BusError)
        } else {
            Ok(())
        };

        unsafe {
            Self::irq_ack();
        }

        res
    }

    pub unsafe fn irq_ack() {
        let reg = unsafe { &*pac::Dma::ptr() };
        reg.ctrl().modify(|_, w| w.dma_ctrl_ack().set_bit());
    }
}

/// DMA IO mode
#[allow(private_bounds)]
pub trait IoMode: crate::Sealed {}

/// Blocking DMA
pub struct Blocking;
impl crate::Sealed for Blocking {}
impl IoMode for Blocking {}

/// Async DMA
pub struct Async;
impl crate::Sealed for Async {}
impl IoMode for Async {}

/// A valid DMA peripheral
#[allow(private_bounds)]
pub trait Instance: crate::Sealed + PeripheralType {
    fn reg() -> &'static pac::dma::RegisterBlock;
}

impl crate::Sealed for DMA {}
impl Instance for DMA {
    fn reg() -> &'static pac::dma::RegisterBlock {
        unsafe { &*pac::Dma::ptr() }
    }
}
