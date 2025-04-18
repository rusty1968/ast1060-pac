#[doc = "Register `SPIPF680` reader"]
pub type R = crate::R<Spipf680Spec>;
#[doc = "Register `SPIPF680` writer"]
pub type W = crate::W<Spipf680Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf680::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf680::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf680Spec;
impl crate::RegisterSpec for Spipf680Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf680::R`](R) reader structure"]
impl crate::Readable for Spipf680Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf680::W`](W) writer structure"]
impl crate::Writable for Spipf680Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF680 to value 0"]
impl crate::Resettable for Spipf680Spec {}
