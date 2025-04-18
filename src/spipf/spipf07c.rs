#[doc = "Register `SPIPF07C` reader"]
pub type R = crate::R<Spipf07cSpec>;
#[doc = "Register `SPIPF07C` writer"]
pub type W = crate::W<Spipf07cSpec>;
#[doc = "Field `WrDisOfHlinkSPIPF000` reader - Write Disable of hlinkSPIPF000"]
pub type WrDisOfHlinkSpipf000R = crate::BitReader;
#[doc = "Field `WrDisOfHlinkSPIPF000` writer - Write Disable of hlinkSPIPF000"]
pub type WrDisOfHlinkSpipf000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfHlinkSPIPF004` reader - Write Disable of hlinkSPIPF004"]
pub type WrDisOfHlinkSpipf004R = crate::BitReader;
#[doc = "Field `WrDisOfHlinkSPIPF004` writer - Write Disable of hlinkSPIPF004"]
pub type WrDisOfHlinkSpipf004W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfHlinkSPIPF008` reader - Write Disable of hlinkSPIPF008"]
pub type WrDisOfHlinkSpipf008R = crate::BitReader;
#[doc = "Field `WrDisOfHlinkSPIPF008` writer - Write Disable of hlinkSPIPF008"]
pub type WrDisOfHlinkSpipf008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfHlinkSPIPF010` reader - Write Disable of hlinkSPIPF010"]
pub type WrDisOfHlinkSpipf010R = crate::BitReader;
#[doc = "Field `WrDisOfHlinkSPIPF010` writer - Write Disable of hlinkSPIPF010"]
pub type WrDisOfHlinkSpipf010W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfHlinkSPIPF014` reader - Write Disable of hlinkSPIPF014"]
pub type WrDisOfHlinkSpipf014R = crate::BitReader;
#[doc = "Field `WrDisOfHlinkSPIPF014` writer - Write Disable of hlinkSPIPF014"]
pub type WrDisOfHlinkSpipf014W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `WrDisOfHlinkSPIPFWA` reader - Write Disable of hlinkSPIPFWA"]
pub type WrDisOfHlinkSpipfwaR = crate::BitReader;
#[doc = "Field `WrDisOfHlinkSPIPFWA` writer - Write Disable of hlinkSPIPFWA"]
pub type WrDisOfHlinkSpipfwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrDisOfHlinkSPIPFRA` reader - Write Disable of hlinkSPIPFRA"]
pub type WrDisOfHlinkSpipfraR = crate::BitReader;
#[doc = "Field `WrDisOfHlinkSPIPFRA` writer - Write Disable of hlinkSPIPFRA"]
pub type WrDisOfHlinkSpipfraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Disable of hlinkSPIPF000"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf000(&self) -> WrDisOfHlinkSpipf000R {
        WrDisOfHlinkSpipf000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Disable of hlinkSPIPF004"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf004(&self) -> WrDisOfHlinkSpipf004R {
        WrDisOfHlinkSpipf004R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Disable of hlinkSPIPF008"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf008(&self) -> WrDisOfHlinkSpipf008R {
        WrDisOfHlinkSpipf008R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Disable of hlinkSPIPF010"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf010(&self) -> WrDisOfHlinkSpipf010R {
        WrDisOfHlinkSpipf010R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Disable of hlinkSPIPF014"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf014(&self) -> WrDisOfHlinkSpipf014R {
        WrDisOfHlinkSpipf014R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:29 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 7) & 0x007f_ffff)
    }
    #[doc = "Bit 30 - Write Disable of hlinkSPIPFWA"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipfwa(&self) -> WrDisOfHlinkSpipfwaR {
        WrDisOfHlinkSpipfwaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Disable of hlinkSPIPFRA"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipfra(&self) -> WrDisOfHlinkSpipfraR {
        WrDisOfHlinkSpipfraR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Disable of hlinkSPIPF000"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf000(&mut self) -> WrDisOfHlinkSpipf000W<Spipf07cSpec> {
        WrDisOfHlinkSpipf000W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Disable of hlinkSPIPF004"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf004(&mut self) -> WrDisOfHlinkSpipf004W<Spipf07cSpec> {
        WrDisOfHlinkSpipf004W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Disable of hlinkSPIPF008"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf008(&mut self) -> WrDisOfHlinkSpipf008W<Spipf07cSpec> {
        WrDisOfHlinkSpipf008W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Spipf07cSpec> {
        Reserved1W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Disable of hlinkSPIPF010"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf010(&mut self) -> WrDisOfHlinkSpipf010W<Spipf07cSpec> {
        WrDisOfHlinkSpipf010W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Disable of hlinkSPIPF014"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipf014(&mut self) -> WrDisOfHlinkSpipf014W<Spipf07cSpec> {
        WrDisOfHlinkSpipf014W::new(self, 5)
    }
    #[doc = "Bit 30 - Write Disable of hlinkSPIPFWA"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipfwa(&mut self) -> WrDisOfHlinkSpipfwaW<Spipf07cSpec> {
        WrDisOfHlinkSpipfwaW::new(self, 30)
    }
    #[doc = "Bit 31 - Write Disable of hlinkSPIPFRA"]
    #[inline(always)]
    pub fn wr_dis_of_hlink_spipfra(&mut self) -> WrDisOfHlinkSpipfraW<Spipf07cSpec> {
        WrDisOfHlinkSpipfraW::new(self, 31)
    }
}
#[doc = "Write Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf07cSpec;
impl crate::RegisterSpec for Spipf07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf07c::R`](R) reader structure"]
impl crate::Readable for Spipf07cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf07c::W`](W) writer structure"]
impl crate::Writable for Spipf07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF07C to value 0"]
impl crate::Resettable for Spipf07cSpec {}
