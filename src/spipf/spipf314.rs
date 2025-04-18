#[doc = "Register `SPIPF314` reader"]
pub type R = crate::R<Spipf314Spec>;
#[doc = "Register `SPIPF314` writer"]
pub type W = crate::W<Spipf314Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf314::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf314::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf314Spec;
impl crate::RegisterSpec for Spipf314Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf314::R`](R) reader structure"]
impl crate::Readable for Spipf314Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf314::W`](W) writer structure"]
impl crate::Writable for Spipf314Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF314 to value 0"]
impl crate::Resettable for Spipf314Spec {}
