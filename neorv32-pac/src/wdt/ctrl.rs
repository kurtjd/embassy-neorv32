#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `WDT_CTRL_EN` reader - WDT enable flag"]
pub type WdtCtrlEnR = crate::BitReader;
#[doc = "Field `WDT_CTRL_EN` writer - WDT enable flag"]
pub type WdtCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CTRL_LOCK` reader - Lock write access to control register, clears on reset only"]
pub type WdtCtrlLockR = crate::BitReader;
#[doc = "Field `WDT_CTRL_LOCK` writer - Lock write access to control register, clears on reset only"]
pub type WdtCtrlLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CTRL_RCAUSE` reader - Cause of last system reset: 0=external reset, 1=OCD reset, 2=WDT reset, 3=WDT access violation"]
pub type WdtCtrlRcauseR = crate::FieldReader;
#[doc = "Field `WDT_CTRL_TIMEOUT` reader - Timeout value"]
pub type WdtCtrlTimeoutR = crate::FieldReader<u32>;
#[doc = "Field `WDT_CTRL_TIMEOUT` writer - Timeout value"]
pub type WdtCtrlTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - WDT enable flag"]
    #[inline(always)]
    pub fn wdt_ctrl_en(&self) -> WdtCtrlEnR {
        WdtCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock write access to control register, clears on reset only"]
    #[inline(always)]
    pub fn wdt_ctrl_lock(&self) -> WdtCtrlLockR {
        WdtCtrlLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Cause of last system reset: 0=external reset, 1=OCD reset, 2=WDT reset, 3=WDT access violation"]
    #[inline(always)]
    pub fn wdt_ctrl_rcause(&self) -> WdtCtrlRcauseR {
        WdtCtrlRcauseR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Timeout value"]
    #[inline(always)]
    pub fn wdt_ctrl_timeout(&self) -> WdtCtrlTimeoutR {
        WdtCtrlTimeoutR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - WDT enable flag"]
    #[inline(always)]
    pub fn wdt_ctrl_en(&mut self) -> WdtCtrlEnW<'_, CtrlSpec> {
        WdtCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock write access to control register, clears on reset only"]
    #[inline(always)]
    pub fn wdt_ctrl_lock(&mut self) -> WdtCtrlLockW<'_, CtrlSpec> {
        WdtCtrlLockW::new(self, 1)
    }
    #[doc = "Bits 8:31 - Timeout value"]
    #[inline(always)]
    pub fn wdt_ctrl_timeout(&mut self) -> WdtCtrlTimeoutW<'_, CtrlSpec> {
        WdtCtrlTimeoutW::new(self, 8)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
