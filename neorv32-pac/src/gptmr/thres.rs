#[doc = "Register `THRES` reader"]
pub type R = crate::R<ThresSpec>;
#[doc = "Register `THRES` writer"]
pub type W = crate::W<ThresSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThresSpec;
impl crate::RegisterSpec for ThresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres::R`](R) reader structure"]
impl crate::Readable for ThresSpec {}
#[doc = "`write(|w| ..)` method takes [`thres::W`](W) writer structure"]
impl crate::Writable for ThresSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRES to value 0"]
impl crate::Resettable for ThresSpec {}
