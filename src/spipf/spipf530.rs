#[doc = "Register `SPIPF530` reader"]
pub type R = crate::R<Spipf530Spec>;
#[doc = "Register `SPIPF530` writer"]
pub type W = crate::W<Spipf530Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf530::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf530::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf530Spec;
impl crate::RegisterSpec for Spipf530Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf530::R`](R) reader structure"]
impl crate::Readable for Spipf530Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf530::W`](W) writer structure"]
impl crate::Writable for Spipf530Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF530 to value 0"]
impl crate::Resettable for Spipf530Spec {}
