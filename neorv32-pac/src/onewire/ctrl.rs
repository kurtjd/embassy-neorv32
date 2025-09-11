#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ONEWIRE_CTRL_EN` reader - ONEWIRE controller enable"]
pub type OnewireCtrlEnR = crate::BitReader;
#[doc = "Field `ONEWIRE_CTRL_EN` writer - ONEWIRE controller enable"]
pub type OnewireCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEWIRE_CTRL_CLEAR` writer - Clear RXT FIFO, auto-clears"]
pub type OnewireCtrlClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEWIRE_CTRL_PRSC` reader - Clock prescaler select"]
pub type OnewireCtrlPrscR = crate::FieldReader;
#[doc = "Field `ONEWIRE_CTRL_PRSC` writer - Clock prescaler select"]
pub type OnewireCtrlPrscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ONEWIRE_CTRL_CLKDIV` reader - Clock divider"]
pub type OnewireCtrlClkdivR = crate::FieldReader;
#[doc = "Field `ONEWIRE_CTRL_CLKDIV` writer - Clock divider"]
pub type OnewireCtrlClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ONEWIRE_CTRL_FIFO` reader - log2(ONEWIRE FIFO size)"]
pub type OnewireCtrlFifoR = crate::FieldReader;
#[doc = "Field `ONEWIRE_CTRL_TX_FULL` reader - TX FIFO full"]
pub type OnewireCtrlTxFullR = crate::BitReader;
#[doc = "Field `ONEWIRE_CTRL_RX_AVAIL` reader - RX FIFO data available"]
pub type OnewireCtrlRxAvailR = crate::BitReader;
#[doc = "Field `ONEWIRE_CTRL_SENSE` reader - Current state of the 1-wire bus line"]
pub type OnewireCtrlSenseR = crate::BitReader;
#[doc = "Field `ONEWIRE_CTRL_BUSY` reader - Operation in progress when set"]
pub type OnewireCtrlBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ONEWIRE controller enable"]
    #[inline(always)]
    pub fn onewire_ctrl_en(&self) -> OnewireCtrlEnR {
        OnewireCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn onewire_ctrl_prsc(&self) -> OnewireCtrlPrscR {
        OnewireCtrlPrscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:11 - Clock divider"]
    #[inline(always)]
    pub fn onewire_ctrl_clkdiv(&self) -> OnewireCtrlClkdivR {
        OnewireCtrlClkdivR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 15:18 - log2(ONEWIRE FIFO size)"]
    #[inline(always)]
    pub fn onewire_ctrl_fifo(&self) -> OnewireCtrlFifoR {
        OnewireCtrlFifoR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - TX FIFO full"]
    #[inline(always)]
    pub fn onewire_ctrl_tx_full(&self) -> OnewireCtrlTxFullR {
        OnewireCtrlTxFullR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RX FIFO data available"]
    #[inline(always)]
    pub fn onewire_ctrl_rx_avail(&self) -> OnewireCtrlRxAvailR {
        OnewireCtrlRxAvailR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Current state of the 1-wire bus line"]
    #[inline(always)]
    pub fn onewire_ctrl_sense(&self) -> OnewireCtrlSenseR {
        OnewireCtrlSenseR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Operation in progress when set"]
    #[inline(always)]
    pub fn onewire_ctrl_busy(&self) -> OnewireCtrlBusyR {
        OnewireCtrlBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ONEWIRE controller enable"]
    #[inline(always)]
    pub fn onewire_ctrl_en(&mut self) -> OnewireCtrlEnW<'_, CtrlSpec> {
        OnewireCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear RXT FIFO, auto-clears"]
    #[inline(always)]
    pub fn onewire_ctrl_clear(&mut self) -> OnewireCtrlClearW<'_, CtrlSpec> {
        OnewireCtrlClearW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn onewire_ctrl_prsc(&mut self) -> OnewireCtrlPrscW<'_, CtrlSpec> {
        OnewireCtrlPrscW::new(self, 2)
    }
    #[doc = "Bits 4:11 - Clock divider"]
    #[inline(always)]
    pub fn onewire_ctrl_clkdiv(&mut self) -> OnewireCtrlClkdivW<'_, CtrlSpec> {
        OnewireCtrlClkdivW::new(self, 4)
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
