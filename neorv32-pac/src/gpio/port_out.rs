#[doc = "Register `PORT_OUT` reader"]
pub type R = crate::R<PortOutSpec>;
#[doc = "Register `PORT_OUT` writer"]
pub type W = crate::W<PortOutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Parallel output port\n\nYou can [`read`](crate::Reg::read) this register and get [`port_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortOutSpec;
impl crate::RegisterSpec for PortOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_out::R`](R) reader structure"]
impl crate::Readable for PortOutSpec {}
#[doc = "`write(|w| ..)` method takes [`port_out::W`](W) writer structure"]
impl crate::Writable for PortOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORT_OUT to value 0"]
impl crate::Resettable for PortOutSpec {}
