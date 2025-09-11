#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg0: Reg0,
    reg1: Reg1,
    reg2: Reg2,
    reg3: Reg3,
    reg4: Reg4,
    reg5: Reg5,
    reg6: Reg6,
    reg7: Reg7,
    reg8: Reg8,
    reg9: Reg9,
    reg10: Reg10,
    reg11: Reg11,
    reg12: Reg12,
    reg13: Reg13,
    reg14: Reg14,
    reg15: Reg15,
    reg16: Reg16,
    reg17: Reg17,
    reg18: Reg18,
    reg19: Reg19,
    reg20: Reg20,
    reg21: Reg21,
    reg22: Reg22,
    reg23: Reg23,
    reg24: Reg24,
    reg25: Reg25,
    reg26: Reg26,
    reg27: Reg27,
    reg28: Reg28,
    reg29: Reg29,
    reg30: Reg30,
    reg31: Reg31,
    reg32: Reg32,
    reg33: Reg33,
    reg34: Reg34,
    reg35: Reg35,
    reg36: Reg36,
    reg37: Reg37,
    reg38: Reg38,
    reg39: Reg39,
    reg40: Reg40,
    reg41: Reg41,
    reg42: Reg42,
    reg43: Reg43,
    reg44: Reg44,
    reg45: Reg45,
    reg46: Reg46,
    reg47: Reg47,
    reg48: Reg48,
    reg49: Reg49,
    reg50: Reg50,
    reg51: Reg51,
    reg52: Reg52,
    reg53: Reg53,
    reg54: Reg54,
    reg55: Reg55,
    reg56: Reg56,
    reg57: Reg57,
    reg58: Reg58,
    reg59: Reg59,
    reg60: Reg60,
    reg61: Reg61,
    reg62: Reg62,
    reg63: Reg63,
}
impl RegisterBlock {
    #[doc = "0x00 - User-defined"]
    #[inline(always)]
    pub const fn reg0(&self) -> &Reg0 {
        &self.reg0
    }
    #[doc = "0x04 - User-defined"]
    #[inline(always)]
    pub const fn reg1(&self) -> &Reg1 {
        &self.reg1
    }
    #[doc = "0x08 - User-defined"]
    #[inline(always)]
    pub const fn reg2(&self) -> &Reg2 {
        &self.reg2
    }
    #[doc = "0x0c - User-defined"]
    #[inline(always)]
    pub const fn reg3(&self) -> &Reg3 {
        &self.reg3
    }
    #[doc = "0x10 - User-defined"]
    #[inline(always)]
    pub const fn reg4(&self) -> &Reg4 {
        &self.reg4
    }
    #[doc = "0x14 - User-defined"]
    #[inline(always)]
    pub const fn reg5(&self) -> &Reg5 {
        &self.reg5
    }
    #[doc = "0x18 - User-defined"]
    #[inline(always)]
    pub const fn reg6(&self) -> &Reg6 {
        &self.reg6
    }
    #[doc = "0x1c - User-defined"]
    #[inline(always)]
    pub const fn reg7(&self) -> &Reg7 {
        &self.reg7
    }
    #[doc = "0x20 - User-defined"]
    #[inline(always)]
    pub const fn reg8(&self) -> &Reg8 {
        &self.reg8
    }
    #[doc = "0x24 - User-defined"]
    #[inline(always)]
    pub const fn reg9(&self) -> &Reg9 {
        &self.reg9
    }
    #[doc = "0x28 - User-defined"]
    #[inline(always)]
    pub const fn reg10(&self) -> &Reg10 {
        &self.reg10
    }
    #[doc = "0x2c - User-defined"]
    #[inline(always)]
    pub const fn reg11(&self) -> &Reg11 {
        &self.reg11
    }
    #[doc = "0x30 - User-defined"]
    #[inline(always)]
    pub const fn reg12(&self) -> &Reg12 {
        &self.reg12
    }
    #[doc = "0x34 - User-defined"]
    #[inline(always)]
    pub const fn reg13(&self) -> &Reg13 {
        &self.reg13
    }
    #[doc = "0x38 - User-defined"]
    #[inline(always)]
    pub const fn reg14(&self) -> &Reg14 {
        &self.reg14
    }
    #[doc = "0x3c - User-defined"]
    #[inline(always)]
    pub const fn reg15(&self) -> &Reg15 {
        &self.reg15
    }
    #[doc = "0x40 - User-defined"]
    #[inline(always)]
    pub const fn reg16(&self) -> &Reg16 {
        &self.reg16
    }
    #[doc = "0x44 - User-defined"]
    #[inline(always)]
    pub const fn reg17(&self) -> &Reg17 {
        &self.reg17
    }
    #[doc = "0x48 - User-defined"]
    #[inline(always)]
    pub const fn reg18(&self) -> &Reg18 {
        &self.reg18
    }
    #[doc = "0x4c - User-defined"]
    #[inline(always)]
    pub const fn reg19(&self) -> &Reg19 {
        &self.reg19
    }
    #[doc = "0x50 - User-defined"]
    #[inline(always)]
    pub const fn reg20(&self) -> &Reg20 {
        &self.reg20
    }
    #[doc = "0x54 - User-defined"]
    #[inline(always)]
    pub const fn reg21(&self) -> &Reg21 {
        &self.reg21
    }
    #[doc = "0x58 - User-defined"]
    #[inline(always)]
    pub const fn reg22(&self) -> &Reg22 {
        &self.reg22
    }
    #[doc = "0x5c - User-defined"]
    #[inline(always)]
    pub const fn reg23(&self) -> &Reg23 {
        &self.reg23
    }
    #[doc = "0x60 - User-defined"]
    #[inline(always)]
    pub const fn reg24(&self) -> &Reg24 {
        &self.reg24
    }
    #[doc = "0x64 - User-defined"]
    #[inline(always)]
    pub const fn reg25(&self) -> &Reg25 {
        &self.reg25
    }
    #[doc = "0x68 - User-defined"]
    #[inline(always)]
    pub const fn reg26(&self) -> &Reg26 {
        &self.reg26
    }
    #[doc = "0x6c - User-defined"]
    #[inline(always)]
    pub const fn reg27(&self) -> &Reg27 {
        &self.reg27
    }
    #[doc = "0x70 - User-defined"]
    #[inline(always)]
    pub const fn reg28(&self) -> &Reg28 {
        &self.reg28
    }
    #[doc = "0x74 - User-defined"]
    #[inline(always)]
    pub const fn reg29(&self) -> &Reg29 {
        &self.reg29
    }
    #[doc = "0x78 - User-defined"]
    #[inline(always)]
    pub const fn reg30(&self) -> &Reg30 {
        &self.reg30
    }
    #[doc = "0x7c - User-defined"]
    #[inline(always)]
    pub const fn reg31(&self) -> &Reg31 {
        &self.reg31
    }
    #[doc = "0x80 - User-defined"]
    #[inline(always)]
    pub const fn reg32(&self) -> &Reg32 {
        &self.reg32
    }
    #[doc = "0x84 - User-defined"]
    #[inline(always)]
    pub const fn reg33(&self) -> &Reg33 {
        &self.reg33
    }
    #[doc = "0x88 - User-defined"]
    #[inline(always)]
    pub const fn reg34(&self) -> &Reg34 {
        &self.reg34
    }
    #[doc = "0x8c - User-defined"]
    #[inline(always)]
    pub const fn reg35(&self) -> &Reg35 {
        &self.reg35
    }
    #[doc = "0x90 - User-defined"]
    #[inline(always)]
    pub const fn reg36(&self) -> &Reg36 {
        &self.reg36
    }
    #[doc = "0x94 - User-defined"]
    #[inline(always)]
    pub const fn reg37(&self) -> &Reg37 {
        &self.reg37
    }
    #[doc = "0x98 - User-defined"]
    #[inline(always)]
    pub const fn reg38(&self) -> &Reg38 {
        &self.reg38
    }
    #[doc = "0x9c - User-defined"]
    #[inline(always)]
    pub const fn reg39(&self) -> &Reg39 {
        &self.reg39
    }
    #[doc = "0xa0 - User-defined"]
    #[inline(always)]
    pub const fn reg40(&self) -> &Reg40 {
        &self.reg40
    }
    #[doc = "0xa4 - User-defined"]
    #[inline(always)]
    pub const fn reg41(&self) -> &Reg41 {
        &self.reg41
    }
    #[doc = "0xa8 - User-defined"]
    #[inline(always)]
    pub const fn reg42(&self) -> &Reg42 {
        &self.reg42
    }
    #[doc = "0xac - User-defined"]
    #[inline(always)]
    pub const fn reg43(&self) -> &Reg43 {
        &self.reg43
    }
    #[doc = "0xb0 - User-defined"]
    #[inline(always)]
    pub const fn reg44(&self) -> &Reg44 {
        &self.reg44
    }
    #[doc = "0xb4 - User-defined"]
    #[inline(always)]
    pub const fn reg45(&self) -> &Reg45 {
        &self.reg45
    }
    #[doc = "0xb8 - User-defined"]
    #[inline(always)]
    pub const fn reg46(&self) -> &Reg46 {
        &self.reg46
    }
    #[doc = "0xbc - User-defined"]
    #[inline(always)]
    pub const fn reg47(&self) -> &Reg47 {
        &self.reg47
    }
    #[doc = "0xc0 - User-defined"]
    #[inline(always)]
    pub const fn reg48(&self) -> &Reg48 {
        &self.reg48
    }
    #[doc = "0xc4 - User-defined"]
    #[inline(always)]
    pub const fn reg49(&self) -> &Reg49 {
        &self.reg49
    }
    #[doc = "0xc8 - User-defined"]
    #[inline(always)]
    pub const fn reg50(&self) -> &Reg50 {
        &self.reg50
    }
    #[doc = "0xcc - User-defined"]
    #[inline(always)]
    pub const fn reg51(&self) -> &Reg51 {
        &self.reg51
    }
    #[doc = "0xd0 - User-defined"]
    #[inline(always)]
    pub const fn reg52(&self) -> &Reg52 {
        &self.reg52
    }
    #[doc = "0xd4 - User-defined"]
    #[inline(always)]
    pub const fn reg53(&self) -> &Reg53 {
        &self.reg53
    }
    #[doc = "0xd8 - User-defined"]
    #[inline(always)]
    pub const fn reg54(&self) -> &Reg54 {
        &self.reg54
    }
    #[doc = "0xdc - User-defined"]
    #[inline(always)]
    pub const fn reg55(&self) -> &Reg55 {
        &self.reg55
    }
    #[doc = "0xe0 - User-defined"]
    #[inline(always)]
    pub const fn reg56(&self) -> &Reg56 {
        &self.reg56
    }
    #[doc = "0xe4 - User-defined"]
    #[inline(always)]
    pub const fn reg57(&self) -> &Reg57 {
        &self.reg57
    }
    #[doc = "0xe8 - User-defined"]
    #[inline(always)]
    pub const fn reg58(&self) -> &Reg58 {
        &self.reg58
    }
    #[doc = "0xec - User-defined"]
    #[inline(always)]
    pub const fn reg59(&self) -> &Reg59 {
        &self.reg59
    }
    #[doc = "0xf0 - User-defined"]
    #[inline(always)]
    pub const fn reg60(&self) -> &Reg60 {
        &self.reg60
    }
    #[doc = "0xf4 - User-defined"]
    #[inline(always)]
    pub const fn reg61(&self) -> &Reg61 {
        &self.reg61
    }
    #[doc = "0xf8 - User-defined"]
    #[inline(always)]
    pub const fn reg62(&self) -> &Reg62 {
        &self.reg62
    }
    #[doc = "0xfc - User-defined"]
    #[inline(always)]
    pub const fn reg63(&self) -> &Reg63 {
        &self.reg63
    }
}
#[doc = "REG0 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg0`] module"]
#[doc(alias = "REG0")]
pub type Reg0 = crate::Reg<reg0::Reg0Spec>;
#[doc = "User-defined"]
pub mod reg0;
#[doc = "REG1 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg1`] module"]
#[doc(alias = "REG1")]
pub type Reg1 = crate::Reg<reg1::Reg1Spec>;
#[doc = "User-defined"]
pub mod reg1;
#[doc = "REG2 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg2`] module"]
#[doc(alias = "REG2")]
pub type Reg2 = crate::Reg<reg2::Reg2Spec>;
#[doc = "User-defined"]
pub mod reg2;
#[doc = "REG3 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg3`] module"]
#[doc(alias = "REG3")]
pub type Reg3 = crate::Reg<reg3::Reg3Spec>;
#[doc = "User-defined"]
pub mod reg3;
#[doc = "REG4 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg4`] module"]
#[doc(alias = "REG4")]
pub type Reg4 = crate::Reg<reg4::Reg4Spec>;
#[doc = "User-defined"]
pub mod reg4;
#[doc = "REG5 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg5`] module"]
#[doc(alias = "REG5")]
pub type Reg5 = crate::Reg<reg5::Reg5Spec>;
#[doc = "User-defined"]
pub mod reg5;
#[doc = "REG6 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg6`] module"]
#[doc(alias = "REG6")]
pub type Reg6 = crate::Reg<reg6::Reg6Spec>;
#[doc = "User-defined"]
pub mod reg6;
#[doc = "REG7 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg7`] module"]
#[doc(alias = "REG7")]
pub type Reg7 = crate::Reg<reg7::Reg7Spec>;
#[doc = "User-defined"]
pub mod reg7;
#[doc = "REG8 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg8`] module"]
#[doc(alias = "REG8")]
pub type Reg8 = crate::Reg<reg8::Reg8Spec>;
#[doc = "User-defined"]
pub mod reg8;
#[doc = "REG9 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg9`] module"]
#[doc(alias = "REG9")]
pub type Reg9 = crate::Reg<reg9::Reg9Spec>;
#[doc = "User-defined"]
pub mod reg9;
#[doc = "REG10 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg10`] module"]
#[doc(alias = "REG10")]
pub type Reg10 = crate::Reg<reg10::Reg10Spec>;
#[doc = "User-defined"]
pub mod reg10;
#[doc = "REG11 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg11`] module"]
#[doc(alias = "REG11")]
pub type Reg11 = crate::Reg<reg11::Reg11Spec>;
#[doc = "User-defined"]
pub mod reg11;
#[doc = "REG12 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg12`] module"]
#[doc(alias = "REG12")]
pub type Reg12 = crate::Reg<reg12::Reg12Spec>;
#[doc = "User-defined"]
pub mod reg12;
#[doc = "REG13 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg13`] module"]
#[doc(alias = "REG13")]
pub type Reg13 = crate::Reg<reg13::Reg13Spec>;
#[doc = "User-defined"]
pub mod reg13;
#[doc = "REG14 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg14`] module"]
#[doc(alias = "REG14")]
pub type Reg14 = crate::Reg<reg14::Reg14Spec>;
#[doc = "User-defined"]
pub mod reg14;
#[doc = "REG15 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg15`] module"]
#[doc(alias = "REG15")]
pub type Reg15 = crate::Reg<reg15::Reg15Spec>;
#[doc = "User-defined"]
pub mod reg15;
#[doc = "REG16 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg16`] module"]
#[doc(alias = "REG16")]
pub type Reg16 = crate::Reg<reg16::Reg16Spec>;
#[doc = "User-defined"]
pub mod reg16;
#[doc = "REG17 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg17`] module"]
#[doc(alias = "REG17")]
pub type Reg17 = crate::Reg<reg17::Reg17Spec>;
#[doc = "User-defined"]
pub mod reg17;
#[doc = "REG18 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg18`] module"]
#[doc(alias = "REG18")]
pub type Reg18 = crate::Reg<reg18::Reg18Spec>;
#[doc = "User-defined"]
pub mod reg18;
#[doc = "REG19 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg19`] module"]
#[doc(alias = "REG19")]
pub type Reg19 = crate::Reg<reg19::Reg19Spec>;
#[doc = "User-defined"]
pub mod reg19;
#[doc = "REG20 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg20`] module"]
#[doc(alias = "REG20")]
pub type Reg20 = crate::Reg<reg20::Reg20Spec>;
#[doc = "User-defined"]
pub mod reg20;
#[doc = "REG21 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg21`] module"]
#[doc(alias = "REG21")]
pub type Reg21 = crate::Reg<reg21::Reg21Spec>;
#[doc = "User-defined"]
pub mod reg21;
#[doc = "REG22 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg22`] module"]
#[doc(alias = "REG22")]
pub type Reg22 = crate::Reg<reg22::Reg22Spec>;
#[doc = "User-defined"]
pub mod reg22;
#[doc = "REG23 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg23`] module"]
#[doc(alias = "REG23")]
pub type Reg23 = crate::Reg<reg23::Reg23Spec>;
#[doc = "User-defined"]
pub mod reg23;
#[doc = "REG24 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg24`] module"]
#[doc(alias = "REG24")]
pub type Reg24 = crate::Reg<reg24::Reg24Spec>;
#[doc = "User-defined"]
pub mod reg24;
#[doc = "REG25 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg25`] module"]
#[doc(alias = "REG25")]
pub type Reg25 = crate::Reg<reg25::Reg25Spec>;
#[doc = "User-defined"]
pub mod reg25;
#[doc = "REG26 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg26`] module"]
#[doc(alias = "REG26")]
pub type Reg26 = crate::Reg<reg26::Reg26Spec>;
#[doc = "User-defined"]
pub mod reg26;
#[doc = "REG27 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg27`] module"]
#[doc(alias = "REG27")]
pub type Reg27 = crate::Reg<reg27::Reg27Spec>;
#[doc = "User-defined"]
pub mod reg27;
#[doc = "REG28 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg28`] module"]
#[doc(alias = "REG28")]
pub type Reg28 = crate::Reg<reg28::Reg28Spec>;
#[doc = "User-defined"]
pub mod reg28;
#[doc = "REG29 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg29`] module"]
#[doc(alias = "REG29")]
pub type Reg29 = crate::Reg<reg29::Reg29Spec>;
#[doc = "User-defined"]
pub mod reg29;
#[doc = "REG30 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg30`] module"]
#[doc(alias = "REG30")]
pub type Reg30 = crate::Reg<reg30::Reg30Spec>;
#[doc = "User-defined"]
pub mod reg30;
#[doc = "REG31 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg31`] module"]
#[doc(alias = "REG31")]
pub type Reg31 = crate::Reg<reg31::Reg31Spec>;
#[doc = "User-defined"]
pub mod reg31;
#[doc = "REG32 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg32`] module"]
#[doc(alias = "REG32")]
pub type Reg32 = crate::Reg<reg32::Reg32Spec>;
#[doc = "User-defined"]
pub mod reg32;
#[doc = "REG33 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg33`] module"]
#[doc(alias = "REG33")]
pub type Reg33 = crate::Reg<reg33::Reg33Spec>;
#[doc = "User-defined"]
pub mod reg33;
#[doc = "REG34 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg34`] module"]
#[doc(alias = "REG34")]
pub type Reg34 = crate::Reg<reg34::Reg34Spec>;
#[doc = "User-defined"]
pub mod reg34;
#[doc = "REG35 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg35`] module"]
#[doc(alias = "REG35")]
pub type Reg35 = crate::Reg<reg35::Reg35Spec>;
#[doc = "User-defined"]
pub mod reg35;
#[doc = "REG36 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg36`] module"]
#[doc(alias = "REG36")]
pub type Reg36 = crate::Reg<reg36::Reg36Spec>;
#[doc = "User-defined"]
pub mod reg36;
#[doc = "REG37 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg37`] module"]
#[doc(alias = "REG37")]
pub type Reg37 = crate::Reg<reg37::Reg37Spec>;
#[doc = "User-defined"]
pub mod reg37;
#[doc = "REG38 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg38`] module"]
#[doc(alias = "REG38")]
pub type Reg38 = crate::Reg<reg38::Reg38Spec>;
#[doc = "User-defined"]
pub mod reg38;
#[doc = "REG39 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg39`] module"]
#[doc(alias = "REG39")]
pub type Reg39 = crate::Reg<reg39::Reg39Spec>;
#[doc = "User-defined"]
pub mod reg39;
#[doc = "REG40 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg40`] module"]
#[doc(alias = "REG40")]
pub type Reg40 = crate::Reg<reg40::Reg40Spec>;
#[doc = "User-defined"]
pub mod reg40;
#[doc = "REG41 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg41`] module"]
#[doc(alias = "REG41")]
pub type Reg41 = crate::Reg<reg41::Reg41Spec>;
#[doc = "User-defined"]
pub mod reg41;
#[doc = "REG42 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg42`] module"]
#[doc(alias = "REG42")]
pub type Reg42 = crate::Reg<reg42::Reg42Spec>;
#[doc = "User-defined"]
pub mod reg42;
#[doc = "REG43 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg43`] module"]
#[doc(alias = "REG43")]
pub type Reg43 = crate::Reg<reg43::Reg43Spec>;
#[doc = "User-defined"]
pub mod reg43;
#[doc = "REG44 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg44`] module"]
#[doc(alias = "REG44")]
pub type Reg44 = crate::Reg<reg44::Reg44Spec>;
#[doc = "User-defined"]
pub mod reg44;
#[doc = "REG45 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg45`] module"]
#[doc(alias = "REG45")]
pub type Reg45 = crate::Reg<reg45::Reg45Spec>;
#[doc = "User-defined"]
pub mod reg45;
#[doc = "REG46 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg46`] module"]
#[doc(alias = "REG46")]
pub type Reg46 = crate::Reg<reg46::Reg46Spec>;
#[doc = "User-defined"]
pub mod reg46;
#[doc = "REG47 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg47`] module"]
#[doc(alias = "REG47")]
pub type Reg47 = crate::Reg<reg47::Reg47Spec>;
#[doc = "User-defined"]
pub mod reg47;
#[doc = "REG48 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg48`] module"]
#[doc(alias = "REG48")]
pub type Reg48 = crate::Reg<reg48::Reg48Spec>;
#[doc = "User-defined"]
pub mod reg48;
#[doc = "REG49 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg49`] module"]
#[doc(alias = "REG49")]
pub type Reg49 = crate::Reg<reg49::Reg49Spec>;
#[doc = "User-defined"]
pub mod reg49;
#[doc = "REG50 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg50`] module"]
#[doc(alias = "REG50")]
pub type Reg50 = crate::Reg<reg50::Reg50Spec>;
#[doc = "User-defined"]
pub mod reg50;
#[doc = "REG51 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg51`] module"]
#[doc(alias = "REG51")]
pub type Reg51 = crate::Reg<reg51::Reg51Spec>;
#[doc = "User-defined"]
pub mod reg51;
#[doc = "REG52 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg52`] module"]
#[doc(alias = "REG52")]
pub type Reg52 = crate::Reg<reg52::Reg52Spec>;
#[doc = "User-defined"]
pub mod reg52;
#[doc = "REG53 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg53`] module"]
#[doc(alias = "REG53")]
pub type Reg53 = crate::Reg<reg53::Reg53Spec>;
#[doc = "User-defined"]
pub mod reg53;
#[doc = "REG54 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg54`] module"]
#[doc(alias = "REG54")]
pub type Reg54 = crate::Reg<reg54::Reg54Spec>;
#[doc = "User-defined"]
pub mod reg54;
#[doc = "REG55 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg55`] module"]
#[doc(alias = "REG55")]
pub type Reg55 = crate::Reg<reg55::Reg55Spec>;
#[doc = "User-defined"]
pub mod reg55;
#[doc = "REG56 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg56`] module"]
#[doc(alias = "REG56")]
pub type Reg56 = crate::Reg<reg56::Reg56Spec>;
#[doc = "User-defined"]
pub mod reg56;
#[doc = "REG57 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg57`] module"]
#[doc(alias = "REG57")]
pub type Reg57 = crate::Reg<reg57::Reg57Spec>;
#[doc = "User-defined"]
pub mod reg57;
#[doc = "REG58 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg58`] module"]
#[doc(alias = "REG58")]
pub type Reg58 = crate::Reg<reg58::Reg58Spec>;
#[doc = "User-defined"]
pub mod reg58;
#[doc = "REG59 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg59`] module"]
#[doc(alias = "REG59")]
pub type Reg59 = crate::Reg<reg59::Reg59Spec>;
#[doc = "User-defined"]
pub mod reg59;
#[doc = "REG60 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg60`] module"]
#[doc(alias = "REG60")]
pub type Reg60 = crate::Reg<reg60::Reg60Spec>;
#[doc = "User-defined"]
pub mod reg60;
#[doc = "REG61 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg61`] module"]
#[doc(alias = "REG61")]
pub type Reg61 = crate::Reg<reg61::Reg61Spec>;
#[doc = "User-defined"]
pub mod reg61;
#[doc = "REG62 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg62`] module"]
#[doc(alias = "REG62")]
pub type Reg62 = crate::Reg<reg62::Reg62Spec>;
#[doc = "User-defined"]
pub mod reg62;
#[doc = "REG63 (rw) register accessor: User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg63`] module"]
#[doc(alias = "REG63")]
pub type Reg63 = crate::Reg<reg63::Reg63Spec>;
#[doc = "User-defined"]
pub mod reg63;
