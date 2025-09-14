//! Extremely basic and very likely not sound
//! Just a simple PoC for now

// TODO: Patch SVD so DATA reg is split into fields

pub trait Instance {
    fn reg() -> &'static neorv32_pac::uart0::RegisterBlock;
}

impl Instance for neorv32_pac::Uart0 {
    fn reg() -> &'static neorv32_pac::uart0::RegisterBlock {
        unsafe { &*neorv32_pac::Uart0::ptr() }
    }
}

pub struct Uart {
    reg: &'static neorv32_pac::uart0::RegisterBlock,
}
unsafe impl Send for Uart {}

impl Uart {
    pub fn new<T: Instance>(_instance: T) -> Self {
        let uart = Self { reg: T::reg() };
        uart.reg.ctrl().write(|w| unsafe { w.bits(0) });
        uart.reg
            .ctrl()
            .write(|w| w.uart_ctrl_en().set_bit().uart_ctrl_sim_mode().set_bit());

        uart
    }

    pub fn write_byte(&self, byte: u8) {
        while self.reg.ctrl().read().uart_ctrl_tx_nfull().bit_is_clear() {}
        self.reg.data().write(|w| unsafe { w.bits(byte as u32) });
    }

    pub fn write(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }
}
