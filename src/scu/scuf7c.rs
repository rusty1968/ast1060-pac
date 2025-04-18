#[doc = "Register `SCUF7C` reader"]
pub type R = crate::R<Scuf7cSpec>;
#[doc = "Register `SCUF7C` writer"]
pub type W = crate::W<Scuf7cSpec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `EnblWrProtOfHlinkSCUF90` reader - Enable Write Protection of hlinkSCUF90"]
pub type EnblWrProtOfHlinkScuf90R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUF90` writer - Enable Write Protection of hlinkSCUF90"]
pub type EnblWrProtOfHlinkScuf90W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUF98` reader - Enable Write Protection of hlinkSCUF98"]
pub type EnblWrProtOfHlinkScuf98R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUF98` writer - Enable Write Protection of hlinkSCUF98"]
pub type EnblWrProtOfHlinkScuf98W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUF9C` reader - Enable Write Protection of hlinkSCUF9C"]
pub type EnblWrProtOfHlinkScuf9cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUF9C` writer - Enable Write Protection of hlinkSCUF9C"]
pub type EnblWrProtOfHlinkScuf9cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFB0` reader - Enable Write Protection of hlinkSCUFB0"]
pub type EnblWrProtOfHlinkScufb0R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFB0` writer - Enable Write Protection of hlinkSCUFB0"]
pub type EnblWrProtOfHlinkScufb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFB4` reader - Enable Write Protection of hlinkSCUFB4"]
pub type EnblWrProtOfHlinkScufb4R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFB4` writer - Enable Write Protection of hlinkSCUFB4"]
pub type EnblWrProtOfHlinkScufb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFB8` reader - Enable Write Protection of hlinkSCUFB8"]
pub type EnblWrProtOfHlinkScufb8R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFB8` writer - Enable Write Protection of hlinkSCUFB8"]
pub type EnblWrProtOfHlinkScufb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFBC` reader - Enable Write Protection of hlinkSCUFBC"]
pub type EnblWrProtOfHlinkScufbcR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFBC` writer - Enable Write Protection of hlinkSCUFBC"]
pub type EnblWrProtOfHlinkScufbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFD0` reader - Enable Write Protection of hlinkSCUFD0"]
pub type EnblWrProtOfHlinkScufd0R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFD0` writer - Enable Write Protection of hlinkSCUFD0"]
pub type EnblWrProtOfHlinkScufd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFD4` reader - Enable Write Protection of hlinkSCUFD4"]
pub type EnblWrProtOfHlinkScufd4R = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFD4` writer - Enable Write Protection of hlinkSCUFD4"]
pub type EnblWrProtOfHlinkScufd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfHlinkSCUFFC` reader - Enable Write Protection of hlinkSCUFFC"]
pub type EnblWrProtOfHlinkScuffcR = crate::BitReader;
#[doc = "Field `EnblWrProtOfHlinkSCUFFC` writer - Enable Write Protection of hlinkSCUFFC"]
pub type EnblWrProtOfHlinkScuffcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF90"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf90(&self) -> EnblWrProtOfHlinkScuf90R {
        EnblWrProtOfHlinkScuf90R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF98"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf98(&self) -> EnblWrProtOfHlinkScuf98R {
        EnblWrProtOfHlinkScuf98R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Protection of hlinkSCUF9C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf9c(&self) -> EnblWrProtOfHlinkScuf9cR {
        EnblWrProtOfHlinkScuf9cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Protection of hlinkSCUFB0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufb0(&self) -> EnblWrProtOfHlinkScufb0R {
        EnblWrProtOfHlinkScufb0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Protection of hlinkSCUFB4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufb4(&self) -> EnblWrProtOfHlinkScufb4R {
        EnblWrProtOfHlinkScufb4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Write Protection of hlinkSCUFB8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufb8(&self) -> EnblWrProtOfHlinkScufb8R {
        EnblWrProtOfHlinkScufb8R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of hlinkSCUFBC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufbc(&self) -> EnblWrProtOfHlinkScufbcR {
        EnblWrProtOfHlinkScufbcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Protection of hlinkSCUFD0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufd0(&self) -> EnblWrProtOfHlinkScufd0R {
        EnblWrProtOfHlinkScufd0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Protection of hlinkSCUFD4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufd4(&self) -> EnblWrProtOfHlinkScufd4R {
        EnblWrProtOfHlinkScufd4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFFC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuffc(&self) -> EnblWrProtOfHlinkScuffcR {
        EnblWrProtOfHlinkScuffcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF90"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf90(&mut self) -> EnblWrProtOfHlinkScuf90W<Scuf7cSpec> {
        EnblWrProtOfHlinkScuf90W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF98"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf98(&mut self) -> EnblWrProtOfHlinkScuf98W<Scuf7cSpec> {
        EnblWrProtOfHlinkScuf98W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Protection of hlinkSCUF9C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuf9c(&mut self) -> EnblWrProtOfHlinkScuf9cW<Scuf7cSpec> {
        EnblWrProtOfHlinkScuf9cW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Protection of hlinkSCUFB0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufb0(&mut self) -> EnblWrProtOfHlinkScufb0W<Scuf7cSpec> {
        EnblWrProtOfHlinkScufb0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Protection of hlinkSCUFB4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufb4(&mut self) -> EnblWrProtOfHlinkScufb4W<Scuf7cSpec> {
        EnblWrProtOfHlinkScufb4W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Write Protection of hlinkSCUFB8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufb8(&mut self) -> EnblWrProtOfHlinkScufb8W<Scuf7cSpec> {
        EnblWrProtOfHlinkScufb8W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of hlinkSCUFBC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufbc(&mut self) -> EnblWrProtOfHlinkScufbcW<Scuf7cSpec> {
        EnblWrProtOfHlinkScufbcW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Protection of hlinkSCUFD0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufd0(&mut self) -> EnblWrProtOfHlinkScufd0W<Scuf7cSpec> {
        EnblWrProtOfHlinkScufd0W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Protection of hlinkSCUFD4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scufd4(&mut self) -> EnblWrProtOfHlinkScufd4W<Scuf7cSpec> {
        EnblWrProtOfHlinkScufd4W::new(self, 25)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFFC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_hlink_scuffc(&mut self) -> EnblWrProtOfHlinkScuffcW<Scuf7cSpec> {
        EnblWrProtOfHlinkScuffcW::new(self, 31)
    }
}
#[doc = "Write Protect Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf7c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf7c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf7cSpec;
impl crate::RegisterSpec for Scuf7cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf7c::R`](R) reader structure"]
impl crate::Readable for Scuf7cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf7c::W`](W) writer structure"]
impl crate::Writable for Scuf7cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF7C to value 0"]
impl crate::Resettable for Scuf7cSpec {}
