#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    data24: Data24,
    data32: Data32,
    strobe: Strobe,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Send 24-bit data"]
    #[inline(always)]
    pub const fn data24(&self) -> &Data24 {
        &self.data24
    }
    #[doc = "0x08 - Send 32-bit data"]
    #[inline(always)]
    pub const fn data32(&self) -> &Data32 {
        &self.data32
    }
    #[doc = "0x0c - Write any value to send STROBE command"]
    #[inline(always)]
    pub const fn strobe(&self) -> &Strobe {
        &self.strobe
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "DATA24 (w) register accessor: Send 24-bit data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data24::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data24`] module"]
#[doc(alias = "DATA24")]
pub type Data24 = crate::Reg<data24::Data24Spec>;
#[doc = "Send 24-bit data"]
pub mod data24;
#[doc = "DATA32 (w) register accessor: Send 32-bit data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data32`] module"]
#[doc(alias = "DATA32")]
pub type Data32 = crate::Reg<data32::Data32Spec>;
#[doc = "Send 32-bit data"]
pub mod data32;
#[doc = "STROBE (w) register accessor: Write any value to send STROBE command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`strobe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strobe`] module"]
#[doc(alias = "STROBE")]
pub type Strobe = crate::Reg<strobe::StrobeSpec>;
#[doc = "Write any value to send STROBE command"]
pub mod strobe;
