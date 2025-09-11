#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TRACER_CTRL_EN` reader - TRACER enable, reset module when 0"]
pub type TracerCtrlEnR = crate::BitReader;
#[doc = "Field `TRACER_CTRL_EN` writer - TRACER enable, reset module when 0"]
pub type TracerCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACER_CTRL_HSEL` reader - Hart select for tracing"]
pub type TracerCtrlHselR = crate::BitReader;
#[doc = "Field `TRACER_CTRL_HSEL` writer - Hart select for tracing"]
pub type TracerCtrlHselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACER_CTRL_START` writer - Start tracing, flag always reads as zero"]
pub type TracerCtrlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACER_CTRL_STOP` writer - Manually stop tracing, flag always reads as zero"]
pub type TracerCtrlStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACER_CTRL_RUN` reader - Tracing in progress when set"]
pub type TracerCtrlRunR = crate::BitReader;
#[doc = "Field `TRACER_CTRL_AVAIL` reader - Trace data available when set"]
pub type TracerCtrlAvailR = crate::BitReader;
#[doc = "Field `TRACER_CTRL_IRQ_CLR` writer - Clear pending interrupt when writing 1"]
pub type TracerCtrlIrqClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACER_CTRL_TBM` reader - log2 of trace buffer depth"]
pub type TracerCtrlTbmR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TRACER enable, reset module when 0"]
    #[inline(always)]
    pub fn tracer_ctrl_en(&self) -> TracerCtrlEnR {
        TracerCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hart select for tracing"]
    #[inline(always)]
    pub fn tracer_ctrl_hsel(&self) -> TracerCtrlHselR {
        TracerCtrlHselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Tracing in progress when set"]
    #[inline(always)]
    pub fn tracer_ctrl_run(&self) -> TracerCtrlRunR {
        TracerCtrlRunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace data available when set"]
    #[inline(always)]
    pub fn tracer_ctrl_avail(&self) -> TracerCtrlAvailR {
        TracerCtrlAvailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:10 - log2 of trace buffer depth"]
    #[inline(always)]
    pub fn tracer_ctrl_tbm(&self) -> TracerCtrlTbmR {
        TracerCtrlTbmR::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TRACER enable, reset module when 0"]
    #[inline(always)]
    pub fn tracer_ctrl_en(&mut self) -> TracerCtrlEnW<'_, CtrlSpec> {
        TracerCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Hart select for tracing"]
    #[inline(always)]
    pub fn tracer_ctrl_hsel(&mut self) -> TracerCtrlHselW<'_, CtrlSpec> {
        TracerCtrlHselW::new(self, 1)
    }
    #[doc = "Bit 2 - Start tracing, flag always reads as zero"]
    #[inline(always)]
    pub fn tracer_ctrl_start(&mut self) -> TracerCtrlStartW<'_, CtrlSpec> {
        TracerCtrlStartW::new(self, 2)
    }
    #[doc = "Bit 3 - Manually stop tracing, flag always reads as zero"]
    #[inline(always)]
    pub fn tracer_ctrl_stop(&mut self) -> TracerCtrlStopW<'_, CtrlSpec> {
        TracerCtrlStopW::new(self, 3)
    }
    #[doc = "Bit 6 - Clear pending interrupt when writing 1"]
    #[inline(always)]
    pub fn tracer_ctrl_irq_clr(&mut self) -> TracerCtrlIrqClrW<'_, CtrlSpec> {
        TracerCtrlIrqClrW::new(self, 6)
    }
}
#[doc = "Control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
