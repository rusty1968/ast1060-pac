#[doc = "Register `SPIPF628` reader"]
pub type R = crate::R<Spipf628Spec>;
#[doc = "Register `SPIPF628` writer"]
pub type W = crate::W<Spipf628Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf628::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf628::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf628Spec;
impl crate::RegisterSpec for Spipf628Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf628::R`](R) reader structure"]
impl crate::Readable for Spipf628Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf628::W`](W) writer structure"]
impl crate::Writable for Spipf628Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF628 to value 0"]
impl crate::Resettable for Spipf628Spec {}
