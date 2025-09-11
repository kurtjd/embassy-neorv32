#[doc = "Register `REG56` reader"]
pub type R = crate::R<Reg56Spec>;
#[doc = "Register `REG56` writer"]
pub type W = crate::W<Reg56Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg56Spec;
impl crate::RegisterSpec for Reg56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg56::R`](R) reader structure"]
impl crate::Readable for Reg56Spec {}
#[doc = "`write(|w| ..)` method takes [`reg56::W`](W) writer structure"]
impl crate::Writable for Reg56Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG56 to value 0"]
impl crate::Resettable for Reg56Spec {}
