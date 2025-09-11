#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    port_in: PortIn,
    port_out: PortOut,
    _reserved2: [u8; 0x08],
    irq_type: IrqType,
    irq_polarity: IrqPolarity,
    irq_enable: IrqEnable,
    irq_pending: IrqPending,
}
impl RegisterBlock {
    #[doc = "0x00 - Parallel input port"]
    #[inline(always)]
    pub const fn port_in(&self) -> &PortIn {
        &self.port_in
    }
    #[doc = "0x04 - Parallel output port"]
    #[inline(always)]
    pub const fn port_out(&self) -> &PortOut {
        &self.port_out
    }
    #[doc = "0x10 - Interrupt trigger type (level/edge) for each input pin"]
    #[inline(always)]
    pub const fn irq_type(&self) -> &IrqType {
        &self.irq_type
    }
    #[doc = "0x14 - Interrupt trigger polarity (rising/falling or high/low) for each input pin"]
    #[inline(always)]
    pub const fn irq_polarity(&self) -> &IrqPolarity {
        &self.irq_polarity
    }
    #[doc = "0x18 - Interrupt enable for each input pin"]
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IrqEnable {
        &self.irq_enable
    }
    #[doc = "0x1c - Interrupt pending for each input pin; cleared by writing zero"]
    #[inline(always)]
    pub const fn irq_pending(&self) -> &IrqPending {
        &self.irq_pending
    }
}
#[doc = "PORT_IN (r) register accessor: Parallel input port\n\nYou can [`read`](crate::Reg::read) this register and get [`port_in::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_in`] module"]
#[doc(alias = "PORT_IN")]
pub type PortIn = crate::Reg<port_in::PortInSpec>;
#[doc = "Parallel input port"]
pub mod port_in;
#[doc = "PORT_OUT (rw) register accessor: Parallel output port\n\nYou can [`read`](crate::Reg::read) this register and get [`port_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_out`] module"]
#[doc(alias = "PORT_OUT")]
pub type PortOut = crate::Reg<port_out::PortOutSpec>;
#[doc = "Parallel output port"]
pub mod port_out;
#[doc = "IRQ_TYPE (rw) register accessor: Interrupt trigger type (level/edge) for each input pin\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_type`] module"]
#[doc(alias = "IRQ_TYPE")]
pub type IrqType = crate::Reg<irq_type::IrqTypeSpec>;
#[doc = "Interrupt trigger type (level/edge) for each input pin"]
pub mod irq_type;
#[doc = "IRQ_POLARITY (rw) register accessor: Interrupt trigger polarity (rising/falling or high/low) for each input pin\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_polarity`] module"]
#[doc(alias = "IRQ_POLARITY")]
pub type IrqPolarity = crate::Reg<irq_polarity::IrqPolaritySpec>;
#[doc = "Interrupt trigger polarity (rising/falling or high/low) for each input pin"]
pub mod irq_polarity;
#[doc = "IRQ_ENABLE (rw) register accessor: Interrupt enable for each input pin\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enable`] module"]
#[doc(alias = "IRQ_ENABLE")]
pub type IrqEnable = crate::Reg<irq_enable::IrqEnableSpec>;
#[doc = "Interrupt enable for each input pin"]
pub mod irq_enable;
#[doc = "IRQ_PENDING (rw) register accessor: Interrupt pending for each input pin; cleared by writing zero\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_pending`] module"]
#[doc(alias = "IRQ_PENDING")]
pub type IrqPending = crate::Reg<irq_pending::IrqPendingSpec>;
#[doc = "Interrupt pending for each input pin; cleared by writing zero"]
pub mod irq_pending;
