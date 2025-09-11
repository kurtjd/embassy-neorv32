#[doc = "Register `RESET` reader"]
pub type R = crate::R<ResetSpec>;
#[doc = "Register `RESET` writer"]
pub type W = crate::W<ResetSpec>;
#[doc = "Field `WDT_RESET` writer - Write password to reset/feed the watchdog (0x709D1AB3)"]
pub type WdtResetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write password to reset/feed the watchdog (0x709D1AB3)"]
    #[inline(always)]
    pub fn wdt_reset(&mut self) -> WdtResetW<'_, ResetSpec> {
        WdtResetW::new(self, 0)
    }
}
#[doc = "Watchdog reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSpec;
impl crate::RegisterSpec for ResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset::R`](R) reader structure"]
impl crate::Readable for ResetSpec {}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for ResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for ResetSpec {}
