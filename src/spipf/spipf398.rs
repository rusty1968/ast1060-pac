#[doc = "Register `SPIPF398` reader"]
pub type R = crate::R<Spipf398Spec>;
#[doc = "Register `SPIPF398` writer"]
pub type W = crate::W<Spipf398Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf398::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf398::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf398Spec;
impl crate::RegisterSpec for Spipf398Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf398::R`](R) reader structure"]
impl crate::Readable for Spipf398Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf398::W`](W) writer structure"]
impl crate::Writable for Spipf398Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF398 to value 0"]
impl crate::Resettable for Spipf398Spec {}
