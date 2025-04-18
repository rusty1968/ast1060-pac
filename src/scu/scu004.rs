#[doc = "Register `SCU004` reader"]
pub type R = crate::R<Scu004Spec>;
#[doc = "Register `SCU004` writer"]
pub type W = crate::W<Scu004Spec>;
#[doc = "Field `Reserved2` reader - Reserved for backward compatible"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `ChipBoundingOption` reader - Chip bounding option"]
pub type ChipBoundingOptionR = crate::FieldReader;
#[doc = "Field `HwRevID` reader - Hardware Revision ID"]
pub type HwRevIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reserved for backward compatible"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Chip bounding option"]
    #[inline(always)]
    pub fn chip_bounding_option(&self) -> ChipBoundingOptionR {
        ChipBoundingOptionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Hardware Revision ID"]
    #[inline(always)]
    pub fn hw_rev_id(&self) -> HwRevIdR {
        HwRevIdR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Silicon Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu004Spec;
impl crate::RegisterSpec for Scu004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu004::R`](R) reader structure"]
impl crate::Readable for Scu004Spec {}
#[doc = "`write(|w| ..)` method takes [`scu004::W`](W) writer structure"]
impl crate::Writable for Scu004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU004 to value 0"]
impl crate::Resettable for Scu004Spec {}
