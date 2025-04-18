#[doc = "Register `SPIPF108` reader"]
pub type R = crate::R<Spipf108Spec>;
#[doc = "Register `SPIPF108` writer"]
pub type W = crate::W<Spipf108Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf108Spec;
impl crate::RegisterSpec for Spipf108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf108::R`](R) reader structure"]
impl crate::Readable for Spipf108Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf108::W`](W) writer structure"]
impl crate::Writable for Spipf108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF108 to value 0"]
impl crate::Resettable for Spipf108Spec {}
