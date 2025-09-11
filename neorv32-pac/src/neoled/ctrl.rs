#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `NEOLED_CTRL_EN` reader - NEOLED enable flag"]
pub type NeoledCtrlEnR = crate::BitReader;
#[doc = "Field `NEOLED_CTRL_EN` writer - NEOLED enable flag"]
pub type NeoledCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEOLED_CTRL_PRSC` reader - Clock prescaler select"]
pub type NeoledCtrlPrscR = crate::FieldReader;
#[doc = "Field `NEOLED_CTRL_PRSC` writer - Clock prescaler select"]
pub type NeoledCtrlPrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NEOLED_CTRL_T_TOT` reader - pulse-clock ticks per total period bit"]
pub type NeoledCtrlTTotR = crate::FieldReader;
#[doc = "Field `NEOLED_CTRL_T_TOT` writer - pulse-clock ticks per total period bit"]
pub type NeoledCtrlTTotW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NEOLED_CTRL_T_0H` reader - pulse-clock ticks per ZERO high-time"]
pub type NeoledCtrlT0hR = crate::FieldReader;
#[doc = "Field `NEOLED_CTRL_T_0H` writer - pulse-clock ticks per ZERO high-time"]
pub type NeoledCtrlT0hW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NEOLED_CTRL_T_1H` reader - pulse-clock ticks per ONE high-time"]
pub type NeoledCtrlT1hR = crate::FieldReader;
#[doc = "Field `NEOLED_CTRL_T_1H` writer - pulse-clock ticks per ONE high-time"]
pub type NeoledCtrlT1hW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NEOLED_CTRL_FIFO` reader - log2(TX FIFO size)"]
pub type NeoledCtrlFifoR = crate::FieldReader;
#[doc = "Field `NEOLED_CTRL_TX_EMPTY` reader - TX FIFO is empty"]
pub type NeoledCtrlTxEmptyR = crate::BitReader;
#[doc = "Field `NEOLED_CTRL_TX_FULL` reader - TX FIFO is full"]
pub type NeoledCtrlTxFullR = crate::BitReader;
#[doc = "Field `NEOLED_CTRL_TX_BUSY` reader - busy flag"]
pub type NeoledCtrlTxBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NEOLED enable flag"]
    #[inline(always)]
    pub fn neoled_ctrl_en(&self) -> NeoledCtrlEnR {
        NeoledCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn neoled_ctrl_prsc(&self) -> NeoledCtrlPrscR {
        NeoledCtrlPrscR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:8 - pulse-clock ticks per total period bit"]
    #[inline(always)]
    pub fn neoled_ctrl_t_tot(&self) -> NeoledCtrlTTotR {
        NeoledCtrlTTotR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - pulse-clock ticks per ZERO high-time"]
    #[inline(always)]
    pub fn neoled_ctrl_t_0h(&self) -> NeoledCtrlT0hR {
        NeoledCtrlT0hR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - pulse-clock ticks per ONE high-time"]
    #[inline(always)]
    pub fn neoled_ctrl_t_1h(&self) -> NeoledCtrlT1hR {
        NeoledCtrlT1hR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - log2(TX FIFO size)"]
    #[inline(always)]
    pub fn neoled_ctrl_fifo(&self) -> NeoledCtrlFifoR {
        NeoledCtrlFifoR::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - TX FIFO is empty"]
    #[inline(always)]
    pub fn neoled_ctrl_tx_empty(&self) -> NeoledCtrlTxEmptyR {
        NeoledCtrlTxEmptyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TX FIFO is full"]
    #[inline(always)]
    pub fn neoled_ctrl_tx_full(&self) -> NeoledCtrlTxFullR {
        NeoledCtrlTxFullR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - busy flag"]
    #[inline(always)]
    pub fn neoled_ctrl_tx_busy(&self) -> NeoledCtrlTxBusyR {
        NeoledCtrlTxBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NEOLED enable flag"]
    #[inline(always)]
    pub fn neoled_ctrl_en(&mut self) -> NeoledCtrlEnW<'_, CtrlSpec> {
        NeoledCtrlEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Clock prescaler select"]
    #[inline(always)]
    pub fn neoled_ctrl_prsc(&mut self) -> NeoledCtrlPrscW<'_, CtrlSpec> {
        NeoledCtrlPrscW::new(self, 1)
    }
    #[doc = "Bits 4:8 - pulse-clock ticks per total period bit"]
    #[inline(always)]
    pub fn neoled_ctrl_t_tot(&mut self) -> NeoledCtrlTTotW<'_, CtrlSpec> {
        NeoledCtrlTTotW::new(self, 4)
    }
    #[doc = "Bits 9:13 - pulse-clock ticks per ZERO high-time"]
    #[inline(always)]
    pub fn neoled_ctrl_t_0h(&mut self) -> NeoledCtrlT0hW<'_, CtrlSpec> {
        NeoledCtrlT0hW::new(self, 9)
    }
    #[doc = "Bits 14:18 - pulse-clock ticks per ONE high-time"]
    #[inline(always)]
    pub fn neoled_ctrl_t_1h(&mut self) -> NeoledCtrlT1hW<'_, CtrlSpec> {
        NeoledCtrlT1hW::new(self, 14)
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
