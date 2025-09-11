#[doc = "Register `DATA32` writer"]
pub type W = crate::W<Data32Spec>;
impl core::fmt::Debug for crate::generic::Reg<Data32Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Send 32-bit data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data32::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data32Spec;
impl crate::RegisterSpec for Data32Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data32::W`](W) writer structure"]
impl crate::Writable for Data32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA32 to value 0"]
impl crate::Resettable for Data32Spec {}
