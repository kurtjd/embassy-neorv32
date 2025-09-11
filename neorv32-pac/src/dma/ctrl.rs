#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `DMA_CTRL_EN` reader - DMA enable flag"]
pub type DmaCtrlEnR = crate::BitReader;
#[doc = "Field `DMA_CTRL_EN` writer - DMA enable flag"]
pub type DmaCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CTRL_START` writer - Start programmed DMA transfer(s)"]
pub type DmaCtrlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CTRL_FIFO` reader - log2(descriptor FIFO depth)"]
pub type DmaCtrlFifoR = crate::FieldReader;
#[doc = "Field `DMA_CTRL_ACK` writer - Write 1 to clear ERROR and DONE flags"]
pub type DmaCtrlAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CTRL_DEMPTY` reader - Descriptor FIFO is empty"]
pub type DmaCtrlDemptyR = crate::BitReader;
#[doc = "Field `DMA_CTRL_DFULL` reader - Descriptor FIFO is full"]
pub type DmaCtrlDfullR = crate::BitReader;
#[doc = "Field `DMA_CTRL_ERROR` reader - Error during last transfer"]
pub type DmaCtrlErrorR = crate::BitReader;
#[doc = "Field `DMA_CTRL_DONE` reader - Transfer(s) done"]
pub type DmaCtrlDoneR = crate::BitReader;
#[doc = "Field `DMA_CTRL_BUSY` reader - Transfer(s) in progress"]
pub type DmaCtrlBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA enable flag"]
    #[inline(always)]
    pub fn dma_ctrl_en(&self) -> DmaCtrlEnR {
        DmaCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:19 - log2(descriptor FIFO depth)"]
    #[inline(always)]
    pub fn dma_ctrl_fifo(&self) -> DmaCtrlFifoR {
        DmaCtrlFifoR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Descriptor FIFO is empty"]
    #[inline(always)]
    pub fn dma_ctrl_dempty(&self) -> DmaCtrlDemptyR {
        DmaCtrlDemptyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Descriptor FIFO is full"]
    #[inline(always)]
    pub fn dma_ctrl_dfull(&self) -> DmaCtrlDfullR {
        DmaCtrlDfullR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Error during last transfer"]
    #[inline(always)]
    pub fn dma_ctrl_error(&self) -> DmaCtrlErrorR {
        DmaCtrlErrorR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transfer(s) done"]
    #[inline(always)]
    pub fn dma_ctrl_done(&self) -> DmaCtrlDoneR {
        DmaCtrlDoneR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transfer(s) in progress"]
    #[inline(always)]
    pub fn dma_ctrl_busy(&self) -> DmaCtrlBusyR {
        DmaCtrlBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable flag"]
    #[inline(always)]
    pub fn dma_ctrl_en(&mut self) -> DmaCtrlEnW<'_, CtrlSpec> {
        DmaCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Start programmed DMA transfer(s)"]
    #[inline(always)]
    pub fn dma_ctrl_start(&mut self) -> DmaCtrlStartW<'_, CtrlSpec> {
        DmaCtrlStartW::new(self, 1)
    }
    #[doc = "Bit 26 - Write 1 to clear ERROR and DONE flags"]
    #[inline(always)]
    pub fn dma_ctrl_ack(&mut self) -> DmaCtrlAckW<'_, CtrlSpec> {
        DmaCtrlAckW::new(self, 26)
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
