#[doc = "Register `REG31` reader"]
pub type R = crate::R<Reg31Spec>;
#[doc = "Register `REG31` writer"]
pub type W = crate::W<Reg31Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg31Spec;
impl crate::RegisterSpec for Reg31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg31::R`](R) reader structure"]
impl crate::Readable for Reg31Spec {}
#[doc = "`write(|w| ..)` method takes [`reg31::W`](W) writer structure"]
impl crate::Writable for Reg31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG31 to value 0"]
impl crate::Resettable for Reg31Spec {}
