#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `SPI_DATA` reader - RX data / TX data/command (via FIFOs)"]
pub type SpiDataR = crate::FieldReader;
#[doc = "Field `SPI_DATA` writer - RX data / TX data/command (via FIFOs)"]
pub type SpiDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_DATA_CMD` writer - SPI TX data (0) / command (1) select"]
pub type SpiDataCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - RX data / TX data/command (via FIFOs)"]
    #[inline(always)]
    pub fn spi_data(&self) -> SpiDataR {
        SpiDataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX data / TX data/command (via FIFOs)"]
    #[inline(always)]
    pub fn spi_data(&mut self) -> SpiDataW<'_, DataSpec> {
        SpiDataW::new(self, 0)
    }
    #[doc = "Bit 31 - SPI TX data (0) / command (1) select"]
    #[inline(always)]
    pub fn spi_data_cmd(&mut self) -> SpiDataCmdW<'_, DataSpec> {
        SpiDataCmdW::new(self, 31)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
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
