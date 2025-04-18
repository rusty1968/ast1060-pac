#[doc = "Register `SPIPF438` reader"]
pub type R = crate::R<Spipf438Spec>;
#[doc = "Register `SPIPF438` writer"]
pub type W = crate::W<Spipf438Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf438::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf438::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf438Spec;
impl crate::RegisterSpec for Spipf438Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf438::R`](R) reader structure"]
impl crate::Readable for Spipf438Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf438::W`](W) writer structure"]
impl crate::Writable for Spipf438Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF438 to value 0"]
impl crate::Resettable for Spipf438Spec {}
