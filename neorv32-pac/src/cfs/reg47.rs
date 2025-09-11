#[doc = "Register `REG47` reader"]
pub type R = crate::R<Reg47Spec>;
#[doc = "Register `REG47` writer"]
pub type W = crate::W<Reg47Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg47Spec;
impl crate::RegisterSpec for Reg47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg47::R`](R) reader structure"]
impl crate::Readable for Reg47Spec {}
#[doc = "`write(|w| ..)` method takes [`reg47::W`](W) writer structure"]
impl crate::Writable for Reg47Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG47 to value 0"]
impl crate::Resettable for Reg47Spec {}
