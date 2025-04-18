#[doc = "Register `SPIPF754` reader"]
pub type R = crate::R<Spipf754Spec>;
#[doc = "Register `SPIPF754` writer"]
pub type W = crate::W<Spipf754Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf754::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf754::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf754Spec;
impl crate::RegisterSpec for Spipf754Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf754::R`](R) reader structure"]
impl crate::Readable for Spipf754Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf754::W`](W) writer structure"]
impl crate::Writable for Spipf754Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF754 to value 0"]
impl crate::Resettable for Spipf754Spec {}
