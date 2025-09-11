#[doc = "Register `REG7` reader"]
pub type R = crate::R<Reg7Spec>;
#[doc = "Register `REG7` writer"]
pub type W = crate::W<Reg7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg7Spec;
impl crate::RegisterSpec for Reg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg7::R`](R) reader structure"]
impl crate::Readable for Reg7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg7::W`](W) writer structure"]
impl crate::Writable for Reg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG7 to value 0"]
impl crate::Resettable for Reg7Spec {}
