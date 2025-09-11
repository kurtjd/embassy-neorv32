#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    channel_cfg0: ChannelCfg0,
    channel_cfg1: ChannelCfg1,
    channel_cfg2: ChannelCfg2,
    channel_cfg3: ChannelCfg3,
    channel_cfg4: ChannelCfg4,
    channel_cfg5: ChannelCfg5,
    channel_cfg6: ChannelCfg6,
    channel_cfg7: ChannelCfg7,
    channel_cfg8: ChannelCfg8,
    channel_cfg9: ChannelCfg9,
    channel_cfg10: ChannelCfg10,
    channel_cfg11: ChannelCfg11,
    channel_cfg12: ChannelCfg12,
    channel_cfg13: ChannelCfg13,
    channel_cfg14: ChannelCfg14,
    channel_cfg15: ChannelCfg15,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel 0 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg0(&self) -> &ChannelCfg0 {
        &self.channel_cfg0
    }
    #[doc = "0x04 - Channel 1 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg1(&self) -> &ChannelCfg1 {
        &self.channel_cfg1
    }
    #[doc = "0x08 - Channel 2 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg2(&self) -> &ChannelCfg2 {
        &self.channel_cfg2
    }
    #[doc = "0x0c - Channel 3 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg3(&self) -> &ChannelCfg3 {
        &self.channel_cfg3
    }
    #[doc = "0x10 - Channel 4 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg4(&self) -> &ChannelCfg4 {
        &self.channel_cfg4
    }
    #[doc = "0x14 - Channel 5 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg5(&self) -> &ChannelCfg5 {
        &self.channel_cfg5
    }
    #[doc = "0x18 - Channel 6 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg6(&self) -> &ChannelCfg6 {
        &self.channel_cfg6
    }
    #[doc = "0x1c - Channel 7 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg7(&self) -> &ChannelCfg7 {
        &self.channel_cfg7
    }
    #[doc = "0x20 - Channel 8 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg8(&self) -> &ChannelCfg8 {
        &self.channel_cfg8
    }
    #[doc = "0x24 - Channel 9 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg9(&self) -> &ChannelCfg9 {
        &self.channel_cfg9
    }
    #[doc = "0x28 - Channel 10 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg10(&self) -> &ChannelCfg10 {
        &self.channel_cfg10
    }
    #[doc = "0x2c - Channel 11 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg11(&self) -> &ChannelCfg11 {
        &self.channel_cfg11
    }
    #[doc = "0x30 - Channel 12 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg12(&self) -> &ChannelCfg12 {
        &self.channel_cfg12
    }
    #[doc = "0x34 - Channel 13 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg13(&self) -> &ChannelCfg13 {
        &self.channel_cfg13
    }
    #[doc = "0x38 - Channel 14 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg14(&self) -> &ChannelCfg14 {
        &self.channel_cfg14
    }
    #[doc = "0x3c - Channel 15 configuration register"]
    #[inline(always)]
    pub const fn channel_cfg15(&self) -> &ChannelCfg15 {
        &self.channel_cfg15
    }
}
#[doc = "CHANNEL_CFG[0] (rw) register accessor: Channel 0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_cfg0`] module"]
#[doc(alias = "CHANNEL_CFG[0]")]
pub type ChannelCfg0 = crate::Reg<channel_cfg0::ChannelCfg0Spec>;
#[doc = "Channel 0 configuration register"]
pub mod channel_cfg0;
pub use ChannelCfg0 as ChannelCfg1;
pub use ChannelCfg0 as ChannelCfg2;
pub use ChannelCfg0 as ChannelCfg3;
pub use ChannelCfg0 as ChannelCfg4;
pub use ChannelCfg0 as ChannelCfg5;
pub use ChannelCfg0 as ChannelCfg6;
pub use ChannelCfg0 as ChannelCfg7;
pub use ChannelCfg0 as ChannelCfg8;
pub use ChannelCfg0 as ChannelCfg9;
pub use ChannelCfg0 as ChannelCfg10;
pub use ChannelCfg0 as ChannelCfg11;
pub use ChannelCfg0 as ChannelCfg12;
pub use ChannelCfg0 as ChannelCfg13;
pub use ChannelCfg0 as ChannelCfg14;
pub use ChannelCfg0 as ChannelCfg15;
pub use channel_cfg0 as channel_cfg1;
pub use channel_cfg0 as channel_cfg2;
pub use channel_cfg0 as channel_cfg3;
pub use channel_cfg0 as channel_cfg4;
pub use channel_cfg0 as channel_cfg5;
pub use channel_cfg0 as channel_cfg6;
pub use channel_cfg0 as channel_cfg7;
pub use channel_cfg0 as channel_cfg8;
pub use channel_cfg0 as channel_cfg9;
pub use channel_cfg0 as channel_cfg10;
pub use channel_cfg0 as channel_cfg11;
pub use channel_cfg0 as channel_cfg12;
pub use channel_cfg0 as channel_cfg13;
pub use channel_cfg0 as channel_cfg14;
pub use channel_cfg0 as channel_cfg15;
