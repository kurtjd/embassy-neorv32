#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SLINK_CTRL_EN` reader - SLINK enable flag"]
pub type SlinkCtrlEnR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_EN` writer - SLINK enable flag"]
pub type SlinkCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLINK_CTRL_RX_EMPTY` reader - RX FIFO empty"]
pub type SlinkCtrlRxEmptyR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_RX_FULL` reader - RX FIFO full"]
pub type SlinkCtrlRxFullR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_TX_EMPTY` reader - TX FIFO empty"]
pub type SlinkCtrlTxEmptyR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_TX_FULL` reader - TX FIFO full"]
pub type SlinkCtrlTxFullR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_RX_LAST` reader - RX link end-of-stream delimiter"]
pub type SlinkCtrlRxLastR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_RX_LAST` writer - RX link end-of-stream delimiter"]
pub type SlinkCtrlRxLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLINK_CTRL_IRQ_RX_NEMPTY` reader - Interrupt if RX FIFO not empty"]
pub type SlinkCtrlIrqRxNemptyR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_IRQ_RX_NEMPTY` writer - Interrupt if RX FIFO not empty"]
pub type SlinkCtrlIrqRxNemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLINK_CTRL_IRQ_RX_FULL` reader - Interrupt if RX FIFO full"]
pub type SlinkCtrlIrqRxFullR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_IRQ_RX_FULL` writer - Interrupt if RX FIFO full"]
pub type SlinkCtrlIrqRxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLINK_CTRL_IRQ_TX_EMPTY` reader - Interrupt if TX FIFO empty"]
pub type SlinkCtrlIrqTxEmptyR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_IRQ_TX_EMPTY` writer - Interrupt if TX FIFO empty"]
pub type SlinkCtrlIrqTxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLINK_CTRL_IRQ_TX_NFULL` reader - Interrupt if TX FIFO not full"]
pub type SlinkCtrlIrqTxNfullR = crate::BitReader;
#[doc = "Field `SLINK_CTRL_IRQ_TX_NFULL` writer - Interrupt if TX FIFO not full"]
pub type SlinkCtrlIrqTxNfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLINK_CTRL_RX_FIFO` reader - log2(RX FIFO size)"]
pub type SlinkCtrlRxFifoR = crate::FieldReader;
#[doc = "Field `SLINK_CTRL_TX_FIFO` reader - log2(TX FIFO size)"]
pub type SlinkCtrlTxFifoR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SLINK enable flag"]
    #[inline(always)]
    pub fn slink_ctrl_en(&self) -> SlinkCtrlEnR {
        SlinkCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO empty"]
    #[inline(always)]
    pub fn slink_ctrl_rx_empty(&self) -> SlinkCtrlRxEmptyR {
        SlinkCtrlRxEmptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO full"]
    #[inline(always)]
    pub fn slink_ctrl_rx_full(&self) -> SlinkCtrlRxFullR {
        SlinkCtrlRxFullR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX FIFO empty"]
    #[inline(always)]
    pub fn slink_ctrl_tx_empty(&self) -> SlinkCtrlTxEmptyR {
        SlinkCtrlTxEmptyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX FIFO full"]
    #[inline(always)]
    pub fn slink_ctrl_tx_full(&self) -> SlinkCtrlTxFullR {
        SlinkCtrlTxFullR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX link end-of-stream delimiter"]
    #[inline(always)]
    pub fn slink_ctrl_rx_last(&self) -> SlinkCtrlRxLastR {
        SlinkCtrlRxLastR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt if RX FIFO not empty"]
    #[inline(always)]
    pub fn slink_ctrl_irq_rx_nempty(&self) -> SlinkCtrlIrqRxNemptyR {
        SlinkCtrlIrqRxNemptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt if RX FIFO full"]
    #[inline(always)]
    pub fn slink_ctrl_irq_rx_full(&self) -> SlinkCtrlIrqRxFullR {
        SlinkCtrlIrqRxFullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt if TX FIFO empty"]
    #[inline(always)]
    pub fn slink_ctrl_irq_tx_empty(&self) -> SlinkCtrlIrqTxEmptyR {
        SlinkCtrlIrqTxEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt if TX FIFO not full"]
    #[inline(always)]
    pub fn slink_ctrl_irq_tx_nfull(&self) -> SlinkCtrlIrqTxNfullR {
        SlinkCtrlIrqTxNfullR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:27 - log2(RX FIFO size)"]
    #[inline(always)]
    pub fn slink_ctrl_rx_fifo(&self) -> SlinkCtrlRxFifoR {
        SlinkCtrlRxFifoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - log2(TX FIFO size)"]
    #[inline(always)]
    pub fn slink_ctrl_tx_fifo(&self) -> SlinkCtrlTxFifoR {
        SlinkCtrlTxFifoR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SLINK enable flag"]
    #[inline(always)]
    pub fn slink_ctrl_en(&mut self) -> SlinkCtrlEnW<'_, CtrlSpec> {
        SlinkCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 12 - RX link end-of-stream delimiter"]
    #[inline(always)]
    pub fn slink_ctrl_rx_last(&mut self) -> SlinkCtrlRxLastW<'_, CtrlSpec> {
        SlinkCtrlRxLastW::new(self, 12)
    }
    #[doc = "Bit 16 - Interrupt if RX FIFO not empty"]
    #[inline(always)]
    pub fn slink_ctrl_irq_rx_nempty(&mut self) -> SlinkCtrlIrqRxNemptyW<'_, CtrlSpec> {
        SlinkCtrlIrqRxNemptyW::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt if RX FIFO full"]
    #[inline(always)]
    pub fn slink_ctrl_irq_rx_full(&mut self) -> SlinkCtrlIrqRxFullW<'_, CtrlSpec> {
        SlinkCtrlIrqRxFullW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt if TX FIFO empty"]
    #[inline(always)]
    pub fn slink_ctrl_irq_tx_empty(&mut self) -> SlinkCtrlIrqTxEmptyW<'_, CtrlSpec> {
        SlinkCtrlIrqTxEmptyW::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt if TX FIFO not full"]
    #[inline(always)]
    pub fn slink_ctrl_irq_tx_nfull(&mut self) -> SlinkCtrlIrqTxNfullW<'_, CtrlSpec> {
        SlinkCtrlIrqTxNfullW::new(self, 19)
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
