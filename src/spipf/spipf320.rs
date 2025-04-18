#[doc = "Register `SPIPF320` reader"]
pub type R = crate::R<Spipf320Spec>;
#[doc = "Register `SPIPF320` writer"]
pub type W = crate::W<Spipf320Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf320::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf320::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf320Spec;
impl crate::RegisterSpec for Spipf320Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf320::R`](R) reader structure"]
impl crate::Readable for Spipf320Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf320::W`](W) writer structure"]
impl crate::Writable for Spipf320Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF320 to value 0"]
impl crate::Resettable for Spipf320Spec {}
