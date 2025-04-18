#[doc = "Register `SCUF50` reader"]
pub type R = crate::R<Scuf50Spec>;
#[doc = "Register `SCUF50` writer"]
pub type W = crate::W<Scuf50Spec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCU830` reader - Enable Write Protection of hlinkSCU830"]
pub type EnblWrProtOfHlinkScu830R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU830` writer - Enable Write Protection of hlinkSCU830"]
pub type EnblWrProtOfHlinkScu830W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU834` reader - Enable Write Protection of hlinkSCU834"]
pub type EnblWrProtOfHlinkScu834R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU834` writer - Enable Write Protection of hlinkSCU834"]
pub type EnblWrProtOfHlinkScu834W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCU850` reader - Enable Write Protection of hlinkSCU850"]
pub type EnblWrProtOfHlinkScu850R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU850` writer - Enable Write Protection of hlinkSCU850"]
pub type EnblWrProtOfHlinkScu850W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU854` reader - Enable Write Protection of hlinkSCU854"]
pub type EnblWrProtOfHlinkScu854R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU854` writer - Enable Write Protection of hlinkSCU854"]
pub type EnblWrProtOfHlinkScu854W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU858` reader - Enable Write Protection of hlinkSCU858"]
pub type EnblWrProtOfHlinkScu858R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU858` writer - Enable Write Protection of hlinkSCU858"]
pub type EnblWrProtOfHlinkScu858W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU85C` reader - Enable Write Protection of hlinkSCU85C"]
pub type EnblWrProtOfHlinkScu85cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU85C` writer - Enable Write Protection of hlinkSCU85C"]
pub type EnblWrProtOfHlinkScu85cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU870` reader - Enable Write Protection of hlinkSCU870"]
pub type EnblWrProtOfHlinkScu870R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU870` writer - Enable Write Protection of hlinkSCU870"]
pub type EnblWrProtOfHlinkScu870W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU874` reader - Enable Write Protection of hlinkSCU874"]
pub type EnblWrProtOfHlinkScu874R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU874` writer - Enable Write Protection of hlinkSCU874"]
pub type EnblWrProtOfHlinkScu874W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU878` reader - Enable Write Protection of hlinkSCU878"]
pub type EnblWrProtOfHlinkScu878R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU878` writer - Enable Write Protection of hlinkSCU878"]
pub type EnblWrProtOfHlinkScu878W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCU87C` reader - Enable Write Protection of hlinkSCU87C"]
pub type EnblWrProtOfHlinkScu87cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU87C` writer - Enable Write Protection of hlinkSCU87C"]
pub type EnblWrProtOfHlinkScu87cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu830(&self) -> EnblWrProtOfHlinkScu830R {
        EnblWrProtOfHlinkScu830R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu834(&self) -> EnblWrProtOfHlinkScu834R {
        EnblWrProtOfHlinkScu834R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu850(&self) -> EnblWrProtOfHlinkScu850R {
        EnblWrProtOfHlinkScu850R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu854(&self) -> EnblWrProtOfHlinkScu854R {
        EnblWrProtOfHlinkScu854R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu858(&self) -> EnblWrProtOfHlinkScu858R {
        EnblWrProtOfHlinkScu858R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Protection of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu85c(&self) -> EnblWrProtOfHlinkScu85cR {
        EnblWrProtOfHlinkScu85cR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu870(&self) -> EnblWrProtOfHlinkScu870R {
        EnblWrProtOfHlinkScu870R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Protection of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu874(&self) -> EnblWrProtOfHlinkScu874R {
        EnblWrProtOfHlinkScu874R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Write Protection of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu878(&self) -> EnblWrProtOfHlinkScu878R {
        EnblWrProtOfHlinkScu878R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu87c(&self) -> EnblWrProtOfHlinkScu87cR {
        EnblWrProtOfHlinkScu87cR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu830(&mut self) -> EnblWrProtOfHlinkScu830W<Scuf50Spec> {
        EnblWrProtOfHlinkScu830W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu834(&mut self) -> EnblWrProtOfHlinkScu834W<Scuf50Spec> {
        EnblWrProtOfHlinkScu834W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu850(&mut self) -> EnblWrProtOfHlinkScu850W<Scuf50Spec> {
        EnblWrProtOfHlinkScu850W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu854(&mut self) -> EnblWrProtOfHlinkScu854W<Scuf50Spec> {
        EnblWrProtOfHlinkScu854W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu858(&mut self) -> EnblWrProtOfHlinkScu858W<Scuf50Spec> {
        EnblWrProtOfHlinkScu858W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Protection of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu85c(&mut self) -> EnblWrProtOfHlinkScu85cW<Scuf50Spec> {
        EnblWrProtOfHlinkScu85cW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu870(&mut self) -> EnblWrProtOfHlinkScu870W<Scuf50Spec> {
        EnblWrProtOfHlinkScu870W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Protection of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu874(&mut self) -> EnblWrProtOfHlinkScu874W<Scuf50Spec> {
        EnblWrProtOfHlinkScu874W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Write Protection of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu878(&mut self) -> EnblWrProtOfHlinkScu878W<Scuf50Spec> {
        EnblWrProtOfHlinkScu878W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu87c(&mut self) -> EnblWrProtOfHlinkScu87cW<Scuf50Spec> {
        EnblWrProtOfHlinkScu87cW::new(self, 15)
    }
}
#[doc = "Write Protect Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf50Spec;
impl crate::RegisterSpec for Scuf50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf50::R`](R) reader structure"]
impl crate::Readable for Scuf50Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf50::W`](W) writer structure"]
impl crate::Writable for Scuf50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF50 to value 0"]
impl crate::Resettable for Scuf50Spec {}
