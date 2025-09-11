#[doc = "Register `REG4` reader"]
pub type R = crate::R<Reg4Spec>;
#[doc = "Register `REG4` writer"]
pub type W = crate::W<Reg4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg4Spec;
impl crate::RegisterSpec for Reg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg4::R`](R) reader structure"]
impl crate::Readable for Reg4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg4::W`](W) writer structure"]
impl crate::Writable for Reg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG4 to value 0"]
impl crate::Resettable for Reg4Spec {}
