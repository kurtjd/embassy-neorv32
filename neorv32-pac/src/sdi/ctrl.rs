#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SDI_CTRL_EN` reader - SDI enable"]
pub type SdiCtrlEnR = crate::BitReader;
#[doc = "Field `SDI_CTRL_EN` writer - SDI enable"]
pub type SdiCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDI_CTRL_CLR_RX` reader - Clear RX FIFO, flag auto-clears"]
pub type SdiCtrlClrRxR = crate::BitReader;
#[doc = "Field `SDI_CTRL_CLR_RX` writer - Clear RX FIFO, flag auto-clears"]
pub type SdiCtrlClrRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDI_CTRL_CLR_TX` reader - Clear TX FIFO, flag auto-clears"]
pub type SdiCtrlClrTxR = crate::BitReader;
#[doc = "Field `SDI_CTRL_CLR_TX` writer - Clear TX FIFO, flag auto-clears"]
pub type SdiCtrlClrTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDI_CTRL_FIFO` reader - log2(FIFO size)"]
pub type SdiCtrlFifoR = crate::FieldReader;
#[doc = "Field `SDI_CTRL_FIFO` writer - log2(FIFO size)"]
pub type SdiCtrlFifoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDI_CTRL_IRQ_RX_NEMPTY` reader - Fire interrupt if RX FIFO is not empty"]
pub type SdiCtrlIrqRxNemptyR = crate::BitReader;
#[doc = "Field `SDI_CTRL_IRQ_RX_NEMPTY` writer - Fire interrupt if RX FIFO is not empty"]
pub type SdiCtrlIrqRxNemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDI_CTRL_IRQ_RX_FULL` reader - Fire interrupt if RX FIFO is full"]
pub type SdiCtrlIrqRxFullR = crate::BitReader;
#[doc = "Field `SDI_CTRL_IRQ_RX_FULL` writer - Fire interrupt if RX FIFO is full"]
pub type SdiCtrlIrqRxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDI_CTRL_IRQ_TX_EMPTY` reader - Fire interrupt if TX FIFO is empty"]
pub type SdiCtrlIrqTxEmptyR = crate::BitReader;
#[doc = "Field `SDI_CTRL_IRQ_TX_EMPTY` writer - Fire interrupt if TX FIFO is empty"]
pub type SdiCtrlIrqTxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDI_CTRL_RX_EMPTY` reader - RX FIFO empty"]
pub type SdiCtrlRxEmptyR = crate::BitReader;
#[doc = "Field `SDI_CTRL_RX_FULL` reader - RX FIFO full"]
pub type SdiCtrlRxFullR = crate::BitReader;
#[doc = "Field `SDI_CTRL_TX_EMPTY` reader - TX FIFO empty"]
pub type SdiCtrlTxEmptyR = crate::BitReader;
#[doc = "Field `SDI_CTRL_TX_FULL` reader - TX FIFO full"]
pub type SdiCtrlTxFullR = crate::BitReader;
#[doc = "Field `SDI_CTRL_CS_ACTIVE` reader - Chip-select line is active when set"]
pub type SdiCtrlCsActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SDI enable"]
    #[inline(always)]
    pub fn sdi_ctrl_en(&self) -> SdiCtrlEnR {
        SdiCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear RX FIFO, flag auto-clears"]
    #[inline(always)]
    pub fn sdi_ctrl_clr_rx(&self) -> SdiCtrlClrRxR {
        SdiCtrlClrRxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear TX FIFO, flag auto-clears"]
    #[inline(always)]
    pub fn sdi_ctrl_clr_tx(&self) -> SdiCtrlClrTxR {
        SdiCtrlClrTxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - log2(FIFO size)"]
    #[inline(always)]
    pub fn sdi_ctrl_fifo(&self) -> SdiCtrlFifoR {
        SdiCtrlFifoR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Fire interrupt if RX FIFO is not empty"]
    #[inline(always)]
    pub fn sdi_ctrl_irq_rx_nempty(&self) -> SdiCtrlIrqRxNemptyR {
        SdiCtrlIrqRxNemptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fire interrupt if RX FIFO is full"]
    #[inline(always)]
    pub fn sdi_ctrl_irq_rx_full(&self) -> SdiCtrlIrqRxFullR {
        SdiCtrlIrqRxFullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fire interrupt if TX FIFO is empty"]
    #[inline(always)]
    pub fn sdi_ctrl_irq_tx_empty(&self) -> SdiCtrlIrqTxEmptyR {
        SdiCtrlIrqTxEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - RX FIFO empty"]
    #[inline(always)]
    pub fn sdi_ctrl_rx_empty(&self) -> SdiCtrlRxEmptyR {
        SdiCtrlRxEmptyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RX FIFO full"]
    #[inline(always)]
    pub fn sdi_ctrl_rx_full(&self) -> SdiCtrlRxFullR {
        SdiCtrlRxFullR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TX FIFO empty"]
    #[inline(always)]
    pub fn sdi_ctrl_tx_empty(&self) -> SdiCtrlTxEmptyR {
        SdiCtrlTxEmptyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - TX FIFO full"]
    #[inline(always)]
    pub fn sdi_ctrl_tx_full(&self) -> SdiCtrlTxFullR {
        SdiCtrlTxFullR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Chip-select line is active when set"]
    #[inline(always)]
    pub fn sdi_ctrl_cs_active(&self) -> SdiCtrlCsActiveR {
        SdiCtrlCsActiveR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDI enable"]
    #[inline(always)]
    pub fn sdi_ctrl_en(&mut self) -> SdiCtrlEnW<'_, CtrlSpec> {
        SdiCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear RX FIFO, flag auto-clears"]
    #[inline(always)]
    pub fn sdi_ctrl_clr_rx(&mut self) -> SdiCtrlClrRxW<'_, CtrlSpec> {
        SdiCtrlClrRxW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear TX FIFO, flag auto-clears"]
    #[inline(always)]
    pub fn sdi_ctrl_clr_tx(&mut self) -> SdiCtrlClrTxW<'_, CtrlSpec> {
        SdiCtrlClrTxW::new(self, 2)
    }
    #[doc = "Bits 4:7 - log2(FIFO size)"]
    #[inline(always)]
    pub fn sdi_ctrl_fifo(&mut self) -> SdiCtrlFifoW<'_, CtrlSpec> {
        SdiCtrlFifoW::new(self, 4)
    }
    #[doc = "Bit 16 - Fire interrupt if RX FIFO is not empty"]
    #[inline(always)]
    pub fn sdi_ctrl_irq_rx_nempty(&mut self) -> SdiCtrlIrqRxNemptyW<'_, CtrlSpec> {
        SdiCtrlIrqRxNemptyW::new(self, 16)
    }
    #[doc = "Bit 17 - Fire interrupt if RX FIFO is full"]
    #[inline(always)]
    pub fn sdi_ctrl_irq_rx_full(&mut self) -> SdiCtrlIrqRxFullW<'_, CtrlSpec> {
        SdiCtrlIrqRxFullW::new(self, 17)
    }
    #[doc = "Bit 18 - Fire interrupt if TX FIFO is empty"]
    #[inline(always)]
    pub fn sdi_ctrl_irq_tx_empty(&mut self) -> SdiCtrlIrqTxEmptyW<'_, CtrlSpec> {
        SdiCtrlIrqTxEmptyW::new(self, 18)
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
