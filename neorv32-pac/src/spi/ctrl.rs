#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SPI_CTRL_EN` reader - SPI enable flag"]
pub type SpiCtrlEnR = crate::BitReader;
#[doc = "Field `SPI_CTRL_EN` writer - SPI enable flag"]
pub type SpiCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CTRL_CPHA` reader - Clock phase"]
pub type SpiCtrlCphaR = crate::BitReader;
#[doc = "Field `SPI_CTRL_CPHA` writer - Clock phase"]
pub type SpiCtrlCphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CTRL_CPOL` reader - Clock polarity"]
pub type SpiCtrlCpolR = crate::BitReader;
#[doc = "Field `SPI_CTRL_CPOL` writer - Clock polarity"]
pub type SpiCtrlCpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CTRL_PRSC` reader - Clock prescaler select"]
pub type SpiCtrlPrscR = crate::FieldReader;
#[doc = "Field `SPI_CTRL_PRSC` writer - Clock prescaler select"]
pub type SpiCtrlPrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_CTRL_CDIV` reader - SPI clock divider"]
pub type SpiCtrlCdivR = crate::FieldReader;
#[doc = "Field `SPI_CTRL_CDIV` writer - SPI clock divider"]
pub type SpiCtrlCdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI_CTRL_RX_AVAIL` reader - RX FIFO data available (RX FIFO not empty)"]
pub type SpiCtrlRxAvailR = crate::BitReader;
#[doc = "Field `SPI_CTRL_TX_EMPTY` reader - TX FIFO is empty"]
pub type SpiCtrlTxEmptyR = crate::BitReader;
#[doc = "Field `SPI_CTRL_TX_FULL` reader - TX FIFO is full"]
pub type SpiCtrlTxFullR = crate::BitReader;
#[doc = "Field `SPI_CTRL_FIFO` reader - log2(FIFO size)"]
pub type SpiCtrlFifoR = crate::FieldReader;
#[doc = "Field `SPI_CS_ACTIVE` reader - Set if a chip-select line is active"]
pub type SpiCsActiveR = crate::BitReader;
#[doc = "Field `SPI_CTRL_BUSY` reader - SPI busy flag (transmission in progress and TX FIFO not empty)"]
pub type SpiCtrlBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI enable flag"]
    #[inline(always)]
    pub fn spi_ctrl_en(&self) -> SpiCtrlEnR {
        SpiCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock phase"]
    #[inline(always)]
    pub fn spi_ctrl_cpha(&self) -> SpiCtrlCphaR {
        SpiCtrlCphaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock polarity"]
    #[inline(always)]
    pub fn spi_ctrl_cpol(&self) -> SpiCtrlCpolR {
        SpiCtrlCpolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Clock prescaler select"]
    #[inline(always)]
    pub fn spi_ctrl_prsc(&self) -> SpiCtrlPrscR {
        SpiCtrlPrscR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:9 - SPI clock divider"]
    #[inline(always)]
    pub fn spi_ctrl_cdiv(&self) -> SpiCtrlCdivR {
        SpiCtrlCdivR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - RX FIFO data available (RX FIFO not empty)"]
    #[inline(always)]
    pub fn spi_ctrl_rx_avail(&self) -> SpiCtrlRxAvailR {
        SpiCtrlRxAvailR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX FIFO is empty"]
    #[inline(always)]
    pub fn spi_ctrl_tx_empty(&self) -> SpiCtrlTxEmptyR {
        SpiCtrlTxEmptyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX FIFO is full"]
    #[inline(always)]
    pub fn spi_ctrl_tx_full(&self) -> SpiCtrlTxFullR {
        SpiCtrlTxFullR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:27 - log2(FIFO size)"]
    #[inline(always)]
    pub fn spi_ctrl_fifo(&self) -> SpiCtrlFifoR {
        SpiCtrlFifoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Set if a chip-select line is active"]
    #[inline(always)]
    pub fn spi_cs_active(&self) -> SpiCsActiveR {
        SpiCsActiveR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI busy flag (transmission in progress and TX FIFO not empty)"]
    #[inline(always)]
    pub fn spi_ctrl_busy(&self) -> SpiCtrlBusyR {
        SpiCtrlBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI enable flag"]
    #[inline(always)]
    pub fn spi_ctrl_en(&mut self) -> SpiCtrlEnW<'_, CtrlSpec> {
        SpiCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock phase"]
    #[inline(always)]
    pub fn spi_ctrl_cpha(&mut self) -> SpiCtrlCphaW<'_, CtrlSpec> {
        SpiCtrlCphaW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock polarity"]
    #[inline(always)]
    pub fn spi_ctrl_cpol(&mut self) -> SpiCtrlCpolW<'_, CtrlSpec> {
        SpiCtrlCpolW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Clock prescaler select"]
    #[inline(always)]
    pub fn spi_ctrl_prsc(&mut self) -> SpiCtrlPrscW<'_, CtrlSpec> {
        SpiCtrlPrscW::new(self, 3)
    }
    #[doc = "Bits 6:9 - SPI clock divider"]
    #[inline(always)]
    pub fn spi_ctrl_cdiv(&mut self) -> SpiCtrlCdivW<'_, CtrlSpec> {
        SpiCtrlCdivW::new(self, 6)
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
