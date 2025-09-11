#[doc = "Register `MTIMECMP0_HI` reader"]
pub type R = crate::R<Mtimecmp0HiSpec>;
#[doc = "Register `MTIMECMP0_HI` writer"]
pub type W = crate::W<Mtimecmp0HiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine timer compare low word; hart0\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp0_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp0_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtimecmp0HiSpec;
impl crate::RegisterSpec for Mtimecmp0HiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp0_hi::R`](R) reader structure"]
impl crate::Readable for Mtimecmp0HiSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp0_hi::W`](W) writer structure"]
impl crate::Writable for Mtimecmp0HiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIMECMP0_HI to value 0"]
impl crate::Resettable for Mtimecmp0HiSpec {}
