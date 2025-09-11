#[doc = "Register `MTIME_HI` reader"]
pub type R = crate::R<MtimeHiSpec>;
#[doc = "Register `MTIME_HI` writer"]
pub type W = crate::W<MtimeHiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine timer high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimeHiSpec;
impl crate::RegisterSpec for MtimeHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime_hi::R`](R) reader structure"]
impl crate::Readable for MtimeHiSpec {}
#[doc = "`write(|w| ..)` method takes [`mtime_hi::W`](W) writer structure"]
impl crate::Writable for MtimeHiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIME_HI to value 0"]
impl crate::Resettable for MtimeHiSpec {}
