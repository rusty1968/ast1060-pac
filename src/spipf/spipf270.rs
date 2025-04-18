#[doc = "Register `SPIPF270` reader"]
pub type R = crate::R<Spipf270Spec>;
#[doc = "Register `SPIPF270` writer"]
pub type W = crate::W<Spipf270Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf270::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf270::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf270Spec;
impl crate::RegisterSpec for Spipf270Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf270::R`](R) reader structure"]
impl crate::Readable for Spipf270Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf270::W`](W) writer structure"]
impl crate::Writable for Spipf270Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF270 to value 0"]
impl crate::Resettable for Spipf270Spec {}
