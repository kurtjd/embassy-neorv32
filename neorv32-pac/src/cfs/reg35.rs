#[doc = "Register `REG35` reader"]
pub type R = crate::R<Reg35Spec>;
#[doc = "Register `REG35` writer"]
pub type W = crate::W<Reg35Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User-defined\n\nYou can [`read`](crate::Reg::read) this register and get [`reg35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg35Spec;
impl crate::RegisterSpec for Reg35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg35::R`](R) reader structure"]
impl crate::Readable for Reg35Spec {}
#[doc = "`write(|w| ..)` method takes [`reg35::W`](W) writer structure"]
impl crate::Writable for Reg35Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG35 to value 0"]
impl crate::Resettable for Reg35Spec {}
