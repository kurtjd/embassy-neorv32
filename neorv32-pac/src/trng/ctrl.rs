#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TRNG_CTRL_EN` reader - TRNG enable flag"]
pub type TrngCtrlEnR = crate::BitReader;
#[doc = "Field `TRNG_CTRL_EN` writer - TRNG enable flag"]
pub type TrngCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_CTRL_FIFO_CLR` writer - Clear data FIFO when set (flag auto clears)"]
pub type TrngCtrlFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_CTRL_FIFO_SIZE` reader - log2(TRNG FIFO size)"]
pub type TrngCtrlFifoSizeR = crate::FieldReader;
#[doc = "Field `TRNG_CTRL_SIM_MODE` reader - TRNG simulation mode (PRNG!) active"]
pub type TrngCtrlSimModeR = crate::BitReader;
#[doc = "Field `TRNG_CTRL_AVAIL` reader - Random data available"]
pub type TrngCtrlAvailR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TRNG enable flag"]
    #[inline(always)]
    pub fn trng_ctrl_en(&self) -> TrngCtrlEnR {
        TrngCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:5 - log2(TRNG FIFO size)"]
    #[inline(always)]
    pub fn trng_ctrl_fifo_size(&self) -> TrngCtrlFifoSizeR {
        TrngCtrlFifoSizeR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - TRNG simulation mode (PRNG!) active"]
    #[inline(always)]
    pub fn trng_ctrl_sim_mode(&self) -> TrngCtrlSimModeR {
        TrngCtrlSimModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Random data available"]
    #[inline(always)]
    pub fn trng_ctrl_avail(&self) -> TrngCtrlAvailR {
        TrngCtrlAvailR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG enable flag"]
    #[inline(always)]
    pub fn trng_ctrl_en(&mut self) -> TrngCtrlEnW<'_, CtrlSpec> {
        TrngCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear data FIFO when set (flag auto clears)"]
    #[inline(always)]
    pub fn trng_ctrl_fifo_clr(&mut self) -> TrngCtrlFifoClrW<'_, CtrlSpec> {
        TrngCtrlFifoClrW::new(self, 1)
    }
}
#[doc = "Control and data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
