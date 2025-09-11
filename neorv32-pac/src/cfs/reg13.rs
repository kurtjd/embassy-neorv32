#[doc = "Register `REG13` reader"]
pub type R = crate::R<Reg13Spec>;
#[doc = "Register `REG13` writer"]
pub type W = crate::W<Reg13Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg13Spec;
impl crate::RegisterSpec for Reg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg13::R`](R) reader structure"]
impl crate::Readable for Reg13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg13::W`](W) writer structure"]
impl crate::Writable for Reg13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG13 to value 0"]
impl crate::Resettable for Reg13Spec {}
