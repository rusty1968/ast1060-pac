#[doc = "Register `SPIPF728` reader"]
pub type R = crate::R<Spipf728Spec>;
#[doc = "Register `SPIPF728` writer"]
pub type W = crate::W<Spipf728Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf728::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf728::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf728Spec;
impl crate::RegisterSpec for Spipf728Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf728::R`](R) reader structure"]
impl crate::Readable for Spipf728Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf728::W`](W) writer structure"]
impl crate::Writable for Spipf728Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF728 to value 0"]
impl crate::Resettable for Spipf728Spec {}
