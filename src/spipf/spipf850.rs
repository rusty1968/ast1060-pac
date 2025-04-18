#[doc = "Register `SPIPF850` reader"]
pub type R = crate::R<Spipf850Spec>;
#[doc = "Register `SPIPF850` writer"]
pub type W = crate::W<Spipf850Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf850::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf850::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf850Spec;
impl crate::RegisterSpec for Spipf850Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf850::R`](R) reader structure"]
impl crate::Readable for Spipf850Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf850::W`](W) writer structure"]
impl crate::Writable for Spipf850Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF850 to value 0"]
impl crate::Resettable for Spipf850Spec {}
