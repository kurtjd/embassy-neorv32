#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `UART_CTRL_EN` reader - UART enable flag"]
pub type UartCtrlEnR = crate::BitReader;
#[doc = "Field `UART_CTRL_EN` writer - UART enable flag"]
pub type UartCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CTRL_SIM_MODE` reader - Simulation output override enable, for use in simulation only"]
pub type UartCtrlSimModeR = crate::BitReader;
#[doc = "Field `UART_CTRL_SIM_MODE` writer - Simulation output override enable, for use in simulation only"]
pub type UartCtrlSimModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CTRL_HWFC_EN` reader - Enable RTS/CTS hardware flow-control"]
pub type UartCtrlHwfcEnR = crate::BitReader;
#[doc = "Field `UART_CTRL_HWFC_EN` writer - Enable RTS/CTS hardware flow-control"]
pub type UartCtrlHwfcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CTRL_PRSC` reader - CLock prescaler select"]
pub type UartCtrlPrscR = crate::FieldReader;
#[doc = "Field `UART_CTRL_PRSC` writer - CLock prescaler select"]
pub type UartCtrlPrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART_CTRL_BAUD` reader - BAUD rate divisor"]
pub type UartCtrlBaudR = crate::FieldReader<u16>;
#[doc = "Field `UART_CTRL_BAUD` writer - BAUD rate divisor"]
pub type UartCtrlBaudW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `UART_CTRL_RX_NEMPTY` reader - RX FIFO not empty"]
pub type UartCtrlRxNemptyR = crate::BitReader;
#[doc = "Field `UART_CTRL_RX_FULL` reader - RX FIFO full"]
pub type UartCtrlRxFullR = crate::BitReader;
#[doc = "Field `UART_CTRL_TX_EMPTY` reader - TX FIFO empty"]
pub type UartCtrlTxEmptyR = crate::BitReader;
#[doc = "Field `UART_CTRL_TX_NFULL` reader - TX FIFO not full"]
pub type UartCtrlTxNfullR = crate::BitReader;
#[doc = "Field `UART_CTRL_IRQ_RX_NEMPTY` reader - Fire IRQ if RX FIFO not empty"]
pub type UartCtrlIrqRxNemptyR = crate::BitReader;
#[doc = "Field `UART_CTRL_IRQ_RX_NEMPTY` writer - Fire IRQ if RX FIFO not empty"]
pub type UartCtrlIrqRxNemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CTRL_IRQ_RX_FULL` reader - Fire IRQ if RX FIFO full"]
pub type UartCtrlIrqRxFullR = crate::BitReader;
#[doc = "Field `UART_CTRL_IRQ_RX_FULL` writer - Fire IRQ if RX FIFO full"]
pub type UartCtrlIrqRxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CTRL_IRQ_TX_EMPTY` reader - Fire IRQ if TX FIFO empty"]
pub type UartCtrlIrqTxEmptyR = crate::BitReader;
#[doc = "Field `UART_CTRL_IRQ_TX_EMPTY` writer - Fire IRQ if TX FIFO empty"]
pub type UartCtrlIrqTxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CTRL_IRQ_TX_NFULL` reader - Fire IRQ if TX FIFO not full"]
pub type UartCtrlIrqTxNfullR = crate::BitReader;
#[doc = "Field `UART_CTRL_IRQ_TX_NFULL` writer - Fire IRQ if TX FIFO not full"]
pub type UartCtrlIrqTxNfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CTRL_RX_OVER` reader - RX FIFO overflow"]
pub type UartCtrlRxOverR = crate::BitReader;
#[doc = "Field `UART_CTRL_TX_BUSY` reader - Transmitter busy or TX FIFO not empty"]
pub type UartCtrlTxBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART enable flag"]
    #[inline(always)]
    pub fn uart_ctrl_en(&self) -> UartCtrlEnR {
        UartCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Simulation output override enable, for use in simulation only"]
    #[inline(always)]
    pub fn uart_ctrl_sim_mode(&self) -> UartCtrlSimModeR {
        UartCtrlSimModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable RTS/CTS hardware flow-control"]
    #[inline(always)]
    pub fn uart_ctrl_hwfc_en(&self) -> UartCtrlHwfcEnR {
        UartCtrlHwfcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - CLock prescaler select"]
    #[inline(always)]
    pub fn uart_ctrl_prsc(&self) -> UartCtrlPrscR {
        UartCtrlPrscR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:15 - BAUD rate divisor"]
    #[inline(always)]
    pub fn uart_ctrl_baud(&self) -> UartCtrlBaudR {
        UartCtrlBaudR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - RX FIFO not empty"]
    #[inline(always)]
    pub fn uart_ctrl_rx_nempty(&self) -> UartCtrlRxNemptyR {
        UartCtrlRxNemptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX FIFO full"]
    #[inline(always)]
    pub fn uart_ctrl_rx_full(&self) -> UartCtrlRxFullR {
        UartCtrlRxFullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX FIFO empty"]
    #[inline(always)]
    pub fn uart_ctrl_tx_empty(&self) -> UartCtrlTxEmptyR {
        UartCtrlTxEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TX FIFO not full"]
    #[inline(always)]
    pub fn uart_ctrl_tx_nfull(&self) -> UartCtrlTxNfullR {
        UartCtrlTxNfullR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Fire IRQ if RX FIFO not empty"]
    #[inline(always)]
    pub fn uart_ctrl_irq_rx_nempty(&self) -> UartCtrlIrqRxNemptyR {
        UartCtrlIrqRxNemptyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Fire IRQ if RX FIFO full"]
    #[inline(always)]
    pub fn uart_ctrl_irq_rx_full(&self) -> UartCtrlIrqRxFullR {
        UartCtrlIrqRxFullR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fire IRQ if TX FIFO empty"]
    #[inline(always)]
    pub fn uart_ctrl_irq_tx_empty(&self) -> UartCtrlIrqTxEmptyR {
        UartCtrlIrqTxEmptyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fire IRQ if TX FIFO not full"]
    #[inline(always)]
    pub fn uart_ctrl_irq_tx_nfull(&self) -> UartCtrlIrqTxNfullR {
        UartCtrlIrqTxNfullR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - RX FIFO overflow"]
    #[inline(always)]
    pub fn uart_ctrl_rx_over(&self) -> UartCtrlRxOverR {
        UartCtrlRxOverR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmitter busy or TX FIFO not empty"]
    #[inline(always)]
    pub fn uart_ctrl_tx_busy(&self) -> UartCtrlTxBusyR {
        UartCtrlTxBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART enable flag"]
    #[inline(always)]
    pub fn uart_ctrl_en(&mut self) -> UartCtrlEnW<'_, CtrlSpec> {
        UartCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Simulation output override enable, for use in simulation only"]
    #[inline(always)]
    pub fn uart_ctrl_sim_mode(&mut self) -> UartCtrlSimModeW<'_, CtrlSpec> {
        UartCtrlSimModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable RTS/CTS hardware flow-control"]
    #[inline(always)]
    pub fn uart_ctrl_hwfc_en(&mut self) -> UartCtrlHwfcEnW<'_, CtrlSpec> {
        UartCtrlHwfcEnW::new(self, 2)
    }
    #[doc = "Bits 3:5 - CLock prescaler select"]
    #[inline(always)]
    pub fn uart_ctrl_prsc(&mut self) -> UartCtrlPrscW<'_, CtrlSpec> {
        UartCtrlPrscW::new(self, 3)
    }
    #[doc = "Bits 6:15 - BAUD rate divisor"]
    #[inline(always)]
    pub fn uart_ctrl_baud(&mut self) -> UartCtrlBaudW<'_, CtrlSpec> {
        UartCtrlBaudW::new(self, 6)
    }
    #[doc = "Bit 20 - Fire IRQ if RX FIFO not empty"]
    #[inline(always)]
    pub fn uart_ctrl_irq_rx_nempty(&mut self) -> UartCtrlIrqRxNemptyW<'_, CtrlSpec> {
        UartCtrlIrqRxNemptyW::new(self, 20)
    }
    #[doc = "Bit 21 - Fire IRQ if RX FIFO full"]
    #[inline(always)]
    pub fn uart_ctrl_irq_rx_full(&mut self) -> UartCtrlIrqRxFullW<'_, CtrlSpec> {
        UartCtrlIrqRxFullW::new(self, 21)
    }
    #[doc = "Bit 22 - Fire IRQ if TX FIFO empty"]
    #[inline(always)]
    pub fn uart_ctrl_irq_tx_empty(&mut self) -> UartCtrlIrqTxEmptyW<'_, CtrlSpec> {
        UartCtrlIrqTxEmptyW::new(self, 22)
    }
    #[doc = "Bit 23 - Fire IRQ if TX FIFO not full"]
    #[inline(always)]
    pub fn uart_ctrl_irq_tx_nfull(&mut self) -> UartCtrlIrqTxNfullW<'_, CtrlSpec> {
        UartCtrlIrqTxNfullW::new(self, 23)
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
