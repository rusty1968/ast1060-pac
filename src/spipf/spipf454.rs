#[doc = "Register `SPIPF454` reader"]
pub type R = crate::R<Spipf454Spec>;
#[doc = "Register `SPIPF454` writer"]
pub type W = crate::W<Spipf454Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf454::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf454::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf454Spec;
impl crate::RegisterSpec for Spipf454Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf454::R`](R) reader structure"]
impl crate::Readable for Spipf454Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf454::W`](W) writer structure"]
impl crate::Writable for Spipf454Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF454 to value 0"]
impl crate::Resettable for Spipf454Spec {}
