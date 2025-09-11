#[doc = "Register `REG38` reader"]
pub type R = crate::R<Reg38Spec>;
#[doc = "Register `REG38` writer"]
pub type W = crate::W<Reg38Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg38Spec;
impl crate::RegisterSpec for Reg38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg38::R`](R) reader structure"]
impl crate::Readable for Reg38Spec {}
#[doc = "`write(|w| ..)` method takes [`reg38::W`](W) writer structure"]
impl crate::Writable for Reg38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG38 to value 0"]
impl crate::Resettable for Reg38Spec {}
