#[doc = "Register `SPIPF518` reader"]
pub type R = crate::R<Spipf518Spec>;
#[doc = "Register `SPIPF518` writer"]
pub type W = crate::W<Spipf518Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf518::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf518::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf518Spec;
impl crate::RegisterSpec for Spipf518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf518::R`](R) reader structure"]
impl crate::Readable for Spipf518Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf518::W`](W) writer structure"]
impl crate::Writable for Spipf518Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF518 to value 0"]
impl crate::Resettable for Spipf518Spec {}
