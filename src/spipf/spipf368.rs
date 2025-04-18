#[doc = "Register `SPIPF368` reader"]
pub type R = crate::R<Spipf368Spec>;
#[doc = "Register `SPIPF368` writer"]
pub type W = crate::W<Spipf368Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf368::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf368::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf368Spec;
impl crate::RegisterSpec for Spipf368Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf368::R`](R) reader structure"]
impl crate::Readable for Spipf368Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf368::W`](W) writer structure"]
impl crate::Writable for Spipf368Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF368 to value 0"]
impl crate::Resettable for Spipf368Spec {}
