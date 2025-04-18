#[doc = "Register `SPIPF608` reader"]
pub type R = crate::R<Spipf608Spec>;
#[doc = "Register `SPIPF608` writer"]
pub type W = crate::W<Spipf608Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf608::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf608::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf608Spec;
impl crate::RegisterSpec for Spipf608Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf608::R`](R) reader structure"]
impl crate::Readable for Spipf608Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf608::W`](W) writer structure"]
impl crate::Writable for Spipf608Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF608 to value 0"]
impl crate::Resettable for Spipf608Spec {}
