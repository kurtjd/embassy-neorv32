#[doc = "Register `REG19` reader"]
pub type R = crate::R<Reg19Spec>;
#[doc = "Register `REG19` writer"]
pub type W = crate::W<Reg19Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg19Spec;
impl crate::RegisterSpec for Reg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg19::R`](R) reader structure"]
impl crate::Readable for Reg19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg19::W`](W) writer structure"]
impl crate::Writable for Reg19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG19 to value 0"]
impl crate::Resettable for Reg19Spec {}
