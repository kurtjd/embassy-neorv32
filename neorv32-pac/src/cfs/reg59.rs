#[doc = "Register `REG59` reader"]
pub type R = crate::R<Reg59Spec>;
#[doc = "Register `REG59` writer"]
pub type W = crate::W<Reg59Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg59::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg59::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg59Spec;
impl crate::RegisterSpec for Reg59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg59::R`](R) reader structure"]
impl crate::Readable for Reg59Spec {}
#[doc = "`write(|w| ..)` method takes [`reg59::W`](W) writer structure"]
impl crate::Writable for Reg59Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG59 to value 0"]
impl crate::Resettable for Reg59Spec {}
