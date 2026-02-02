#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `UART_DATA_RTX` reader - Receive/transmit data"]
pub type UartDataRtxR = crate::FieldReader;
#[doc = "Field `UART_DATA_RTX` writer - Receive/transmit data"]
pub type UartDataRtxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART_DATA_RX_FIFO` reader - log2(RX FIFO size)"]
pub type UartDataRxFifoR = crate::FieldReader;
#[doc = "Field `UART_DATA_TX_FIFO` reader - log2(TX FIFO size)"]
pub type UartDataTxFifoR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive/transmit data"]
    #[inline(always)]
    pub fn uart_data_rtx(&self) -> UartDataRtxR {
        UartDataRtxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - log2(RX FIFO size)"]
    #[inline(always)]
    pub fn uart_data_rx_fifo(&self) -> UartDataRxFifoR {
        UartDataRxFifoR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - log2(TX FIFO size)"]
    #[inline(always)]
    pub fn uart_data_tx_fifo(&self) -> UartDataTxFifoR {
        UartDataTxFifoR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive/transmit data"]
    #[inline(always)]
    pub fn uart_data_rtx(&mut self) -> UartDataRtxW<'_, DataSpec> {
        UartDataRtxW::new(self, 0)
    }
}
#[doc = "RTX data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
