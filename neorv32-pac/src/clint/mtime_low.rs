#[doc = "Register `MTIME_LOW` reader"]
pub type R = crate::R<MtimeLowSpec>;
#[doc = "Register `MTIME_LOW` writer"]
pub type W = crate::W<MtimeLowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine timer low word\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimeLowSpec;
impl crate::RegisterSpec for MtimeLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime_low::R`](R) reader structure"]
impl crate::Readable for MtimeLowSpec {}
#[doc = "`write(|w| ..)` method takes [`mtime_low::W`](W) writer structure"]
impl crate::Writable for MtimeLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIME_LOW to value 0"]
impl crate::Resettable for MtimeLowSpec {}
