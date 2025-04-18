#[doc = "Register `SPIPF664` reader"]
pub type R = crate::R<Spipf664Spec>;
#[doc = "Register `SPIPF664` writer"]
pub type W = crate::W<Spipf664Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf664::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf664::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf664Spec;
impl crate::RegisterSpec for Spipf664Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf664::R`](R) reader structure"]
impl crate::Readable for Spipf664Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf664::W`](W) writer structure"]
impl crate::Writable for Spipf664Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF664 to value 0"]
impl crate::Resettable for Spipf664Spec {}
