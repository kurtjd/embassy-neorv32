#[doc = "Register `STROBE` writer"]
pub type W = crate::W<StrobeSpec>;
impl core::fmt::Debug for crate::generic::Reg<StrobeSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Write any value to send STROBE command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`strobe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrobeSpec;
impl crate::RegisterSpec for StrobeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`strobe::W`](W) writer structure"]
impl crate::Writable for StrobeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STROBE to value 0"]
impl crate::Resettable for StrobeSpec {}
