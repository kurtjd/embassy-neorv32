#[doc = "Register `REG15` reader"]
pub type R = crate::R<Reg15Spec>;
#[doc = "Register `REG15` writer"]
pub type W = crate::W<Reg15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg15Spec;
impl crate::RegisterSpec for Reg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg15::R`](R) reader structure"]
impl crate::Readable for Reg15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg15::W`](W) writer structure"]
impl crate::Writable for Reg15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG15 to value 0"]
impl crate::Resettable for Reg15Spec {}
