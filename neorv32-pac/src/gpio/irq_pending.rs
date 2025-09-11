#[doc = "Register `IRQ_PENDING` reader"]
pub type R = crate::R<IrqPendingSpec>;
#[doc = "Register `IRQ_PENDING` writer"]
pub type W = crate::W<IrqPendingSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt pending for each input pin; cleared by writing zero\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqPendingSpec;
impl crate::RegisterSpec for IrqPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_pending::R`](R) reader structure"]
impl crate::Readable for IrqPendingSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_pending::W`](W) writer structure"]
impl crate::Writable for IrqPendingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_PENDING to value 0"]
impl crate::Resettable for IrqPendingSpec {}
