#[doc = "Register `SPIPF734` reader"]
pub type R = crate::R<Spipf734Spec>;
#[doc = "Register `SPIPF734` writer"]
pub type W = crate::W<Spipf734Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf734::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf734::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf734Spec;
impl crate::RegisterSpec for Spipf734Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf734::R`](R) reader structure"]
impl crate::Readable for Spipf734Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf734::W`](W) writer structure"]
impl crate::Writable for Spipf734Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF734 to value 0"]
impl crate::Resettable for Spipf734Spec {}
