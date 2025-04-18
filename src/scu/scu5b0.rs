#[doc = "Register `SCU5B0` reader"]
pub type R = crate::R<Scu5b0Spec>;
#[doc = "Register `SCU5B0` writer"]
pub type W = crate::W<Scu5b0Spec>;
#[doc = "Field `ChipUniqueIDBit63` reader - 0] read back"]
pub type ChipUniqueIdbit63R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 0] read back"]
    #[inline(always)]
    pub fn chip_unique_idbit63(&self) -> ChipUniqueIdbit63R {
        ChipUniqueIdbit63R::new(self.bits)
    }
}
impl W {}
#[doc = "Chip Unique ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5b0Spec;
impl crate::RegisterSpec for Scu5b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5b0::R`](R) reader structure"]
impl crate::Readable for Scu5b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5b0::W`](W) writer structure"]
impl crate::Writable for Scu5b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5B0 to value 0"]
impl crate::Resettable for Scu5b0Spec {}
