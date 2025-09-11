#[doc = "Register `DCMD` reader"]
pub type R = crate::R<DcmdSpec>;
#[doc = "Register `DCMD` writer"]
pub type W = crate::W<DcmdSpec>;
#[doc = "Field `ONEWIRE_DCMD_DATA` reader - RTX data, transmitted LSB-first"]
pub type OnewireDcmdDataR = crate::FieldReader;
#[doc = "Field `ONEWIRE_DCMD_DATA` writer - RTX data, transmitted LSB-first"]
pub type OnewireDcmdDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ONEWIRE_DCMD_CMD` writer - Operation command"]
pub type OnewireDcmdCmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ONEWIRE_DCMD_PRESENCE` reader - Bus presence detected"]
pub type OnewireDcmdPresenceR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - RTX data, transmitted LSB-first"]
    #[inline(always)]
    pub fn onewire_dcmd_data(&self) -> OnewireDcmdDataR {
        OnewireDcmdDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 10 - Bus presence detected"]
    #[inline(always)]
    pub fn onewire_dcmd_presence(&self) -> OnewireDcmdPresenceR {
        OnewireDcmdPresenceR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTX data, transmitted LSB-first"]
    #[inline(always)]
    pub fn onewire_dcmd_data(&mut self) -> OnewireDcmdDataW<'_, DcmdSpec> {
        OnewireDcmdDataW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Operation command"]
    #[inline(always)]
    pub fn onewire_dcmd_cmd(&mut self) -> OnewireDcmdCmdW<'_, DcmdSpec> {
        OnewireDcmdCmdW::new(self, 8)
    }
}
#[doc = "Read/write transmission data/command register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
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
