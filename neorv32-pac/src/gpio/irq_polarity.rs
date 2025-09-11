#[doc = "Register `IRQ_POLARITY` reader"]
pub type R = crate::R<IrqPolaritySpec>;
#[doc = "Register `IRQ_POLARITY` writer"]
pub type W = crate::W<IrqPolaritySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt trigger polarity (rising/falling or high/low) for each input pin\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqPolaritySpec;
impl crate::RegisterSpec for IrqPolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_polarity::R`](R) reader structure"]
impl crate::Readable for IrqPolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`irq_polarity::W`](W) writer structure"]
impl crate::Writable for IrqPolaritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_POLARITY to value 0"]
impl crate::Resettable for IrqPolaritySpec {}
