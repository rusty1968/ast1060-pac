#[doc = "Register `SCUF1C` reader"]
pub type R = crate::R<Scuf1cSpec>;
#[doc = "Register `SCUF1C` writer"]
pub type W = crate::W<Scuf1cSpec>;
#[doc = "Field `EnblWrProtOfHlinkSCU310` reader - Enable Write Protection of hlinkSCU310"]
pub type EnblWrProtOfHlinkScu310R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU310` writer - Enable Write Protection of hlinkSCU310"]
pub type EnblWrProtOfHlinkScu310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU314` reader - Enable Write Protection of hlinkSCU314"]
pub type EnblWrProtOfHlinkScu314R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU314` writer - Enable Write Protection of hlinkSCU314"]
pub type EnblWrProtOfHlinkScu314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCU330` reader - Enable Write Protection of hlinkSCU330"]
pub type EnblWrProtOfHlinkScu330R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU330` writer - Enable Write Protection of hlinkSCU330"]
pub type EnblWrProtOfHlinkScu330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU334` reader - Enable Write Protection of hlinkSCU334"]
pub type EnblWrProtOfHlinkScu334R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU334` writer - Enable Write Protection of hlinkSCU334"]
pub type EnblWrProtOfHlinkScu334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU338` reader - Enable Write Protection of hlinkSCU338"]
pub type EnblWrProtOfHlinkScu338R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU338` writer - Enable Write Protection of hlinkSCU338"]
pub type EnblWrProtOfHlinkScu338W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU33C` reader - Enable Write Protection of hlinkSCU33C"]
pub type EnblWrProtOfHlinkScu33cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU33C` writer - Enable Write Protection of hlinkSCU33C"]
pub type EnblWrProtOfHlinkScu33cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU310"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu310(&self) -> EnblWrProtOfHlinkScu310R {
        EnblWrProtOfHlinkScu310R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu314(&self) -> EnblWrProtOfHlinkScu314R {
        EnblWrProtOfHlinkScu314R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu330(&self) -> EnblWrProtOfHlinkScu330R {
        EnblWrProtOfHlinkScu330R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu334(&self) -> EnblWrProtOfHlinkScu334R {
        EnblWrProtOfHlinkScu334R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Write Protection of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu338(&self) -> EnblWrProtOfHlinkScu338R {
        EnblWrProtOfHlinkScu338R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu33c(&self) -> EnblWrProtOfHlinkScu33cR {
        EnblWrProtOfHlinkScu33cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU310"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu310(&mut self) -> EnblWrProtOfHlinkScu310W<Scuf1cSpec> {
        EnblWrProtOfHlinkScu310W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu314(&mut self) -> EnblWrProtOfHlinkScu314W<Scuf1cSpec> {
        EnblWrProtOfHlinkScu314W::new(self, 1)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu330(&mut self) -> EnblWrProtOfHlinkScu330W<Scuf1cSpec> {
        EnblWrProtOfHlinkScu330W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu334(&mut self) -> EnblWrProtOfHlinkScu334W<Scuf1cSpec> {
        EnblWrProtOfHlinkScu334W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Write Protection of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu338(&mut self) -> EnblWrProtOfHlinkScu338W<Scuf1cSpec> {
        EnblWrProtOfHlinkScu338W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu33c(&mut self) -> EnblWrProtOfHlinkScu33cW<Scuf1cSpec> {
        EnblWrProtOfHlinkScu33cW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scuf1cSpec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scuf1cSpec> {
        Reserved1W::new(self, 10)
    }
}
#[doc = "Write Protect Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf1cSpec;
impl crate::RegisterSpec for Scuf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf1c::R`](R) reader structure"]
impl crate::Readable for Scuf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf1c::W`](W) writer structure"]
impl crate::Writable for Scuf1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF1C to value 0"]
impl crate::Resettable for Scuf1cSpec {}
