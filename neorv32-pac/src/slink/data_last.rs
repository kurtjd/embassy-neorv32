#[doc = "Register `DATA_LAST` reader"]
pub type R = crate::R<DataLastSpec>;
#[doc = "Register `DATA_LAST` writer"]
pub type W = crate::W<DataLastSpec>;
impl core::fmt::Debug for crate::generic::Reg<DataLastSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "TX data register (plus TX end-of-stream delimiter)\n\nYou can [`read`](crate::Reg::read) this register and get [`data_last::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_last::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct DataLastSpec;
impl crate::RegisterSpec for DataLastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_last::R`](R) reader structure"]
impl crate::Readable for DataLastSpec {}
#[doc = "`write(|w| ..)` method takes [`data_last::W`](W) writer structure"]
impl crate::Writable for DataLastSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_LAST to value 0"]
impl crate::Resettable for DataLastSpec {}
