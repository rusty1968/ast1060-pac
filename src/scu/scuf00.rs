#[doc = "Register `SCUF00` reader"]
pub type R = crate::R<Scuf00Spec>;
#[doc = "Register `SCUF00` writer"]
pub type W = crate::W<Scuf00Spec>;
#[doc = "Field `Reserved5` reader - Reserved(0)"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCU040` reader - Enable Write Protection of hlinkSCU040"]
pub type EnblWrProtOfHlinkScu040R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU040` writer - Enable Write Protection of hlinkSCU040"]
pub type EnblWrProtOfHlinkScu040W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU048` reader - Enable Write Protection of hlinkSCU048"]
pub type EnblWrProtOfHlinkScu048R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU048` writer - Enable Write Protection of hlinkSCU048"]
pub type EnblWrProtOfHlinkScu048W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU060` reader - Enable Write Protection of hlinkSCU060"]
pub type EnblWrProtOfHlinkScu060R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU060` writer - Enable Write Protection of hlinkSCU060"]
pub type EnblWrProtOfHlinkScu060W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfHlinkSCU080` reader - Enable Write Protection of hlinkSCU080"]
pub type EnblWrProtOfHlinkScu080R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU080` writer - Enable Write Protection of hlinkSCU080"]
pub type EnblWrProtOfHlinkScu080W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU088` reader - Enable Write Protection of hlinkSCU088"]
pub type EnblWrProtOfHlinkScu088R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCU088` writer - Enable Write Protection of hlinkSCU088"]
pub type EnblWrProtOfHlinkScu088W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU040"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu040(&self) -> EnblWrProtOfHlinkScu040R {
        EnblWrProtOfHlinkScu040R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU048"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu048(&self) -> EnblWrProtOfHlinkScu048R {
        EnblWrProtOfHlinkScu048R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU060"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu060(&self) -> EnblWrProtOfHlinkScu060R {
        EnblWrProtOfHlinkScu060R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU080"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu080(&self) -> EnblWrProtOfHlinkScu080R {
        EnblWrProtOfHlinkScu080R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCU088"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu088(&self) -> EnblWrProtOfHlinkScu088R {
        EnblWrProtOfHlinkScu088R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU040"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu040(&mut self) -> EnblWrProtOfHlinkScu040W<Scuf00Spec> {
        EnblWrProtOfHlinkScu040W::new(self, 8)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU048"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu048(&mut self) -> EnblWrProtOfHlinkScu048W<Scuf00Spec> {
        EnblWrProtOfHlinkScu048W::new(self, 10)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU060"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu060(&mut self) -> EnblWrProtOfHlinkScu060W<Scuf00Spec> {
        EnblWrProtOfHlinkScu060W::new(self, 12)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCU080"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu080(&mut self) -> EnblWrProtOfHlinkScu080W<Scuf00Spec> {
        EnblWrProtOfHlinkScu080W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCU088"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scu088(&mut self) -> EnblWrProtOfHlinkScu088W<Scuf00Spec> {
        EnblWrProtOfHlinkScu088W::new(self, 18)
    }
}
#[doc = "Write Protect Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf00Spec;
impl crate::RegisterSpec for Scuf00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf00::R`](R) reader structure"]
impl crate::Readable for Scuf00Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf00::W`](W) writer structure"]
impl crate::Writable for Scuf00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF00 to value 0"]
impl crate::Resettable for Scuf00Spec {}
