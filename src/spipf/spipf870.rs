#[doc = "Register `SPIPF870` reader"]
pub type R = crate::R<Spipf870Spec>;
#[doc = "Register `SPIPF870` writer"]
pub type W = crate::W<Spipf870Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf870::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf870::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf870Spec;
impl crate::RegisterSpec for Spipf870Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf870::R`](R) reader structure"]
impl crate::Readable for Spipf870Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf870::W`](W) writer structure"]
impl crate::Writable for Spipf870Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF870 to value 0"]
impl crate::Resettable for Spipf870Spec {}
