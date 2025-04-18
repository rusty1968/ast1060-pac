#[doc = "Register `SPIPF568` reader"]
pub type R = crate::R<Spipf568Spec>;
#[doc = "Register `SPIPF568` writer"]
pub type W = crate::W<Spipf568Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf568::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf568::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf568Spec;
impl crate::RegisterSpec for Spipf568Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf568::R`](R) reader structure"]
impl crate::Readable for Spipf568Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf568::W`](W) writer structure"]
impl crate::Writable for Spipf568Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF568 to value 0"]
impl crate::Resettable for Spipf568Spec {}
