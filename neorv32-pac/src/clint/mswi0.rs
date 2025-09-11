#[doc = "Register `MSWI0` reader"]
pub type R = crate::R<Mswi0Spec>;
#[doc = "Register `MSWI0` writer"]
pub type W = crate::W<Mswi0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine software interrupt; hart 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mswi0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mswi0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mswi0Spec;
impl crate::RegisterSpec for Mswi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mswi0::R`](R) reader structure"]
impl crate::Readable for Mswi0Spec {}
#[doc = "`write(|w| ..)` method takes [`mswi0::W`](W) writer structure"]
impl crate::Writable for Mswi0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSWI0 to value 0"]
impl crate::Resettable for Mswi0Spec {}
