#[doc = "Register `SPIPF480` reader"]
pub type R = crate::R<Spipf480Spec>;
#[doc = "Register `SPIPF480` writer"]
pub type W = crate::W<Spipf480Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf480::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf480::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf480Spec;
impl crate::RegisterSpec for Spipf480Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf480::R`](R) reader structure"]
impl crate::Readable for Spipf480Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf480::W`](W) writer structure"]
impl crate::Writable for Spipf480Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF480 to value 0"]
impl crate::Resettable for Spipf480Spec {}
