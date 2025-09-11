#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TWI_CTRL_EN` reader - TWI enable flag"]
pub type TwiCtrlEnR = crate::BitReader;
#[doc = "Field `TWI_CTRL_EN` writer - TWI enable flag"]
pub type TwiCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI_CTRL_PRSC` reader - Clock prescaler select"]
pub type TwiCtrlPrscR = crate::FieldReader;
#[doc = "Field `TWI_CTRL_PRSC` writer - Clock prescaler select"]
pub type TwiCtrlPrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TWI_CTRL_CDIV` reader - TWI clock divider"]
pub type TwiCtrlCdivR = crate::FieldReader;
#[doc = "Field `TWI_CTRL_CDIV` writer - TWI clock divider"]
pub type TwiCtrlCdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TWI_CTRL_CLKSTR` reader - Enable (allow) clock stretching"]
pub type TwiCtrlClkstrR = crate::BitReader;
#[doc = "Field `TWI_CTRL_CLKSTR` writer - Enable (allow) clock stretching"]
pub type TwiCtrlClkstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI_CTRL_FIFO` reader - log2(TWI FIFO size)"]
pub type TwiCtrlFifoR = crate::FieldReader;
#[doc = "Field `TWI_CTRL_SENSE_SCL` reader - current state of the SCL bus line"]
pub type TwiCtrlSenseSclR = crate::BitReader;
#[doc = "Field `TWI_CTRL_SENSE_SDA` reader - current state of the SDA bus line"]
pub type TwiCtrlSenseSdaR = crate::BitReader;
#[doc = "Field `TWI_CTRL_TX_FULL` reader - TX FIFO full"]
pub type TwiCtrlTxFullR = crate::BitReader;
#[doc = "Field `TWI_CTRL_RX_AVAIL` reader - RX FIFO not empty (data available)"]
pub type TwiCtrlRxAvailR = crate::BitReader;
#[doc = "Field `TWI_CTRL_BUSY` reader - Bus engine busy of TX FIFO not empty"]
pub type TwiCtrlBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TWI enable flag"]
    #[inline(always)]
    pub fn twi_ctrl_en(&self) -> TwiCtrlEnR {
        TwiCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn twi_ctrl_prsc(&self) -> TwiCtrlPrscR {
        TwiCtrlPrscR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - TWI clock divider"]
    #[inline(always)]
    pub fn twi_ctrl_cdiv(&self) -> TwiCtrlCdivR {
        TwiCtrlCdivR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable (allow) clock stretching"]
    #[inline(always)]
    pub fn twi_ctrl_clkstr(&self) -> TwiCtrlClkstrR {
        TwiCtrlClkstrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 15:18 - log2(TWI FIFO size)"]
    #[inline(always)]
    pub fn twi_ctrl_fifo(&self) -> TwiCtrlFifoR {
        TwiCtrlFifoR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - current state of the SCL bus line"]
    #[inline(always)]
    pub fn twi_ctrl_sense_scl(&self) -> TwiCtrlSenseSclR {
        TwiCtrlSenseSclR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - current state of the SDA bus line"]
    #[inline(always)]
    pub fn twi_ctrl_sense_sda(&self) -> TwiCtrlSenseSdaR {
        TwiCtrlSenseSdaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TX FIFO full"]
    #[inline(always)]
    pub fn twi_ctrl_tx_full(&self) -> TwiCtrlTxFullR {
        TwiCtrlTxFullR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RX FIFO not empty (data available)"]
    #[inline(always)]
    pub fn twi_ctrl_rx_avail(&self) -> TwiCtrlRxAvailR {
        TwiCtrlRxAvailR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus engine busy of TX FIFO not empty"]
    #[inline(always)]
    pub fn twi_ctrl_busy(&self) -> TwiCtrlBusyR {
        TwiCtrlBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI enable flag"]
    #[inline(always)]
    pub fn twi_ctrl_en(&mut self) -> TwiCtrlEnW<'_, CtrlSpec> {
        TwiCtrlEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn twi_ctrl_prsc(&mut self) -> TwiCtrlPrscW<'_, CtrlSpec> {
        TwiCtrlPrscW::new(self, 1)
    }
    #[doc = "Bits 4:7 - TWI clock divider"]
    #[inline(always)]
    pub fn twi_ctrl_cdiv(&mut self) -> TwiCtrlCdivW<'_, CtrlSpec> {
        TwiCtrlCdivW::new(self, 4)
    }
    #[doc = "Bit 8 - Enable (allow) clock stretching"]
    #[inline(always)]
    pub fn twi_ctrl_clkstr(&mut self) -> TwiCtrlClkstrW<'_, CtrlSpec> {
        TwiCtrlClkstrW::new(self, 8)
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
