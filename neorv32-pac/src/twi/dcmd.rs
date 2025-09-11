#[doc = "Register `DCMD` reader"]
pub type R = crate::R<DcmdSpec>;
#[doc = "Register `DCMD` writer"]
pub type W = crate::W<DcmdSpec>;
#[doc = "Field `TWI_DCMD` reader - RX/TX data"]
pub type TwiDcmdR = crate::FieldReader;
#[doc = "Field `TWI_DCMD` writer - RX/TX data"]
pub type TwiDcmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWI_DCMD_ACK` reader - RX = ACK/NACK, TX = MACK"]
pub type TwiDcmdAckR = crate::BitReader;
#[doc = "Field `TWI_DCMD_ACK` writer - RX = ACK/NACK, TX = MACK"]
pub type TwiDcmdAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI_DCMD_CMD` reader - Operation command"]
pub type TwiDcmdCmdR = crate::FieldReader;
#[doc = "Field `TWI_DCMD_CMD` writer - Operation command"]
pub type TwiDcmdCmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - RX/TX data"]
    #[inline(always)]
    pub fn twi_dcmd(&self) -> TwiDcmdR {
        TwiDcmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - RX = ACK/NACK, TX = MACK"]
    #[inline(always)]
    pub fn twi_dcmd_ack(&self) -> TwiDcmdAckR {
        TwiDcmdAckR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Operation command"]
    #[inline(always)]
    pub fn twi_dcmd_cmd(&self) -> TwiDcmdCmdR {
        TwiDcmdCmdR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX/TX data"]
    #[inline(always)]
    pub fn twi_dcmd(&mut self) -> TwiDcmdW<'_, DcmdSpec> {
        TwiDcmdW::new(self, 0)
    }
    #[doc = "Bit 8 - RX = ACK/NACK, TX = MACK"]
    #[inline(always)]
    pub fn twi_dcmd_ack(&mut self) -> TwiDcmdAckW<'_, DcmdSpec> {
        TwiDcmdAckW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Operation command"]
    #[inline(always)]
    pub fn twi_dcmd_cmd(&mut self) -> TwiDcmdCmdW<'_, DcmdSpec> {
        TwiDcmdCmdW::new(self, 9)
    }
}
#[doc = "RX/TX data/command register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct DcmdSpec;
impl crate::RegisterSpec for DcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmd::R`](R) reader structure"]
impl crate::Readable for DcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`dcmd::W`](W) writer structure"]
impl crate::Writable for DcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCMD to value 0"]
impl crate::Resettable for DcmdSpec {}
