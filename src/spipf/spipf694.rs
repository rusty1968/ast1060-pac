#[doc = "Register `SPIPF694` reader"]
pub type R = crate::R<Spipf694Spec>;
#[doc = "Register `SPIPF694` writer"]
pub type W = crate::W<Spipf694Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf694::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf694::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf694Spec;
impl crate::RegisterSpec for Spipf694Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf694::R`](R) reader structure"]
impl crate::Readable for Spipf694Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf694::W`](W) writer structure"]
impl crate::Writable for Spipf694Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF694 to value 0"]
impl crate::Resettable for Spipf694Spec {}
