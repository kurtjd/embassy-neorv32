#[doc = "Register `PORT_IN` reader"]
pub type R = crate::R<PortInSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Parallel input port\n\nYou can [`read`](crate::Reg::read) this register and get [`port_in::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortInSpec;
impl crate::RegisterSpec for PortInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_in::R`](R) reader structure"]
impl crate::Readable for PortInSpec {}
#[doc = "`reset()` method sets PORT_IN to value 0"]
impl crate::Resettable for PortInSpec {}
