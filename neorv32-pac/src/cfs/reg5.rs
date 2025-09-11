#[doc = "Register `REG5` reader"]
pub type R = crate::R<Reg5Spec>;
#[doc = "Register `REG5` writer"]
pub type W = crate::W<Reg5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg5Spec;
impl crate::RegisterSpec for Reg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg5::R`](R) reader structure"]
impl crate::Readable for Reg5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg5::W`](W) writer structure"]
impl crate::Writable for Reg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG5 to value 0"]
impl crate::Resettable for Reg5Spec {}
