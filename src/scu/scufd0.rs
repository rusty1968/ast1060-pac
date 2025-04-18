#[doc = "Register `SCUFD0` reader"]
pub type R = crate::R<Scufd0Spec>;
#[doc = "Register `SCUFD0` writer"]
pub type W = crate::W<Scufd0Spec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU830` reader - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu830R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU830` writer - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu830W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU834` reader - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu834R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU834` writer - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu834W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU850` reader - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu850R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU850` writer - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu850W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU854` reader - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu854R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU854` writer - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu854W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU858` reader - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu858R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU858` writer - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu858W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU85C` reader - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu85cR = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU85C` writer - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu85cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU870` reader - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu870R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU870` writer - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu870W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU874` reader - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu874R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU874` writer - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu874W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU878` reader - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu878R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU878` writer - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu878W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU87C` reader - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu87cR = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU87C` writer - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu87cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu830(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu830R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu830R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu834(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu834R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu834R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu850(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu850R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu850R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu854(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu854R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu854R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu858(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu858R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu858R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu85c(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu85cR {
        EnblHlinkArmrstnasRstSrcOfHlinkScu85cR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu870(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu870R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu870R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu874(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu874R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu874R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu878(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu878R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu878R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu87c(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu87cR {
        EnblHlinkArmrstnasRstSrcOfHlinkScu87cR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu830(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu830W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu830W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu834(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu834W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu834W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu850(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu850W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu850W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu854(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu854W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu854W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu858(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu858W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu858W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu85c(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu85cW<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu85cW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu870(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu870W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu870W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu874(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu874W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu874W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu878(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu878W<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu878W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu87c(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu87cW<Scufd0Spec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu87cW::new(self, 15)
    }
}
#[doc = "Reset Source Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scufd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scufd0Spec;
impl crate::RegisterSpec for Scufd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scufd0::R`](R) reader structure"]
impl crate::Readable for Scufd0Spec {}
#[doc = "`write(|w| ..)` method takes [`scufd0::W`](W) writer structure"]
impl crate::Writable for Scufd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUFD0 to value 0"]
impl crate::Resettable for Scufd0Spec {}
