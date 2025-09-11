#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk: Clk,
    mem: Mem,
    soc: Soc,
    cache: Cache,
}
impl RegisterBlock {
    #[doc = "0x00 - Processor clock speed in Hz"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x04 - Miscellaneous system configurations"]
    #[inline(always)]
    pub const fn mem(&self) -> &Mem {
        &self.mem
    }
    #[doc = "0x08 - SoC configuration"]
    #[inline(always)]
    pub const fn soc(&self) -> &Soc {
        &self.soc
    }
    #[doc = "0x0c - Cache configuration"]
    #[inline(always)]
    pub const fn cache(&self) -> &Cache {
        &self.cache
    }
}
#[doc = "CLK (rw) register accessor: Processor clock speed in Hz\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "Processor clock speed in Hz"]
pub mod clk;
#[doc = "MEM (r) register accessor: Miscellaneous system configurations\n\nYou can [`read`](crate::Reg::read) this register and get [`mem::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem`] module"]
#[doc(alias = "MEM")]
pub type Mem = crate::Reg<mem::MemSpec>;
#[doc = "Miscellaneous system configurations"]
pub mod mem;
#[doc = "SOC (r) register accessor: SoC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`soc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc`] module"]
#[doc(alias = "SOC")]
pub type Soc = crate::Reg<soc::SocSpec>;
#[doc = "SoC configuration"]
pub mod soc;
#[doc = "CACHE (r) register accessor: Cache configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cache::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache`] module"]
#[doc(alias = "CACHE")]
pub type Cache = crate::Reg<cache::CacheSpec>;
#[doc = "Cache configuration"]
pub mod cache;
