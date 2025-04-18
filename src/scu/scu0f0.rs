#[doc = "Register `SCU0F0` reader"]
pub type R = crate::R<Scu0f0Spec>;
#[doc = "Register `SCU0F0` writer"]
pub type W = crate::W<Scu0f0Spec>;
#[doc = "Field `SelectIntSPIMasterConnection` reader - Select internal SPI master connection"]
pub type SelectIntSpimasterConnectionR = crate::FieldReader;
#[doc = "Field `SelectIntSPIMasterConnection` writer - Select internal SPI master connection"]
pub type SelectIntSpimasterConnectionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IntSPIMasterSel` reader - Internal SPI master selection"]
pub type IntSpimasterSelR = crate::BitReader;
#[doc = "Field `IntSPIMasterSel` writer - Internal SPI master selection"]
pub type IntSpimasterSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblPassthroughOfSPIPF1` reader - Enable passthrough of SPI_PF1"]
pub type EnblPassthroughOfSpipf1R = crate::BitReader;
#[doc = "Field `EnblPassthroughOfSPIPF1` writer - Enable passthrough of SPI_PF1"]
pub type EnblPassthroughOfSpipf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblPassthroughOfSPIPF2` reader - Enable passthrough of SPI_PF2"]
pub type EnblPassthroughOfSpipf2R = crate::BitReader;
#[doc = "Field `EnblPassthroughOfSPIPF2` writer - Enable passthrough of SPI_PF2"]
pub type EnblPassthroughOfSpipf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblPassthroughOfSPIPF3` reader - Enable passthrough of SPI_PF3"]
pub type EnblPassthroughOfSpipf3R = crate::BitReader;
#[doc = "Field `EnblPassthroughOfSPIPF3` writer - Enable passthrough of SPI_PF3"]
pub type EnblPassthroughOfSpipf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblPassthroughOfSPIPF4` reader - Enable passthrough of SPI_PF4"]
pub type EnblPassthroughOfSpipf4R = crate::BitReader;
#[doc = "Field `EnblPassthroughOfSPIPF4` writer - Enable passthrough of SPI_PF4"]
pub type EnblPassthroughOfSpipf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblFilterOfSPIPF1` reader - Enable filter of SPI_PF1"]
pub type EnblFilterOfSpipf1R = crate::BitReader;
#[doc = "Field `EnblFilterOfSPIPF1` writer - Enable filter of SPI_PF1"]
pub type EnblFilterOfSpipf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblFilterOfSPIPF2` reader - Enable filter of SPI_PF2"]
pub type EnblFilterOfSpipf2R = crate::BitReader;
#[doc = "Field `EnblFilterOfSPIPF2` writer - Enable filter of SPI_PF2"]
pub type EnblFilterOfSpipf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblFilterOfSPIPF3` reader - Enable filter of SPI_PF3"]
pub type EnblFilterOfSpipf3R = crate::BitReader;
#[doc = "Field `EnblFilterOfSPIPF3` writer - Enable filter of SPI_PF3"]
pub type EnblFilterOfSpipf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblFilterOfSPIPF4` reader - Enable filter of SPI_PF4"]
pub type EnblFilterOfSpipf4R = crate::BitReader;
#[doc = "Field `EnblFilterOfSPIPF4` writer - Enable filter of SPI_PF4"]
pub type EnblFilterOfSpipf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ExtMuxSelectSigOfSPIPF1` reader - External Mux Select Signal of SPI_PF1"]
pub type ExtMuxSelectSigOfSpipf1R = crate::BitReader;
#[doc = "Field `ExtMuxSelectSigOfSPIPF1` writer - External Mux Select Signal of SPI_PF1"]
pub type ExtMuxSelectSigOfSpipf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ExtMuxSelectSigOfSPIPF2` reader - External Mux Select Signal of SPI_PF2"]
pub type ExtMuxSelectSigOfSpipf2R = crate::BitReader;
#[doc = "Field `ExtMuxSelectSigOfSPIPF2` writer - External Mux Select Signal of SPI_PF2"]
pub type ExtMuxSelectSigOfSpipf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ExtMuxSelectSigOfSPIPF3` reader - External Mux Select Signal of SPI_PF3"]
pub type ExtMuxSelectSigOfSpipf3R = crate::BitReader;
#[doc = "Field `ExtMuxSelectSigOfSPIPF3` writer - External Mux Select Signal of SPI_PF3"]
pub type ExtMuxSelectSigOfSpipf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ExtMuxSelectSigOfSPIPF4` reader - External Mux Select Signal of SPI_PF4"]
pub type ExtMuxSelectSigOfSpipf4R = crate::BitReader;
#[doc = "Field `ExtMuxSelectSigOfSPIPF4` writer - External Mux Select Signal of SPI_PF4"]
pub type ExtMuxSelectSigOfSpipf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF1RstSig` reader - SPI_PF1 Reset Signal"]
pub type Spipf1rstSigR = crate::BitReader;
#[doc = "Field `SPIPF1RstSig` writer - SPI_PF1 Reset Signal"]
pub type Spipf1rstSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF2RstSig` reader - SPI_PF2 Reset Signal"]
pub type Spipf2rstSigR = crate::BitReader;
#[doc = "Field `SPIPF2RstSig` writer - SPI_PF2 Reset Signal"]
pub type Spipf2rstSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF3RstSig` reader - SPI_PF3 Reset Signal"]
pub type Spipf3rstSigR = crate::BitReader;
#[doc = "Field `SPIPF3RstSig` writer - SPI_PF3 Reset Signal"]
pub type Spipf3rstSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF4RstSig` reader - SPI_PF4 Reset Signal"]
pub type Spipf4rstSigR = crate::BitReader;
#[doc = "Field `SPIPF4RstSig` writer - SPI_PF4 Reset Signal"]
pub type Spipf4rstSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF1RstSourceSel` reader - SPI_PF1 Reset Source Selection"]
pub type Spipf1rstSourceSelR = crate::BitReader;
#[doc = "Field `SPIPF1RstSourceSel` writer - SPI_PF1 Reset Source Selection"]
pub type Spipf1rstSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF2RstSourceSel` reader - SPI_PF2 Reset Source Selection"]
pub type Spipf2rstSourceSelR = crate::BitReader;
#[doc = "Field `SPIPF2RstSourceSel` writer - SPI_PF2 Reset Source Selection"]
pub type Spipf2rstSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF3RstSourceSel` reader - SPI_PF3 Reset Source Selection"]
pub type Spipf3rstSourceSelR = crate::BitReader;
#[doc = "Field `SPIPF3RstSourceSel` writer - SPI_PF3 Reset Source Selection"]
pub type Spipf3rstSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF4RstSourceSel` reader - SPI_PF4 Reset Source Selection"]
pub type Spipf4rstSourceSelR = crate::BitReader;
#[doc = "Field `SPIPF4RstSourceSel` writer - SPI_PF4 Reset Source Selection"]
pub type Spipf4rstSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF1RstOutputEnbl` reader - SPI_PF1 Reset Output Enable"]
pub type Spipf1rstOutputEnblR = crate::BitReader;
#[doc = "Field `SPIPF1RstOutputEnbl` writer - SPI_PF1 Reset Output Enable"]
pub type Spipf1rstOutputEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF2RstOutputEnbl` reader - SPI_PF2 Reset Output Enable"]
pub type Spipf2rstOutputEnblR = crate::BitReader;
#[doc = "Field `SPIPF2RstOutputEnbl` writer - SPI_PF2 Reset Output Enable"]
pub type Spipf2rstOutputEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF3RstOutputEnbl` reader - SPI_PF3 Reset Output Enable"]
pub type Spipf3rstOutputEnblR = crate::BitReader;
#[doc = "Field `SPIPF3RstOutputEnbl` writer - SPI_PF3 Reset Output Enable"]
pub type Spipf3rstOutputEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIPF4RstOutputEnbl` reader - SPI_PF4 Reset Output Enable"]
pub type Spipf4rstOutputEnblR = crate::BitReader;
#[doc = "Field `SPIPF4RstOutputEnbl` writer - SPI_PF4 Reset Output Enable"]
pub type Spipf4rstOutputEnblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Select internal SPI master connection"]
    #[inline(always)]
    pub fn select_int_spimaster_connection(&self) -> SelectIntSpimasterConnectionR {
        SelectIntSpimasterConnectionR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Internal SPI master selection"]
    #[inline(always)]
    pub fn int_spimaster_sel(&self) -> IntSpimasterSelR {
        IntSpimasterSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable passthrough of SPI_PF1"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf1(&self) -> EnblPassthroughOfSpipf1R {
        EnblPassthroughOfSpipf1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable passthrough of SPI_PF2"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf2(&self) -> EnblPassthroughOfSpipf2R {
        EnblPassthroughOfSpipf2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable passthrough of SPI_PF3"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf3(&self) -> EnblPassthroughOfSpipf3R {
        EnblPassthroughOfSpipf3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable passthrough of SPI_PF4"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf4(&self) -> EnblPassthroughOfSpipf4R {
        EnblPassthroughOfSpipf4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable filter of SPI_PF1"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf1(&self) -> EnblFilterOfSpipf1R {
        EnblFilterOfSpipf1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable filter of SPI_PF2"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf2(&self) -> EnblFilterOfSpipf2R {
        EnblFilterOfSpipf2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable filter of SPI_PF3"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf3(&self) -> EnblFilterOfSpipf3R {
        EnblFilterOfSpipf3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable filter of SPI_PF4"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf4(&self) -> EnblFilterOfSpipf4R {
        EnblFilterOfSpipf4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Mux Select Signal of SPI_PF1"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf1(&self) -> ExtMuxSelectSigOfSpipf1R {
        ExtMuxSelectSigOfSpipf1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Mux Select Signal of SPI_PF2"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf2(&self) -> ExtMuxSelectSigOfSpipf2R {
        ExtMuxSelectSigOfSpipf2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Mux Select Signal of SPI_PF3"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf3(&self) -> ExtMuxSelectSigOfSpipf3R {
        ExtMuxSelectSigOfSpipf3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Mux Select Signal of SPI_PF4"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf4(&self) -> ExtMuxSelectSigOfSpipf4R {
        ExtMuxSelectSigOfSpipf4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI_PF1 Reset Signal"]
    #[inline(always)]
    pub fn spipf1rst_sig(&self) -> Spipf1rstSigR {
        Spipf1rstSigR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPI_PF2 Reset Signal"]
    #[inline(always)]
    pub fn spipf2rst_sig(&self) -> Spipf2rstSigR {
        Spipf2rstSigR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SPI_PF3 Reset Signal"]
    #[inline(always)]
    pub fn spipf3rst_sig(&self) -> Spipf3rstSigR {
        Spipf3rstSigR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPI_PF4 Reset Signal"]
    #[inline(always)]
    pub fn spipf4rst_sig(&self) -> Spipf4rstSigR {
        Spipf4rstSigR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI_PF1 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf1rst_source_sel(&self) -> Spipf1rstSourceSelR {
        Spipf1rstSourceSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI_PF2 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf2rst_source_sel(&self) -> Spipf2rstSourceSelR {
        Spipf2rstSourceSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SPI_PF3 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf3rst_source_sel(&self) -> Spipf3rstSourceSelR {
        Spipf3rstSourceSelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SPI_PF4 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf4rst_source_sel(&self) -> Spipf4rstSourceSelR {
        Spipf4rstSourceSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SPI_PF1 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf1rst_output_enbl(&self) -> Spipf1rstOutputEnblR {
        Spipf1rstOutputEnblR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SPI_PF2 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf2rst_output_enbl(&self) -> Spipf2rstOutputEnblR {
        Spipf2rstOutputEnblR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SPI_PF3 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf3rst_output_enbl(&self) -> Spipf3rstOutputEnblR {
        Spipf3rstOutputEnblR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SPI_PF4 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf4rst_output_enbl(&self) -> Spipf4rstOutputEnblR {
        Spipf4rstOutputEnblR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select internal SPI master connection"]
    #[inline(always)]
    pub fn select_int_spimaster_connection(&mut self) -> SelectIntSpimasterConnectionW<Scu0f0Spec> {
        SelectIntSpimasterConnectionW::new(self, 0)
    }
    #[doc = "Bit 3 - Internal SPI master selection"]
    #[inline(always)]
    pub fn int_spimaster_sel(&mut self) -> IntSpimasterSelW<Scu0f0Spec> {
        IntSpimasterSelW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable passthrough of SPI_PF1"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf1(&mut self) -> EnblPassthroughOfSpipf1W<Scu0f0Spec> {
        EnblPassthroughOfSpipf1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable passthrough of SPI_PF2"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf2(&mut self) -> EnblPassthroughOfSpipf2W<Scu0f0Spec> {
        EnblPassthroughOfSpipf2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable passthrough of SPI_PF3"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf3(&mut self) -> EnblPassthroughOfSpipf3W<Scu0f0Spec> {
        EnblPassthroughOfSpipf3W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable passthrough of SPI_PF4"]
    #[inline(always)]
    pub fn enbl_passthrough_of_spipf4(&mut self) -> EnblPassthroughOfSpipf4W<Scu0f0Spec> {
        EnblPassthroughOfSpipf4W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable filter of SPI_PF1"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf1(&mut self) -> EnblFilterOfSpipf1W<Scu0f0Spec> {
        EnblFilterOfSpipf1W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable filter of SPI_PF2"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf2(&mut self) -> EnblFilterOfSpipf2W<Scu0f0Spec> {
        EnblFilterOfSpipf2W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable filter of SPI_PF3"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf3(&mut self) -> EnblFilterOfSpipf3W<Scu0f0Spec> {
        EnblFilterOfSpipf3W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable filter of SPI_PF4"]
    #[inline(always)]
    pub fn enbl_filter_of_spipf4(&mut self) -> EnblFilterOfSpipf4W<Scu0f0Spec> {
        EnblFilterOfSpipf4W::new(self, 11)
    }
    #[doc = "Bit 12 - External Mux Select Signal of SPI_PF1"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf1(&mut self) -> ExtMuxSelectSigOfSpipf1W<Scu0f0Spec> {
        ExtMuxSelectSigOfSpipf1W::new(self, 12)
    }
    #[doc = "Bit 13 - External Mux Select Signal of SPI_PF2"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf2(&mut self) -> ExtMuxSelectSigOfSpipf2W<Scu0f0Spec> {
        ExtMuxSelectSigOfSpipf2W::new(self, 13)
    }
    #[doc = "Bit 14 - External Mux Select Signal of SPI_PF3"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf3(&mut self) -> ExtMuxSelectSigOfSpipf3W<Scu0f0Spec> {
        ExtMuxSelectSigOfSpipf3W::new(self, 14)
    }
    #[doc = "Bit 15 - External Mux Select Signal of SPI_PF4"]
    #[inline(always)]
    pub fn ext_mux_select_sig_of_spipf4(&mut self) -> ExtMuxSelectSigOfSpipf4W<Scu0f0Spec> {
        ExtMuxSelectSigOfSpipf4W::new(self, 15)
    }
    #[doc = "Bit 16 - SPI_PF1 Reset Signal"]
    #[inline(always)]
    pub fn spipf1rst_sig(&mut self) -> Spipf1rstSigW<Scu0f0Spec> {
        Spipf1rstSigW::new(self, 16)
    }
    #[doc = "Bit 17 - SPI_PF2 Reset Signal"]
    #[inline(always)]
    pub fn spipf2rst_sig(&mut self) -> Spipf2rstSigW<Scu0f0Spec> {
        Spipf2rstSigW::new(self, 17)
    }
    #[doc = "Bit 18 - SPI_PF3 Reset Signal"]
    #[inline(always)]
    pub fn spipf3rst_sig(&mut self) -> Spipf3rstSigW<Scu0f0Spec> {
        Spipf3rstSigW::new(self, 18)
    }
    #[doc = "Bit 19 - SPI_PF4 Reset Signal"]
    #[inline(always)]
    pub fn spipf4rst_sig(&mut self) -> Spipf4rstSigW<Scu0f0Spec> {
        Spipf4rstSigW::new(self, 19)
    }
    #[doc = "Bit 20 - SPI_PF1 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf1rst_source_sel(&mut self) -> Spipf1rstSourceSelW<Scu0f0Spec> {
        Spipf1rstSourceSelW::new(self, 20)
    }
    #[doc = "Bit 21 - SPI_PF2 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf2rst_source_sel(&mut self) -> Spipf2rstSourceSelW<Scu0f0Spec> {
        Spipf2rstSourceSelW::new(self, 21)
    }
    #[doc = "Bit 22 - SPI_PF3 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf3rst_source_sel(&mut self) -> Spipf3rstSourceSelW<Scu0f0Spec> {
        Spipf3rstSourceSelW::new(self, 22)
    }
    #[doc = "Bit 23 - SPI_PF4 Reset Source Selection"]
    #[inline(always)]
    pub fn spipf4rst_source_sel(&mut self) -> Spipf4rstSourceSelW<Scu0f0Spec> {
        Spipf4rstSourceSelW::new(self, 23)
    }
    #[doc = "Bit 24 - SPI_PF1 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf1rst_output_enbl(&mut self) -> Spipf1rstOutputEnblW<Scu0f0Spec> {
        Spipf1rstOutputEnblW::new(self, 24)
    }
    #[doc = "Bit 25 - SPI_PF2 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf2rst_output_enbl(&mut self) -> Spipf2rstOutputEnblW<Scu0f0Spec> {
        Spipf2rstOutputEnblW::new(self, 25)
    }
    #[doc = "Bit 26 - SPI_PF3 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf3rst_output_enbl(&mut self) -> Spipf3rstOutputEnblW<Scu0f0Spec> {
        Spipf3rstOutputEnblW::new(self, 26)
    }
    #[doc = "Bit 27 - SPI_PF4 Reset Output Enable"]
    #[inline(always)]
    pub fn spipf4rst_output_enbl(&mut self) -> Spipf4rstOutputEnblW<Scu0f0Spec> {
        Spipf4rstOutputEnblW::new(self, 27)
    }
}
#[doc = "QSPI Monitor Internal Mux Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0f0Spec;
impl crate::RegisterSpec for Scu0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0f0::R`](R) reader structure"]
impl crate::Readable for Scu0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0f0::W`](W) writer structure"]
impl crate::Writable for Scu0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0F0 to value 0"]
impl crate::Resettable for Scu0f0Spec {}
