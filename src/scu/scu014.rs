#[doc = "Register `SCU014` reader"]
pub type R = crate::R<Scu014Spec>;
#[doc = "Register `SCU014` writer"]
pub type W = crate::W<Scu014Spec>;
#[doc = "Field `Reserved3` reader - Reserved for backward compatible"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `ChipBoundingOption1` reader - Chip bounding option"]
pub type ChipBoundingOption1R = crate::FieldReader;
#[doc = "Field `HwRevID1` reader - Hardware Revision ID"]
pub type HwRevId1R = crate::FieldReader;
#[doc = "Field `Reserved1` reader - Reserved (0xA0)"]
pub type Reserved1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reserved for backward compatible"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Chip bounding option"]
    #[inline(always)]
    pub fn chip_bounding_option1(&self) -> ChipBoundingOption1R {
        ChipBoundingOption1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Hardware Revision ID"]
    #[inline(always)]
    pub fn hw_rev_id1(&self) -> HwRevId1R {
        HwRevId1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved (0xA0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Silicon Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu014Spec;
impl crate::RegisterSpec for Scu014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu014::R`](R) reader structure"]
impl crate::Readable for Scu014Spec {}
#[doc = "`write(|w| ..)` method takes [`scu014::W`](W) writer structure"]
impl crate::Writable for Scu014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU014 to value 0"]
impl crate::Resettable for Scu014Spec {}
