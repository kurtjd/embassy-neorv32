#[doc = "Register `COUNT` reader"]
pub type R = crate::R<CountSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountSpec;
impl crate::RegisterSpec for CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for CountSpec {}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for CountSpec {}
