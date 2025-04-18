#[doc = "Register `SCUF58` reader"]
pub type R = crate::R<Scuf58Spec>;
#[doc = "Register `SCUF58` writer"]
pub type W = crate::W<Scuf58Spec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA14` reader - Enable Write Protection of hlinkSCUA14"]
pub type EnblWrProtOfHlinkScua14R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA14` writer - Enable Write Protection of hlinkSCUA14"]
pub type EnblWrProtOfHlinkScua14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUA18` reader - Enable Write Protection of hlinkSCUA18"]
pub type EnblWrProtOfHlinkScua18R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA18` writer - Enable Write Protection of hlinkSCUA18"]
pub type EnblWrProtOfHlinkScua18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUA1C` reader - Enable Write Protection of hlinkSCUA1C"]
pub type EnblWrProtOfHlinkScua1cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA1C` writer - Enable Write Protection of hlinkSCUA1C"]
pub type EnblWrProtOfHlinkScua1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA50` reader - Enable Write Protection of hlinkSCUA50"]
pub type EnblWrProtOfHlinkScua50R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA50` writer - Enable Write Protection of hlinkSCUA50"]
pub type EnblWrProtOfHlinkScua50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUA54` reader - Enable Write Protection of hlinkSCUA54"]
pub type EnblWrProtOfHlinkScua54R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA54` writer - Enable Write Protection of hlinkSCUA54"]
pub type EnblWrProtOfHlinkScua54W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUA58` reader - Enable Write Protection of hlinkSCUA58"]
pub type EnblWrProtOfHlinkScua58R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUA58` writer - Enable Write Protection of hlinkSCUA58"]
pub type EnblWrProtOfHlinkScua58W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCUA14"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua14(&self) -> EnblWrProtOfHlinkScua14R {
        EnblWrProtOfHlinkScua14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCUA18"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua18(&self) -> EnblWrProtOfHlinkScua18R {
        EnblWrProtOfHlinkScua18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Protection of hlinkSCUA1C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua1c(&self) -> EnblWrProtOfHlinkScua1cR {
        EnblWrProtOfHlinkScua1cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCUA50"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua50(&self) -> EnblWrProtOfHlinkScua50R {
        EnblWrProtOfHlinkScua50R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCUA54"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua54(&self) -> EnblWrProtOfHlinkScua54R {
        EnblWrProtOfHlinkScua54R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCUA58"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua58(&self) -> EnblWrProtOfHlinkScua58R {
        EnblWrProtOfHlinkScua58R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCUA14"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua14(&mut self) -> EnblWrProtOfHlinkScua14W<Scuf58Spec> {
        EnblWrProtOfHlinkScua14W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCUA18"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua18(&mut self) -> EnblWrProtOfHlinkScua18W<Scuf58Spec> {
        EnblWrProtOfHlinkScua18W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Protection of hlinkSCUA1C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua1c(&mut self) -> EnblWrProtOfHlinkScua1cW<Scuf58Spec> {
        EnblWrProtOfHlinkScua1cW::new(self, 3)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCUA50"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua50(&mut self) -> EnblWrProtOfHlinkScua50W<Scuf58Spec> {
        EnblWrProtOfHlinkScua50W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCUA54"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua54(&mut self) -> EnblWrProtOfHlinkScua54W<Scuf58Spec> {
        EnblWrProtOfHlinkScua54W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCUA58"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scua58(&mut self) -> EnblWrProtOfHlinkScua58W<Scuf58Spec> {
        EnblWrProtOfHlinkScua58W::new(self, 10)
    }
}
#[doc = "Write Protect Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf58Spec;
impl crate::RegisterSpec for Scuf58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf58::R`](R) reader structure"]
impl crate::Readable for Scuf58Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf58::W`](W) writer structure"]
impl crate::Writable for Scuf58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF58 to value 0"]
impl crate::Resettable for Scuf58Spec {}
