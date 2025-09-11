#[doc = "Register `IRQ_TYPE` reader"]
pub type R = crate::R<IrqTypeSpec>;
#[doc = "Register `IRQ_TYPE` writer"]
pub type W = crate::W<IrqTypeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt trigger type (level/edge) for each input pin\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqTypeSpec;
impl crate::RegisterSpec for IrqTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_type::R`](R) reader structure"]
impl crate::Readable for IrqTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_type::W`](W) writer structure"]
impl crate::Writable for IrqTypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_TYPE to value 0"]
impl crate::Resettable for IrqTypeSpec {}
