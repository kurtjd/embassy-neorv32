#[doc = "Register `CHANNEL_CFG[0]` reader"]
pub type R = crate::R<ChannelCfg0Spec>;
#[doc = "Register `CHANNEL_CFG[0]` writer"]
pub type W = crate::W<ChannelCfg0Spec>;
#[doc = "Field `PWM_CFG_DUTY` reader - Duty cycle"]
pub type PwmCfgDutyR = crate::FieldReader;
#[doc = "Field `PWM_CFG_DUTY` writer - Duty cycle"]
pub type PwmCfgDutyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWM_CFG_CDIV` reader - Clock divider"]
pub type PwmCfgCdivR = crate::FieldReader<u16>;
#[doc = "Field `PWM_CFG_CDIV` writer - Clock divider"]
pub type PwmCfgCdivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PWM_CFG_POL` reader - Channel polarity, inverted when set"]
pub type PwmCfgPolR = crate::BitReader;
#[doc = "Field `PWM_CFG_POL` writer - Channel polarity, inverted when set"]
pub type PwmCfgPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_CFG_PRSC` reader - Clock prescaler select"]
pub type PwmCfgPrscR = crate::FieldReader;
#[doc = "Field `PWM_CFG_PRSC` writer - Clock prescaler select"]
pub type PwmCfgPrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PWM_CFG_EN` reader - Channel enable"]
pub type PwmCfgEnR = crate::BitReader;
#[doc = "Field `PWM_CFG_EN` writer - Channel enable"]
pub type PwmCfgEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Duty cycle"]
    #[inline(always)]
    pub fn pwm_cfg_duty(&self) -> PwmCfgDutyR {
        PwmCfgDutyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - Clock divider"]
    #[inline(always)]
    pub fn pwm_cfg_cdiv(&self) -> PwmCfgCdivR {
        PwmCfgCdivR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 27 - Channel polarity, inverted when set"]
    #[inline(always)]
    pub fn pwm_cfg_pol(&self) -> PwmCfgPolR {
        PwmCfgPolR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Clock prescaler select"]
    #[inline(always)]
    pub fn pwm_cfg_prsc(&self) -> PwmCfgPrscR {
        PwmCfgPrscR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn pwm_cfg_en(&self) -> PwmCfgEnR {
        PwmCfgEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Duty cycle"]
    #[inline(always)]
    pub fn pwm_cfg_duty(&mut self) -> PwmCfgDutyW<'_, ChannelCfg0Spec> {
        PwmCfgDutyW::new(self, 0)
    }
    #[doc = "Bits 8:17 - Clock divider"]
    #[inline(always)]
    pub fn pwm_cfg_cdiv(&mut self) -> PwmCfgCdivW<'_, ChannelCfg0Spec> {
        PwmCfgCdivW::new(self, 8)
    }
    #[doc = "Bit 27 - Channel polarity, inverted when set"]
    #[inline(always)]
    pub fn pwm_cfg_pol(&mut self) -> PwmCfgPolW<'_, ChannelCfg0Spec> {
        PwmCfgPolW::new(self, 27)
    }
    #[doc = "Bits 28:30 - Clock prescaler select"]
    #[inline(always)]
    pub fn pwm_cfg_prsc(&mut self) -> PwmCfgPrscW<'_, ChannelCfg0Spec> {
        PwmCfgPrscW::new(self, 28)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn pwm_cfg_en(&mut self) -> PwmCfgEnW<'_, ChannelCfg0Spec> {
        PwmCfgEnW::new(self, 31)
    }
}
#[doc = "Channel 0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelCfg0Spec;
impl crate::RegisterSpec for ChannelCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel_cfg0::R`](R) reader structure"]
impl crate::Readable for ChannelCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`channel_cfg0::W`](W) writer structure"]
impl crate::Writable for ChannelCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNEL_CFG[0] to value 0"]
impl crate::Resettable for ChannelCfg0Spec {}
