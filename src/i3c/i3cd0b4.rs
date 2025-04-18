#[doc = "Register `I3CD0B4` reader"]
pub type R = crate::R<I3cd0b4Spec>;
#[doc = "Register `I3CD0B4` writer"]
pub type W = crate::W<I3cd0b4Spec>;
#[doc = "Field `I3CODLCNT` reader - I3C_OD_LCNT"]
pub type I3codlcntR = crate::FieldReader;
#[doc = "Field `I3CODLCNT` writer - I3C_OD_LCNT"]
pub type I3codlcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD158` reader - These bits in SCL I3C OD timing register are reserved."]
pub type Rsvd158R = crate::FieldReader;
#[doc = "Field `I3CODHCNT` reader - I3C_OD_HCNT"]
pub type I3codhcntR = crate::FieldReader;
#[doc = "Field `I3CODHCNT` writer - I3C_OD_HCNT"]
pub type I3codhcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD3124` reader - These bits in SCL I3C OD timing register are reserved."]
pub type Rsvd3124R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - I3C_OD_LCNT"]
    #[inline(always)]
    pub fn i3codlcnt(&self) -> I3codlcntR {
        I3codlcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits in SCL I3C OD timing register are reserved."]
    #[inline(always)]
    pub fn rsvd158(&self) -> Rsvd158R {
        Rsvd158R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I3C_OD_HCNT"]
    #[inline(always)]
    pub fn i3codhcnt(&self) -> I3codhcntR {
        I3codhcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - These bits in SCL I3C OD timing register are reserved."]
    #[inline(always)]
    pub fn rsvd3124(&self) -> Rsvd3124R {
        Rsvd3124R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I3C_OD_LCNT"]
    #[inline(always)]
    pub fn i3codlcnt(&mut self) -> I3codlcntW<I3cd0b4Spec> {
        I3codlcntW::new(self, 0)
    }
    #[doc = "Bits 16:23 - I3C_OD_HCNT"]
    #[inline(always)]
    pub fn i3codhcnt(&mut self) -> I3codhcntW<I3cd0b4Spec> {
        I3codhcntW::new(self, 16)
    }
}
#[doc = "SCL I3C Open Drain Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0b4Spec;
impl crate::RegisterSpec for I3cd0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0b4::R`](R) reader structure"]
impl crate::Readable for I3cd0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0b4::W`](W) writer structure"]
impl crate::Writable for I3cd0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0B4 to value 0"]
impl crate::Resettable for I3cd0b4Spec {}
