#[doc = "Register `SCUF6C` reader"]
pub type R = crate::R<Scuf6cSpec>;
#[doc = "Register `SCUF6C` writer"]
pub type W = crate::W<Scuf6cSpec>;
#[doc = "Field `EnblWrProtOfHlinkSCUF80` reader - Enable Write Protection of hlinkSCUF80"]
pub type EnblWrProtOfHlinkScuf80R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUF80` writer - Enable Write Protection of hlinkSCUF80"]
pub type EnblWrProtOfHlinkScuf80W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUF88` reader - Enable Write Protection of hlinkSCUF88"]
pub type EnblWrProtOfHlinkScuf88R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUF88` writer - Enable Write Protection of hlinkSCUF88"]
pub type EnblWrProtOfHlinkScuf88W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFEC` reader - Enable Write Protection of hlinkSCUFEC"]
pub type EnblWrProtOfHlinkScufecR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFEC` writer - Enable Write Protection of hlinkSCUFEC"]
pub type EnblWrProtOfHlinkScufecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF80"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf80(&self) -> EnblWrProtOfHlinkScuf80R {
        EnblWrProtOfHlinkScuf80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF88"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf88(&self) -> EnblWrProtOfHlinkScuf88R {
        EnblWrProtOfHlinkScuf88R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFEC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufec(&self) -> EnblWrProtOfHlinkScufecR {
        EnblWrProtOfHlinkScufecR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF80"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf80(&mut self) -> EnblWrProtOfHlinkScuf80W<Scuf6cSpec> {
        EnblWrProtOfHlinkScuf80W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF88"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf88(&mut self) -> EnblWrProtOfHlinkScuf88W<Scuf6cSpec> {
        EnblWrProtOfHlinkScuf88W::new(self, 18)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFEC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufec(&mut self) -> EnblWrProtOfHlinkScufecW<Scuf6cSpec> {
        EnblWrProtOfHlinkScufecW::new(self, 31)
    }
}
#[doc = "Write Protect Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf6cSpec;
impl crate::RegisterSpec for Scuf6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf6c::R`](R) reader structure"]
impl crate::Readable for Scuf6cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf6c::W`](W) writer structure"]
impl crate::Writable for Scuf6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF6C to value 0"]
impl crate::Resettable for Scuf6cSpec {}
