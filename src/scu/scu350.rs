#[doc = "Register `SCU350` reader"]
pub type R = crate::R<Scu350Spec>;
#[doc = "Register `SCU350` writer"]
pub type W = crate::W<Scu350Spec>;
#[doc = "Field `MACRGMIITXCLK1GClkOutputDelay` reader - MAC RGMII_TXCLK (1G) clock output delay"]
pub type Macrgmiitxclk1gclkOutputDelayR = crate::FieldReader;
#[doc = "Field `MACRGMIITXCLK1GClkOutputDelay` writer - MAC RGMII_TXCLK (1G) clock output delay"]
pub type Macrgmiitxclk1gclkOutputDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MACRMIIRCLKRGMIIRXCLK1GClkInputDelay` reader - MAC RMII_RCLK/RGMII_RXCLK (1G) clock input delay"]
pub type Macrmiirclkrgmiirxclk1gclkInputDelayR = crate::FieldReader;
#[doc = "Field `MACRMIIRCLKRGMIIRXCLK1GClkInputDelay` writer - MAC RMII_RCLK/RGMII_RXCLK (1G) clock input delay"]
pub type Macrmiirclkrgmiirxclk1gclkInputDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MACRXCLKClkTreeInverePhase` reader - MAC RXCLK clock tree invere phase"]
pub type MacrxclkclkTreeInverePhaseR = crate::BitReader;
#[doc = "Field `MACRXCLKClkTreeInverePhase` writer - MAC RXCLK clock tree invere phase"]
pub type MacrxclkclkTreeInverePhaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACRMIITxDataAtClkFallingEdge` reader - MAC RMII transmit data at clock falling edge"]
pub type MacrmiitxDataAtClkFallingEdgeR = crate::BitReader;
#[doc = "Field `MACRMIITxDataAtClkFallingEdge` writer - MAC RMII transmit data at clock falling edge"]
pub type MacrmiitxDataAtClkFallingEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMII50MHzRCLKOutputEnbl` reader - RMII 50MHz RCLK output enable"]
pub type Rmii50mhzRclkoutputEnblR = crate::BitReader;
#[doc = "Field `RMII50MHzRCLKOutputEnbl` writer - RMII 50MHz RCLK output enable"]
pub type Rmii50mhzRclkoutputEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGMII125MHzClkSrcSel` reader - RGMII 125MHz clock source selection"]
pub type Rgmii125mhzClkSrcSelR = crate::BitReader;
#[doc = "Field `RGMII125MHzClkSrcSel` writer - RGMII 125MHz clock source selection"]
pub type Rgmii125mhzClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - MAC RGMII_TXCLK (1G) clock output delay"]
    #[inline(always)]
    pub fn macrgmiitxclk1gclk_output_delay(&self) -> Macrgmiitxclk1gclkOutputDelayR {
        Macrgmiitxclk1gclkOutputDelayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - MAC RMII_RCLK/RGMII_RXCLK (1G) clock input delay"]
    #[inline(always)]
    pub fn macrmiirclkrgmiirxclk1gclk_input_delay(&self) -> Macrmiirclkrgmiirxclk1gclkInputDelayR {
        Macrmiirclkrgmiirxclk1gclkInputDelayR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - MAC RXCLK clock tree invere phase"]
    #[inline(always)]
    pub fn macrxclkclk_tree_invere_phase(&self) -> MacrxclkclkTreeInverePhaseR {
        MacrxclkclkTreeInverePhaseR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MAC RMII transmit data at clock falling edge"]
    #[inline(always)]
    pub fn macrmiitx_data_at_clk_falling_edge(&self) -> MacrmiitxDataAtClkFallingEdgeR {
        MacrmiitxDataAtClkFallingEdgeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RMII 50MHz RCLK output enable"]
    #[inline(always)]
    pub fn rmii50mhz_rclkoutput_enbl(&self) -> Rmii50mhzRclkoutputEnblR {
        Rmii50mhzRclkoutputEnblR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - RGMII 125MHz clock source selection"]
    #[inline(always)]
    pub fn rgmii125mhz_clk_src_sel(&self) -> Rgmii125mhzClkSrcSelR {
        Rgmii125mhzClkSrcSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - MAC RGMII_TXCLK (1G) clock output delay"]
    #[inline(always)]
    pub fn macrgmiitxclk1gclk_output_delay(
        &mut self,
    ) -> Macrgmiitxclk1gclkOutputDelayW<Scu350Spec> {
        Macrgmiitxclk1gclkOutputDelayW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Scu350Spec> {
        Reserved5W::new(self, 6)
    }
    #[doc = "Bits 12:17 - MAC RMII_RCLK/RGMII_RXCLK (1G) clock input delay"]
    #[inline(always)]
    pub fn macrmiirclkrgmiirxclk1gclk_input_delay(
        &mut self,
    ) -> Macrmiirclkrgmiirxclk1gclkInputDelayW<Scu350Spec> {
        Macrmiirclkrgmiirxclk1gclkInputDelayW::new(self, 12)
    }
    #[doc = "Bits 18:23 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu350Spec> {
        Reserved4W::new(self, 18)
    }
    #[doc = "Bit 24 - MAC RXCLK clock tree invere phase"]
    #[inline(always)]
    pub fn macrxclkclk_tree_invere_phase(&mut self) -> MacrxclkclkTreeInverePhaseW<Scu350Spec> {
        MacrxclkclkTreeInverePhaseW::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu350Spec> {
        Reserved3W::new(self, 25)
    }
    #[doc = "Bit 26 - MAC RMII transmit data at clock falling edge"]
    #[inline(always)]
    pub fn macrmiitx_data_at_clk_falling_edge(
        &mut self,
    ) -> MacrmiitxDataAtClkFallingEdgeW<Scu350Spec> {
        MacrmiitxDataAtClkFallingEdgeW::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu350Spec> {
        Reserved2W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu350Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 29 - RMII 50MHz RCLK output enable"]
    #[inline(always)]
    pub fn rmii50mhz_rclkoutput_enbl(&mut self) -> Rmii50mhzRclkoutputEnblW<Scu350Spec> {
        Rmii50mhzRclkoutputEnblW::new(self, 29)
    }
    #[doc = "Bit 31 - RGMII 125MHz clock source selection"]
    #[inline(always)]
    pub fn rgmii125mhz_clk_src_sel(&mut self) -> Rgmii125mhzClkSrcSelW<Scu350Spec> {
        Rgmii125mhzClkSrcSelW::new(self, 31)
    }
}
#[doc = "MAC Interface Clock Delay Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu350::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu350::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu350Spec;
impl crate::RegisterSpec for Scu350Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu350::R`](R) reader structure"]
impl crate::Readable for Scu350Spec {}
#[doc = "`write(|w| ..)` method takes [`scu350::W`](W) writer structure"]
impl crate::Writable for Scu350Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU350 to value 0"]
impl crate::Resettable for Scu350Spec {}
