#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TWD_CTRL_EN` reader - TWD enable flag"]
pub type TwdCtrlEnR = crate::BitReader;
#[doc = "Field `TWD_CTRL_EN` writer - TWD enable flag"]
pub type TwdCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_CTRL_CLR_RX` writer - Clear RX FIFO, flag auto-clears"]
pub type TwdCtrlClrRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_CTRL_CLR_TX` writer - Clear TX FIFO, flag auto-clears"]
pub type TwdCtrlClrTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_CTRL_FSEL` reader - Bus sample clock / filter select"]
pub type TwdCtrlFselR = crate::BitReader;
#[doc = "Field `TWD_CTRL_FSEL` writer - Bus sample clock / filter select"]
pub type TwdCtrlFselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_CTRL_DEV_ADDR` reader - Device address (7-bit)"]
pub type TwdCtrlDevAddrR = crate::FieldReader;
#[doc = "Field `TWD_CTRL_DEV_ADDR` writer - Device address (7-bit)"]
pub type TwdCtrlDevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TWD_CTRL_IRQ_RX_AVAIL` reader - IRQ if RX FIFO data available"]
pub type TwdCtrlIrqRxAvailR = crate::BitReader;
#[doc = "Field `TWD_CTRL_IRQ_RX_AVAIL` writer - IRQ if RX FIFO data available"]
pub type TwdCtrlIrqRxAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_CTRL_IRQ_RX_FULL` reader - IRQ if RX FIFO full"]
pub type TwdCtrlIrqRxFullR = crate::BitReader;
#[doc = "Field `TWD_CTRL_IRQ_RX_FULL` writer - IRQ if RX FIFO full"]
pub type TwdCtrlIrqRxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_CTRL_IRQ_TX_EMPTY` reader - IRQ if TX FIFO empty"]
pub type TwdCtrlIrqTxEmptyR = crate::BitReader;
#[doc = "Field `TWD_CTRL_IRQ_TX_EMPTY` writer - IRQ if TX FIFO empty"]
pub type TwdCtrlIrqTxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_CTRL_RX_FIFO` reader - log2(TWD RX FIFO size)"]
pub type TwdCtrlRxFifoR = crate::FieldReader;
#[doc = "Field `TWD_CTRL_TX_FIFO` reader - log2(TWD TX FIFO size)"]
pub type TwdCtrlTxFifoR = crate::FieldReader;
#[doc = "Field `TWD_CTRL_RX_AVAIL` reader - RX FIFO data available"]
pub type TwdCtrlRxAvailR = crate::BitReader;
#[doc = "Field `TWD_CTRL_RX_FULL` reader - RX FIFO full"]
pub type TwdCtrlRxFullR = crate::BitReader;
#[doc = "Field `TWD_CTRL_TX_EMPTY` reader - TX FIFO empty"]
pub type TwdCtrlTxEmptyR = crate::BitReader;
#[doc = "Field `TWD_CTRL_TX_FULL` reader - TX FIFO full"]
pub type TwdCtrlTxFullR = crate::BitReader;
#[doc = "Field `TWD_CTRL_SENSE_SCL` reader - current state of the SCL bus line"]
pub type TwdCtrlSenseSclR = crate::BitReader;
#[doc = "Field `TWD_CTRL_SENSE_SDA` reader - current state of the SDA bus line"]
pub type TwdCtrlSenseSdaR = crate::BitReader;
#[doc = "Field `TWD_CTRL_BUSY` reader - bus engine is busy (transaction in progress)"]
pub type TwdCtrlBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TWD enable flag"]
    #[inline(always)]
    pub fn twd_ctrl_en(&self) -> TwdCtrlEnR {
        TwdCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Bus sample clock / filter select"]
    #[inline(always)]
    pub fn twd_ctrl_fsel(&self) -> TwdCtrlFselR {
        TwdCtrlFselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address (7-bit)"]
    #[inline(always)]
    pub fn twd_ctrl_dev_addr(&self) -> TwdCtrlDevAddrR {
        TwdCtrlDevAddrR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - IRQ if RX FIFO data available"]
    #[inline(always)]
    pub fn twd_ctrl_irq_rx_avail(&self) -> TwdCtrlIrqRxAvailR {
        TwdCtrlIrqRxAvailR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IRQ if RX FIFO full"]
    #[inline(always)]
    pub fn twd_ctrl_irq_rx_full(&self) -> TwdCtrlIrqRxFullR {
        TwdCtrlIrqRxFullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IRQ if TX FIFO empty"]
    #[inline(always)]
    pub fn twd_ctrl_irq_tx_empty(&self) -> TwdCtrlIrqTxEmptyR {
        TwdCtrlIrqTxEmptyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - log2(TWD RX FIFO size)"]
    #[inline(always)]
    pub fn twd_ctrl_rx_fifo(&self) -> TwdCtrlRxFifoR {
        TwdCtrlRxFifoR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - log2(TWD TX FIFO size)"]
    #[inline(always)]
    pub fn twd_ctrl_tx_fifo(&self) -> TwdCtrlTxFifoR {
        TwdCtrlTxFifoR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - RX FIFO data available"]
    #[inline(always)]
    pub fn twd_ctrl_rx_avail(&self) -> TwdCtrlRxAvailR {
        TwdCtrlRxAvailR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RX FIFO full"]
    #[inline(always)]
    pub fn twd_ctrl_rx_full(&self) -> TwdCtrlRxFullR {
        TwdCtrlRxFullR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TX FIFO empty"]
    #[inline(always)]
    pub fn twd_ctrl_tx_empty(&self) -> TwdCtrlTxEmptyR {
        TwdCtrlTxEmptyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TX FIFO full"]
    #[inline(always)]
    pub fn twd_ctrl_tx_full(&self) -> TwdCtrlTxFullR {
        TwdCtrlTxFullR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - current state of the SCL bus line"]
    #[inline(always)]
    pub fn twd_ctrl_sense_scl(&self) -> TwdCtrlSenseSclR {
        TwdCtrlSenseSclR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - current state of the SDA bus line"]
    #[inline(always)]
    pub fn twd_ctrl_sense_sda(&self) -> TwdCtrlSenseSdaR {
        TwdCtrlSenseSdaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - bus engine is busy (transaction in progress)"]
    #[inline(always)]
    pub fn twd_ctrl_busy(&self) -> TwdCtrlBusyR {
        TwdCtrlBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWD enable flag"]
    #[inline(always)]
    pub fn twd_ctrl_en(&mut self) -> TwdCtrlEnW<'_, CtrlSpec> {
        TwdCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear RX FIFO, flag auto-clears"]
    #[inline(always)]
    pub fn twd_ctrl_clr_rx(&mut self) -> TwdCtrlClrRxW<'_, CtrlSpec> {
        TwdCtrlClrRxW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear TX FIFO, flag auto-clears"]
    #[inline(always)]
    pub fn twd_ctrl_clr_tx(&mut self) -> TwdCtrlClrTxW<'_, CtrlSpec> {
        TwdCtrlClrTxW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus sample clock / filter select"]
    #[inline(always)]
    pub fn twd_ctrl_fsel(&mut self) -> TwdCtrlFselW<'_, CtrlSpec> {
        TwdCtrlFselW::new(self, 3)
    }
    #[doc = "Bits 4:10 - Device address (7-bit)"]
    #[inline(always)]
    pub fn twd_ctrl_dev_addr(&mut self) -> TwdCtrlDevAddrW<'_, CtrlSpec> {
        TwdCtrlDevAddrW::new(self, 4)
    }
    #[doc = "Bit 11 - IRQ if RX FIFO data available"]
    #[inline(always)]
    pub fn twd_ctrl_irq_rx_avail(&mut self) -> TwdCtrlIrqRxAvailW<'_, CtrlSpec> {
        TwdCtrlIrqRxAvailW::new(self, 11)
    }
    #[doc = "Bit 12 - IRQ if RX FIFO full"]
    #[inline(always)]
    pub fn twd_ctrl_irq_rx_full(&mut self) -> TwdCtrlIrqRxFullW<'_, CtrlSpec> {
        TwdCtrlIrqRxFullW::new(self, 12)
    }
    #[doc = "Bit 13 - IRQ if TX FIFO empty"]
    #[inline(always)]
    pub fn twd_ctrl_irq_tx_empty(&mut self) -> TwdCtrlIrqTxEmptyW<'_, CtrlSpec> {
        TwdCtrlIrqTxEmptyW::new(self, 13)
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
