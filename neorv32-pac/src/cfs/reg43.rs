#[doc = "Register `REG43` reader"]
pub type R = crate::R<Reg43Spec>;
#[doc = "Register `REG43` writer"]
pub type W = crate::W<Reg43Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg43Spec;
impl crate::RegisterSpec for Reg43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg43::R`](R) reader structure"]
impl crate::Readable for Reg43Spec {}
#[doc = "`write(|w| ..)` method takes [`reg43::W`](W) writer structure"]
impl crate::Writable for Reg43Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG43 to value 0"]
impl crate::Resettable for Reg43Spec {}
