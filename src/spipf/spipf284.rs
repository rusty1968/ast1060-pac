#[doc = "Register `SPIPF284` reader"]
pub type R = crate::R<Spipf284Spec>;
#[doc = "Register `SPIPF284` writer"]
pub type W = crate::W<Spipf284Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf284::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf284::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf284Spec;
impl crate::RegisterSpec for Spipf284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf284::R`](R) reader structure"]
impl crate::Readable for Spipf284Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf284::W`](W) writer structure"]
impl crate::Writable for Spipf284Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF284 to value 0"]
impl crate::Resettable for Spipf284Spec {}
