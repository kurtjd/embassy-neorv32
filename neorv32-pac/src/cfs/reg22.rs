#[doc = "Register `REG22` reader"]
pub type R = crate::R<Reg22Spec>;
#[doc = "Register `REG22` writer"]
pub type W = crate::W<Reg22Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg22Spec;
impl crate::RegisterSpec for Reg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg22::R`](R) reader structure"]
impl crate::Readable for Reg22Spec {}
#[doc = "`write(|w| ..)` method takes [`reg22::W`](W) writer structure"]
impl crate::Writable for Reg22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG22 to value 0"]
impl crate::Resettable for Reg22Spec {}
