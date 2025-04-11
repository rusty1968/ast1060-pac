#[doc = "Register `SPIPF654` reader"]
pub type R = crate::R<Spipf654Spec>;
#[doc = "Register `SPIPF654` writer"]
pub type W = crate::W<Spipf654Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf654::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf654::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf654Spec;
impl crate::RegisterSpec for Spipf654Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf654::R`](R) reader structure"]
impl crate::Readable for Spipf654Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf654::W`](W) writer structure"]
impl crate::Writable for Spipf654Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF654 to value 0"]
impl crate::Resettable for Spipf654Spec {}
