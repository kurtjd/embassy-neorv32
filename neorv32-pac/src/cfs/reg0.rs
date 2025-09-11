#[doc = "Register `REG0` reader"]
pub type R = crate::R<Reg0Spec>;
#[doc = "Register `REG0` writer"]
pub type W = crate::W<Reg0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg0Spec;
impl crate::RegisterSpec for Reg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg0::R`](R) reader structure"]
impl crate::Readable for Reg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg0::W`](W) writer structure"]
impl crate::Writable for Reg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG0 to value 0"]
impl crate::Resettable for Reg0Spec {}
