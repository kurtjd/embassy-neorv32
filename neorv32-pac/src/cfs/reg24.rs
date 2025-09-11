#[doc = "Register `REG24` reader"]
pub type R = crate::R<Reg24Spec>;
#[doc = "Register `REG24` writer"]
pub type W = crate::W<Reg24Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg24Spec;
impl crate::RegisterSpec for Reg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg24::R`](R) reader structure"]
impl crate::Readable for Reg24Spec {}
#[doc = "`write(|w| ..)` method takes [`reg24::W`](W) writer structure"]
impl crate::Writable for Reg24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG24 to value 0"]
impl crate::Resettable for Reg24Spec {}
