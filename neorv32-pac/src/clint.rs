#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mswi0: Mswi0,
    _reserved1: [u8; 0x3ffc],
    mtimecmp0_low: Mtimecmp0Low,
    mtimecmp0_hi: Mtimecmp0Hi,
    _reserved3: [u8; 0x7ff0],
    mtime_low: MtimeLow,
    mtime_hi: MtimeHi,
}
impl RegisterBlock {
    #[doc = "0x00 - Machine software interrupt; hart 0"]
    #[inline(always)]
    pub const fn mswi0(&self) -> &Mswi0 {
        &self.mswi0
    }
    #[doc = "0x4000 - Machine timer compare low word; hart0"]
    #[inline(always)]
    pub const fn mtimecmp0_low(&self) -> &Mtimecmp0Low {
        &self.mtimecmp0_low
    }
    #[doc = "0x4004 - Machine timer compare low word; hart0"]
    #[inline(always)]
    pub const fn mtimecmp0_hi(&self) -> &Mtimecmp0Hi {
        &self.mtimecmp0_hi
    }
    #[doc = "0xbff8 - Machine timer low word"]
    #[inline(always)]
    pub const fn mtime_low(&self) -> &MtimeLow {
        &self.mtime_low
    }
    #[doc = "0xbffc - Machine timer high word"]
    #[inline(always)]
    pub const fn mtime_hi(&self) -> &MtimeHi {
        &self.mtime_hi
    }
}
#[doc = "MSWI0 (rw) register accessor: Machine software interrupt; hart 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mswi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mswi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mswi0`] module"]
#[doc(alias = "MSWI0")]
pub type Mswi0 = crate::Reg<mswi0::Mswi0Spec>;
#[doc = "Machine software interrupt; hart 0"]
pub mod mswi0;
#[doc = "MTIMECMP0_LOW (rw) register accessor: Machine timer compare low word; hart0\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp0_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp0_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp0_low`] module"]
#[doc(alias = "MTIMECMP0_LOW")]
pub type Mtimecmp0Low = crate::Reg<mtimecmp0_low::Mtimecmp0LowSpec>;
#[doc = "Machine timer compare low word; hart0"]
pub mod mtimecmp0_low;
#[doc = "MTIMECMP0_HI (rw) register accessor: Machine timer compare low word; hart0\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp0_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp0_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp0_hi`] module"]
#[doc(alias = "MTIMECMP0_HI")]
pub type Mtimecmp0Hi = crate::Reg<mtimecmp0_hi::Mtimecmp0HiSpec>;
#[doc = "Machine timer compare low word; hart0"]
pub mod mtimecmp0_hi;
#[doc = "MTIME_LOW (rw) register accessor: Machine timer low word\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_low`] module"]
#[doc(alias = "MTIME_LOW")]
pub type MtimeLow = crate::Reg<mtime_low::MtimeLowSpec>;
#[doc = "Machine timer low word"]
pub mod mtime_low;
#[doc = "MTIME_HI (rw) register accessor: Machine timer high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_hi`] module"]
#[doc(alias = "MTIME_HI")]
pub type MtimeHi = crate::Reg<mtime_hi::MtimeHiSpec>;
#[doc = "Machine timer high word"]
pub mod mtime_hi;
