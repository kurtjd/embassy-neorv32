#[doc = "Register `REG48` reader"]
pub type R = crate::R<Reg48Spec>;
#[doc = "Register `REG48` writer"]
pub type W = crate::W<Reg48Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg48Spec;
impl crate::RegisterSpec for Reg48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg48::R`](R) reader structure"]
impl crate::Readable for Reg48Spec {}
#[doc = "`write(|w| ..)` method takes [`reg48::W`](W) writer structure"]
impl crate::Writable for Reg48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG48 to value 0"]
impl crate::Resettable for Reg48Spec {}
