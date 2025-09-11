#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `GPTMR_CTRL_EN` reader - Timer enable flag"]
pub type GptmrCtrlEnR = crate::BitReader;
#[doc = "Field `GPTMR_CTRL_EN` writer - Timer enable flag"]
pub type GptmrCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTMR_CTRL_PRSC` reader - Clock prescaler select"]
pub type GptmrCtrlPrscR = crate::FieldReader;
#[doc = "Field `GPTMR_CTRL_PRSC` writer - Clock prescaler select"]
pub type GptmrCtrlPrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPTMR_CTRL_IRQ_CLR` writer - Set to clear timer-match interrupt"]
pub type GptmrCtrlIrqClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTMR_CTRL_IRQ_PND` reader - Timer-match interrupt pending"]
pub type GptmrCtrlIrqPndR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer enable flag"]
    #[inline(always)]
    pub fn gptmr_ctrl_en(&self) -> GptmrCtrlEnR {
        GptmrCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn gptmr_ctrl_prsc(&self) -> GptmrCtrlPrscR {
        GptmrCtrlPrscR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 31 - Timer-match interrupt pending"]
    #[inline(always)]
    pub fn gptmr_ctrl_irq_pnd(&self) -> GptmrCtrlIrqPndR {
        GptmrCtrlIrqPndR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable flag"]
    #[inline(always)]
    pub fn gptmr_ctrl_en(&mut self) -> GptmrCtrlEnW<'_, CtrlSpec> {
        GptmrCtrlEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn gptmr_ctrl_prsc(&mut self) -> GptmrCtrlPrscW<'_, CtrlSpec> {
        GptmrCtrlPrscW::new(self, 1)
    }
    #[doc = "Bit 30 - Set to clear timer-match interrupt"]
    #[inline(always)]
    pub fn gptmr_ctrl_irq_clr(&mut self) -> GptmrCtrlIrqClrW<'_, CtrlSpec> {
        GptmrCtrlIrqClrW::new(self, 30)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
