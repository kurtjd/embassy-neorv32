#[doc = "Register `DATA24` writer"]
pub type W = crate::W<Data24Spec>;
impl core::fmt::Debug for crate::generic::Reg<Data24Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Send 24-bit data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data24::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data24Spec;
impl crate::RegisterSpec for Data24Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data24::W`](W) writer structure"]
impl crate::Writable for Data24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA24 to value 0"]
impl crate::Resettable for Data24Spec {}
