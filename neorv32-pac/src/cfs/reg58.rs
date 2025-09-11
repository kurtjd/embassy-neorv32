#[doc = "Register `REG58` reader"]
pub type R = crate::R<Reg58Spec>;
#[doc = "Register `REG58` writer"]
pub type W = crate::W<Reg58Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg58Spec;
impl crate::RegisterSpec for Reg58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg58::R`](R) reader structure"]
impl crate::Readable for Reg58Spec {}
#[doc = "`write(|w| ..)` method takes [`reg58::W`](W) writer structure"]
impl crate::Writable for Reg58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG58 to value 0"]
impl crate::Resettable for Reg58Spec {}
