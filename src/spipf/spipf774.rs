#[doc = "Register `SPIPF774` reader"]
pub type R = crate::R<Spipf774Spec>;
#[doc = "Register `SPIPF774` writer"]
pub type W = crate::W<Spipf774Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf774::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf774::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf774Spec;
impl crate::RegisterSpec for Spipf774Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf774::R`](R) reader structure"]
impl crate::Readable for Spipf774Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf774::W`](W) writer structure"]
impl crate::Writable for Spipf774Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF774 to value 0"]
impl crate::Resettable for Spipf774Spec {}
