#[doc = "Register `REG45` reader"]
pub type R = crate::R<Reg45Spec>;
#[doc = "Register `REG45` writer"]
pub type W = crate::W<Reg45Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg45Spec;
impl crate::RegisterSpec for Reg45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg45::R`](R) reader structure"]
impl crate::Readable for Reg45Spec {}
#[doc = "`write(|w| ..)` method takes [`reg45::W`](W) writer structure"]
impl crate::Writable for Reg45Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG45 to value 0"]
impl crate::Resettable for Reg45Spec {}
