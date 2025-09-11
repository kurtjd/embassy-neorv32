#[doc = "Register `MTIMECMP0_LOW` reader"]
pub type R = crate::R<Mtimecmp0LowSpec>;
#[doc = "Register `MTIMECMP0_LOW` writer"]
pub type W = crate::W<Mtimecmp0LowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine timer compare low word; hart0\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp0_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp0_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtimecmp0LowSpec;
impl crate::RegisterSpec for Mtimecmp0LowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp0_low::R`](R) reader structure"]
impl crate::Readable for Mtimecmp0LowSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp0_low::W`](W) writer structure"]
impl crate::Writable for Mtimecmp0LowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIMECMP0_LOW to value 0"]
impl crate::Resettable for Mtimecmp0LowSpec {}
