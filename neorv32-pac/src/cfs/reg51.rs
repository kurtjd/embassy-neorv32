#[doc = "Register `REG51` reader"]
pub type R = crate::R<Reg51Spec>;
#[doc = "Register `REG51` writer"]
pub type W = crate::W<Reg51Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg51Spec;
impl crate::RegisterSpec for Reg51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg51::R`](R) reader structure"]
impl crate::Readable for Reg51Spec {}
#[doc = "`write(|w| ..)` method takes [`reg51::W`](W) writer structure"]
impl crate::Writable for Reg51Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG51 to value 0"]
impl crate::Resettable for Reg51Spec {}
