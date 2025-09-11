#[doc = "Register `STOP_ADDR` reader"]
pub type R = crate::R<StopAddrSpec>;
#[doc = "Register `STOP_ADDR` writer"]
pub type W = crate::W<StopAddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Stop tracing when reaching this address, set to all-zero to disable auto-stopping\n\nYou can [`read`](crate::Reg::read) this register and get [`stop_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StopAddrSpec;
impl crate::RegisterSpec for StopAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stop_addr::R`](R) reader structure"]
impl crate::Readable for StopAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`stop_addr::W`](W) writer structure"]
impl crate::Writable for StopAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STOP_ADDR to value 0"]
impl crate::Resettable for StopAddrSpec {}
